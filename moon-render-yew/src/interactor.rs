use common::{Color, Config, PaintTheMoonScProxy, Point, CONTRACT_CODE};

use multiversx_sc_snippets_dapp::imports::{
    test_wallets, Address, Bech32Address, BytesValue, CodeMetadata, ReturnsNewBech32Address,
    ReturnsResult,
};
use multiversx_sc_snippets_dapp::{imports::*, DappInteractor};

pub struct ContractInteract {
    pub interactor: DappInteractor,
    pub wallet_address: Address,
    pub contract_code: BytesValue,
    pub config: Config,
}

impl ContractInteract {
    pub async fn new() -> Self {
        let config = Config::new();
        let mut interactor = DappInteractor::new(config.gateway(), false).await;
        let wallet_address = interactor.register_wallet(test_wallets::mike()).await;

        let contract_code = BytesValue::from(CONTRACT_CODE.paint_the_moon);

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
                self.config.paint_the_moon_address().to_string(),
            ))
            .typed(PaintTheMoonScProxy)
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
            .typed(PaintTheMoonScProxy)
            .init()
            .code(&self.contract_code)
            .code_metadata(CodeMetadata::UPGRADEABLE)
            .returns(ReturnsNewBech32Address)
            .prepare_async()
            .run()
            .await;

        // set config addr
        // for now, also a new address should be set in the file
        self.config
            .set_paint_the_moon_address(new_address.to_bech32_string());

        // send config to microsv, updates the local file
        // the entire logic will be moved to the microservice
        // let result = post_request::<Destination>(
        //     &self.config.microservice_url(), // add actual route
        //     Some(JsValue::from(format!("{:?}", self.config.dest()))),
        // )
        // .await;

        // match result {
        //     Ok(dest) => {
        //         log::info!("New state destination: {dest:#?}");
        //         self.config.set_dest(dest);
        //     }
        //     Err(err) => log::info!("Updating state destination request failed: {err:#?}"),
        // }

        Ok(new_address)
    }

    pub async fn paint(&mut self, point: u64, color: Color) -> Result<String, String> {
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(Bech32Address::from_bech32_string(
                self.config.paint_the_moon_address().to_string(),
            ))
            .gas(5_000_000u64)
            .typed(PaintTheMoonScProxy)
            .paint(point, color)
            .prepare_async()
            .run()
            .await;

        Ok("Painting successful".to_string())
    }

    pub async fn _initial_moon_setup(
        &mut self,
        painted_points: Vec<Point>,
    ) -> Result<String, String> {
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(Bech32Address::from_bech32_string(
                self.config.paint_the_moon_address().to_string(),
            ))
            .gas(60_000_000u64)
            .typed(PaintTheMoonScProxy)
            .initial_map_setup(painted_points)
            .prepare_async()
            .run()
            .await;

        Ok("Initial moon setup successful".to_string())
    }
}
