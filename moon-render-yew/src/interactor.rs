use imports::{Address, Bech32Address, BytesValue};
use multiversx_sc_snippets_dapp::*;
use serde::{Deserialize, Serialize};

const GATEWAY: &str = sdk::core::gateway::DEVNET_GATEWAY;
const CONTRACT_ADDRESS: &str = "erd1qqqqqqqqqqqqqpgqf8snmxxg4tkq8fg7hl8uqamkgdwy29fga4sqjg2set";
const PAINT_THE_MOON_CODE: &[u8] = include_bytes!("../paint-the-moon-sc.wasm");

use multiversx_sc_snippets_dapp::imports::*;

use crate::requests::paint_the_moon_proxy::{self, Color, Point};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    gateway: String,
    contract_address: String,
}

impl Config {
    // Deserializes state from file
    pub fn new() -> Self {
        Config {
            gateway: GATEWAY.to_string(),
            contract_address: CONTRACT_ADDRESS.to_string(),
        }
    }

    /// Sets the contract address
    #[allow(unused)]
    pub fn set_address(&mut self, address: Bech32Address) {
        self.contract_address = address.to_string()
    }

    /// Returns the contract address
    pub fn current_address(&self) -> &String {
        &self.contract_address
    }
}

pub struct ContractInteract {
    pub interactor: DappInteractor,
    pub wallet_address: Address,
    pub contract_code: BytesValue,
    pub config: Config,
}

impl ContractInteract {
    pub async fn new() -> Self {
        let config = Config::new();
        let mut interactor = DappInteractor::new(&config.gateway, false).await;
        let wallet_address = interactor.register_wallet(test_wallets::mike()).await;

        let contract_code = BytesValue::from(PAINT_THE_MOON_CODE);

        ContractInteract {
            interactor,
            wallet_address,
            contract_code,
            config,
        }
    }

    pub async fn get_all_points(&mut self) -> Result<Vec<Point>, String> {
        let points = self
            .interactor
            .query()
            .to(Bech32Address::from_bech32_string(
                self.config.current_address().to_string(),
            ))
            .typed(paint_the_moon_proxy::PaintTheMoonScProxy)
            .get_all_points()
            .returns(ReturnsResult)
            .prepare_async()
            .run()
            .await;

        Ok(points.into_vec())
    }

    pub async fn deploy_paint_the_moon(&mut self) -> Result<Bech32Address, String> {
        let new_address = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .gas(60_000_000u64)
            .typed(paint_the_moon_proxy::PaintTheMoonScProxy)
            .init()
            .code(&self.contract_code)
            .code_metadata(CodeMetadata::UPGRADEABLE)
            .returns(ReturnsNewBech32Address)
            .prepare_async()
            .run()
            .await;

        Ok(new_address)
    }

    pub async fn paint(&mut self, point: u64, color: Color) -> Result<String, String> {
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(Bech32Address::from_bech32_string(
                self.config.current_address().to_string(),
            ))
            .gas(5_000_000u64)
            .typed(paint_the_moon_proxy::PaintTheMoonScProxy)
            .paint(point, color)
            .prepare_async()
            .run()
            .await;

        Ok("Painting successful".to_string())
    }

    pub async fn initial_moon_setup(
        &mut self,
        painted_points: Vec<Point>,
    ) -> Result<String, String> {
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(Bech32Address::from_bech32_string(
                self.config.current_address().to_string(),
            ))
            .gas(60_000_000u64)
            .typed(paint_the_moon_proxy::PaintTheMoonScProxy)
            .initial_map_setup(painted_points)
            .prepare_async()
            .run()
            .await;

        Ok("Initial moon setup successful".to_string())
    }
}
