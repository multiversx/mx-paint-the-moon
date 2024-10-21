#![allow(dead_code)]

use common::{Color, Config, ContractCode, PaintTheMoonScProxy, Point, Points, CONTRACT_CODE};
use multiversx_sc_snippets::imports::*;
use serde::{Deserialize, Serialize};
use std::{
    io::{Read, Write},
    path::Path,
};

const STATE_FILE: &str = "../state.toml";

#[tokio::main]
async fn main() {
    env_logger::init();
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct State {
    paint_the_moon_address: Option<Bech32Address>,
}

impl State {
    pub fn load_state() -> Self {
        if Path::new(STATE_FILE).exists() {
            let mut file = std::fs::File::open(STATE_FILE).unwrap();
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();
            toml::from_str(&content).unwrap()
        } else {
            Self::default()
        }
    }

    pub fn set_paint_the_moon_address(&mut self, address: Bech32Address) {
        self.paint_the_moon_address = Some(address);
    }

    pub fn current_address(&self) -> &Bech32Address {
        self.paint_the_moon_address
            .as_ref()
            .expect("no known contract, deploy first")
    }
}

impl Drop for State {
    // Serializes state to file
    fn drop(&mut self) {
        let mut file = std::fs::File::create(STATE_FILE).unwrap();
        file.write_all(toml::to_string(self).unwrap().as_bytes())
            .unwrap();
    }
}

struct ContractInteract {
    interactor: Interactor,
    wallet_address: Address,
    contract_code: ContractCode,
    config: Config,
    state: State,
}

impl ContractInteract {
    async fn new() -> Self {
        let config = Config::new();
        let mut interactor = HttpInteractor::new(config.gateway(), false).await;
        let wallet_address = interactor.register_wallet(test_wallets::mike()).await;

        ContractInteract {
            interactor,
            wallet_address,
            contract_code: CONTRACT_CODE,
            config,
            state: State::load_state(),
        }
    }

    async fn deploy_paint_the_moon(&mut self) {
        let paint_the_moon_code = BytesValue::from(self.contract_code.paint_the_moon);

        let new_address = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .gas(60_000_000u64)
            .typed(PaintTheMoonScProxy)
            .init()
            .code(paint_the_moon_code)
            .code_metadata(CodeMetadata::UPGRADEABLE)
            .returns(ReturnsNewAddress)
            .prepare_async()
            .run()
            .await;

        let new_address_bech32 = bech32::encode(&new_address);
        self.state
            .set_paint_the_moon_address(Bech32Address::from_bech32_string(
                new_address_bech32.clone(),
            ));

        println!("new address: {new_address_bech32}");
    }

    async fn initial_moon_setup(&mut self, points: Points) {
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(60_000_000u64)
            .typed(PaintTheMoonScProxy)
            .initial_map_setup(points.0)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;
    }

    async fn get_all_points(&mut self) {
        let result = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(PaintTheMoonScProxy)
            .get_all_points()
            .returns(ReturnsResult)
            .prepare_async()
            .run()
            .await;
        println!("All points: {:?}", result);
    }
}

#[tokio::test]
async fn test_moon_max_size() {
    let mut interact = ContractInteract::new().await;
    let mut points = Vec::new();

    points.extend((0..500).flat_map(|x| {
        (0..500).map(move |y| Point {
            x,
            y,
            color: Color::Red,
        })
    }));
    interact.deploy_paint_the_moon().await;
    interact.initial_moon_setup(Points(points)).await;
    interact.get_all_points().await;
}
