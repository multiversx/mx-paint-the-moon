use crate::ContractInteract;
use common::{Color, PaintTheMoonScProxy, Point};
use multiversx_sc_snippets::{
    base::InteractorPrepareAsync, imports::*,
    multiversx_sc_scenario::scenario_model::TxResponseStatus,
};

impl ContractInteract {
    pub async fn get_points(&mut self) -> Result<Vec<Point>, TxResponseStatus> {
        let current_address = self.config.paint_the_moon_address();

        self.interactor
            .query()
            .to(Bech32Address::from_bech32_string(
                current_address.to_string(),
            ))
            .typed(PaintTheMoonScProxy)
            .get_all_points()
            .returns(ReturnsHandledOrError::new().returns(ReturnsResultUnmanaged))
            .run()
            .await
    }

    pub async fn paint(
        &mut self,
        point: Point,
        payment: EsdtTokenPayment<StaticApi>,
    ) -> Result<String, TxResponseStatus> {
        let result = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(Bech32Address::from_bech32_string(
                self.config.paint_the_moon_address().to_string(),
            ))
            .gas(5_000_000u64)
            .typed(PaintTheMoonScProxy)
            .paint(point)
            .payment(payment)
            .returns(ReturnsHandledOrError::new())
            .run()
            .await;

        match result {
            Ok(_) => Ok("Painting successful".to_string()),
            Err(err) => Err(err),
        }
    }

    pub async fn deploy_paint_the_moon(
        &mut self,
        setup: MultiValueEncoded<StaticApi, (TokenIdentifier<StaticApi>, Color)>,
    ) -> Result<Bech32Address, TxResponseStatus> {
        let paint_the_moon_code = BytesValue::from(self.contract_code.paint_the_moon);

        self.interactor
            .tx()
            .from(&self.wallet_address)
            .gas(60_000_000u64)
            .typed(PaintTheMoonScProxy)
            .init(setup)
            .code(paint_the_moon_code)
            .code_metadata(CodeMetadata::UPGRADEABLE)
            .returns(ReturnsHandledOrError::new().returns(ReturnsNewBech32Address))
            .run()
            .await
    }

    pub async fn initial_moon_setup(
        &mut self,
        points: Vec<Point>,
    ) -> Result<String, TxResponseStatus> {
        let result = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(Bech32Address::from_bech32_string(
                self.config.paint_the_moon_address().to_string(),
            ))
            .gas(30_000_000u64)
            .typed(PaintTheMoonScProxy)
            .initial_map_setup(points)
            .returns(ReturnsHandledOrError::new())
            .run()
            .await;

        match result {
            Ok(_) => Ok("Initial map setup successful".to_string()),
            Err(err) => Err(err),
        }
    }

    async fn upgrade(&mut self) -> Result<Bech32Address, TxResponseStatus> {
        let paint_the_moon_code = BytesValue::from(self.contract_code.paint_the_moon);

        self.interactor
            .tx()
            .to(Bech32Address::from_bech32_string(
                self.config.paint_the_moon_address().to_string(),
            ))
            .from(&self.wallet_address)
            .gas(30_000_000u64)
            .typed(PaintTheMoonScProxy)
            .upgrade()
            .code(paint_the_moon_code)
            .code_metadata(CodeMetadata::UPGRADEABLE)
            .returns(ReturnsHandledOrError::new().returns(ReturnsNewBech32Address))
            .run()
            .await
    }
}
