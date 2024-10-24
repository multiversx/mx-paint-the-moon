mod basic_interact_cli;
mod basic_interact_config;
mod basic_interact_state;

use std::{
    collections::{BTreeMap, HashMap},
    fs::OpenOptions,
};

use crate::basic_interact_state::State;
pub use basic_interact_config::Config;
use basic_interact_state::ContractInfo;
use clap::Parser;
use multiversx_sc_snippets::imports::*;
use paint_the_moon_sc::paint_proxy;
use rand::Rng;
use std::io::Write;

const BLOCK_SIZES: &[usize] = &[4, 8, 16, 32];

pub async fn adder_cli() {
    env_logger::init();

    let config = Config::load_config();

    let mut basic_interact = AdderInteract::init(config).await;

    let cli = basic_interact_cli::InteractCli::parse();
    match &cli.command {
        Some(basic_interact_cli::InteractCliCommand::Deploy) => {
            basic_interact.multi_deploy().await;
        }
        Some(basic_interact_cli::InteractCliCommand::Sizes) => {
            basic_interact.print_sizes().await;
        }
        Some(basic_interact_cli::InteractCliCommand::Paint(_args)) => {
            basic_interact.paint_one().await;
        }
        Some(basic_interact_cli::InteractCliCommand::PaintAll) => {
            basic_interact.paint_all().await;
        }
        Some(basic_interact_cli::InteractCliCommand::PaintRectangles) => {
            basic_interact.paint_rectangles().await;
        }
        Some(basic_interact_cli::InteractCliCommand::PaintRand) => {
            basic_interact.paint_rand().await;
        }
        _ => {}
    }
}

#[allow(unused)]
pub struct AdderInteract {
    pub interactor: Interactor,
    pub owner_address: Bech32Address,
    pub state: State,
}

impl AdderInteract {
    pub async fn init(config: Config) -> Self {
        let mut interactor =
            Interactor::new(config.gateway_uri(), config.use_chain_simulator()).await;

        let owner_address = interactor
            .register_wallet(Wallet::from_pem_file("paint-owner.pem").unwrap())
            .await;

        // generate blocks until ESDTSystemSCAddress is enabled
        interactor.generate_blocks_until_epoch(1).await.unwrap();

        Self {
            interactor,
            owner_address: owner_address.into(),
            state: State::load_state(),
        }
    }

    pub async fn set_state(&mut self) {
        println!("wallet address: {}", self.owner_address);
        self.interactor.retrieve_account(&self.owner_address).await;
    }

    pub async fn multi_deploy(&mut self) {
        self.set_state().await;

        let mut buffer = self.interactor.homogenous_call_buffer();
        for block_size in BLOCK_SIZES {
            println!("block size {block_size:2} deploying ....");
            let code_path =
                format!("../paint-the-moon-sc/output/paint-the-moon-{block_size}.mxsc.json");

            buffer.push_tx(|tx| {
                tx.from(&self.owner_address)
                    .typed(paint_proxy::PaintTheMoonScProxy)
                    .init()
                    .code(MxscPath::new(&code_path))
                    .gas(10_000_000)
                    .returns(ReturnsNewBech32Address)
            });
        }

        let results: Vec<Bech32Address> = buffer.run().await;

        self.state.contract_info_list.clear();

        for (i, new_address) in results.into_iter().enumerate() {
            let block_size = BLOCK_SIZES[i];

            println!("block size {block_size:2} new address: {new_address}");

            self.state.contract_info_list.push(ContractInfo {
                block_size,
                address: new_address,
            });
        }
    }

    pub async fn print_sizes(&mut self) {
        for contract_info in &self.state.contract_info_list {
            let check_size = self
                .interactor
                .query()
                .to(&contract_info.address)
                .typed(paint_proxy::PaintTheMoonScProxy)
                .block_size()
                .returns(ReturnsResult)
                .run()
                .await;

            println!(
                "block size {:2} new address: {} check block size: {check_size}",
                contract_info.block_size, contract_info.address
            );
        }
    }

    pub async fn paint_one(&mut self) {
        self.set_state().await;

        let mut buffer = self.interactor.homogenous_call_buffer();
        for contract_info in &self.state.contract_info_list {
            let gas = match contract_info.block_size {
                64 => 25_000_000,
                32 => 7_000_000,
                _ => 5_000_000,
            };
            // let gas = 3_000_000;
            buffer.push_tx(|tx| {
                tx.from(&self.owner_address)
                    .to(&contract_info.address)
                    .typed(paint_proxy::PaintTheMoonScProxy)
                    .paint(100u32, 100u32, 2)
                    .gas(gas)
                    .returns(PassValue(contract_info.clone()))
                    .returns(ReturnsGasUsed)
            });
        }

        let result = buffer.run().await;
        for (contract_info, gas_used) in result {
            println!(
                "block size {:2} paint 1 gas used {:8}",
                contract_info.block_size, gas_used
            );
        }
    }

    pub async fn paint_all(&mut self) {
        self.set_state().await;

        let mut gas_report_raw = std::fs::File::create("gas-report-raw.csv").unwrap();
        let mut gas_report_group = std::fs::File::create("gas-report-group.csv").unwrap();

        const STEP: usize = 16;
        let max_x: usize = 1024;
        let max_y: usize = 512;

        let start_time = std::time::Instant::now();

        for x in 0..max_x {
            let mut current_y = 0;
            let mut next_y = 0;
            while next_y < max_y {
                next_y += STEP;

                let current_batch_time = std::time::Instant::now();

                print!("points:");
                let mut buffer = self.interactor.homogenous_call_buffer();
                for y in current_y..next_y {
                    for contract_info in self.state.contract_info_list.iter() {
                        buffer.push_tx(|tx| {
                            tx.from(&self.owner_address)
                                .to(&contract_info.address)
                                .typed(paint_proxy::PaintTheMoonScProxy)
                                .paint(x, y, 2)
                                .gas(8_000_000)
                                .returns(PassValue(x))
                                .returns(PassValue(y))
                                .returns(PassValue(contract_info.block_size))
                                .returns(ReturnsHandledOrError::new().returns(ReturnsGasUsed))
                        });
                    }
                    print!(" ({x}, {y})");
                }
                println!();

                let result = buffer.run().await;

                let mut buffer = HashMap::<String, BTreeMap<usize, u64>>::new();
                for (x, y, block_size, gas_result) in result {
                    let gas_used = gas_result.unwrap_or_default();
                    writeln!(gas_report_raw, "{x},{y},{block_size},{gas_used}",).unwrap();

                    let x_y_label = format!("\"({x}, {y})\"");
                    let line = buffer.entry(x_y_label.clone()).or_default();
                    line.insert(block_size, gas_used);
                    if line.len() == BLOCK_SIZES.len() {
                        write!(gas_report_group, "{x_y_label}").unwrap();
                        for block_size in BLOCK_SIZES {
                            write!(gas_report_group, ",{}", line[block_size]).unwrap();
                        }
                        writeln!(gas_report_group).unwrap();
                    }
                }

                println!("Elapsed from start: {:?}", start_time.elapsed());
                println!("Elapsed for batch:  {:?}", current_batch_time.elapsed());
                current_y = next_y;
            }
        }
    }

    pub async fn paint_rand(&mut self) {
        self.set_state().await;

        let mut rand_report_raw = OpenOptions::new()
            .write(true)
            .append(true)
            .open("rand-report-raw.csv")
            .unwrap();
        let mut rand_report_group = OpenOptions::new()
            .write(true)
            .append(true)
            .open("rand-report-group.csv")
            .unwrap();

        const STEP: usize = 16;
        let max_x: usize = 1024;
        let max_y: usize = 512;

        let start_time = std::time::Instant::now();
        let mut rng = rand::thread_rng();

        for i in 0..usize::MAX {
            let current_batch_time = std::time::Instant::now();

            let mut buffer = self.interactor.homogenous_call_buffer();
            for _ in 0..STEP {
                let x = rng.gen_range(0..max_x);
                let y = rng.gen_range(0..max_y);

                for contract_info in self.state.contract_info_list.iter() {
                    buffer.push_tx(|tx| {
                        tx.from(&self.owner_address)
                            .to(&contract_info.address)
                            .typed(paint_proxy::PaintTheMoonScProxy)
                            .paint(x, y, rng.gen_range(0..16u8))
                            .gas(8_000_000)
                            .returns(PassValue(x))
                            .returns(PassValue(y))
                            .returns(PassValue(contract_info.block_size))
                            .returns(ReturnsHandledOrError::new().returns(ReturnsGasUsed))
                    });
                }
            }

            let result = buffer.run().await;

            let mut buffer = HashMap::<String, BTreeMap<usize, u64>>::new();
            for (x, y, block_size, gas_result) in result {
                let gas_used = gas_result.unwrap_or_default();
                writeln!(rand_report_raw, "{x},{y},{block_size},{gas_used}",).unwrap();

                let x_y_label = format!("\"({x}, {y})\"");
                let line = buffer.entry(x_y_label.clone()).or_default();
                line.insert(block_size, gas_used);
                if line.len() == BLOCK_SIZES.len() {
                    write!(rand_report_group, "{x_y_label}").unwrap();
                    for block_size in BLOCK_SIZES {
                        write!(rand_report_group, ",{}", line[block_size]).unwrap();
                    }
                    writeln!(rand_report_group).unwrap();
                }
            }

            println!("#{i} Elapsed from start: {:?}", start_time.elapsed());
            println!(
                "#{i} Elapsed for batch:  {:?}",
                current_batch_time.elapsed()
            );
        }
    }

    pub async fn paint_rectangles(&mut self) {
        self.set_state().await;

        let mut rect_report_raw = std::fs::File::create("rect-report-raw.csv").unwrap();

        const STEP: usize = 32;
        let max_x: usize = 1024;
        let max_y: usize = 512;

        let start_time = std::time::Instant::now();

        let mut current_x = 0;
        let mut next_x = 0;
        while next_x < max_x {
            next_x += STEP;

            let mut current_y = 0;
            let mut next_y = 0;
            while next_y < max_y {
                next_y += STEP;

                let current_batch_time = std::time::Instant::now();

                print!("points:");
                let mut buffer = self.interactor.homogenous_call_buffer();
                for contract_info in self.state.contract_info_list.iter() {
                    buffer.push_tx(|tx| {
                        tx.from(&self.owner_address)
                            .to(&contract_info.address)
                            .typed(paint_proxy::PaintTheMoonScProxy)
                            .paint_rect(current_x, current_y, next_x, next_y, 5)
                            .gas(20_000_000)
                            .returns(PassValue(current_x))
                            .returns(PassValue(next_x))
                            .returns(PassValue(current_y))
                            .returns(PassValue(next_y))
                            .returns(PassValue(contract_info.block_size))
                            .returns(ReturnsHandledOrError::new().returns(ReturnsGasUsed))
                    });
                }
                print!(" ({current_x} .. {next_x}, {current_y} .. {next_y})");
                println!();

                let result = buffer.run().await;

                for (current_x, next_x, current_y, next_y, block_size, gas_result) in result {
                    let gas_used = gas_result.unwrap_or_default();

                    let x_y_label =
                        format!("\"({current_x}, {current_y}) -> ({next_x}, {next_y})\"");
                    writeln!(rect_report_raw, "{x_y_label},{block_size},{gas_used}",).unwrap();
                }

                println!("Elapsed from start: {:?}", start_time.elapsed());
                println!("Elapsed for batch:  {:?}", current_batch_time.elapsed());
                current_y = next_y;
            }

            current_x = next_x;
        }
    }
}
