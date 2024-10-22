mod basic_interact_cli;
mod basic_interact_config;
mod basic_interact_state;

use crate::basic_interact_state::State;
pub use basic_interact_config::Config;
use basic_interact_state::ContractInfo;
use clap::Parser;
use paint_the_moon_sc::paint_proxy;

use multiversx_sc_snippets::imports::*;

const INTERACTOR_SCENARIO_TRACE_PATH: &str = "interactor_trace.scen.json";

const BLOCK_SIZES: &[usize] = &[4, 8, 16, 32, 64];

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
            basic_interact.paint().await;
        }
        // Some(basic_interact_cli::InteractCliCommand::Sum) => {
        //     let sum = basic_interact.get_sum().await;
        //     println!("sum: {sum}");
        // },
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
        let mut interactor = Interactor::new(config.gateway_uri(), config.use_chain_simulator())
            .await
            .with_tracer(INTERACTOR_SCENARIO_TRACE_PATH)
            .await;

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
                    .gas(8_000_000)
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

    pub async fn paint(&mut self) {
        self.set_state().await;

        let mut buffer = self.interactor.homogenous_call_buffer();
        for contract_info in &self.state.contract_info_list {
            let gas = match contract_info.block_size {
                64 => 10_000_000,
                32 => 10_000_000,
                _ => 10_000_000,
            };
            buffer.push_tx(|tx| {
                tx.from(&self.owner_address)
                    .to(&contract_info.address)
                    .typed(paint_proxy::PaintTheMoonScProxy)
                    .paint(512u32, 256u32, 1)
                    .gas(gas)
                    .returns(ReturnsGasUsed)
            });
        }

        let gas_used_list = buffer.run().await;
        for (i, contract_info) in self.state.contract_info_list.iter().enumerate() {
            println!(
                "block size {:2} paint 1 gas used {:8}",
                contract_info.block_size, gas_used_list[i]
            );
        }
    }

    // pub async fn add(&mut self, value: u32) {
    //     self.interactor
    //         .tx()
    //         .from(&self.wallet_address)
    //         .to(self.state.current_adder_address())
    //         .gas(6_000_000)
    //         .typed(adder_proxy::AdderProxy)
    //         .add(value)
    //         .run()
    //         .await;

    //     println!("successfully performed add");
    // }
}
