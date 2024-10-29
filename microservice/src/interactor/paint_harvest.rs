use crate::ContractInteract;
use common::PaintHarvestScProxy;
use multiversx_sc_snippets::{
    base::InteractorPrepareAsync,
    imports::{
        Bech32Address, BytesValue, CodeMetadata, ReturnsHandledOrError, ReturnsNewBech32Address,
        TokenIdentifier,
    },
    multiversx_sc_scenario::scenario_model::TxResponseStatus,
};

impl ContractInteract {
    pub async fn deploy_paint_harvest(
        &mut self,
        collection_token_id: String,
        is_open: bool,
    ) -> Result<Bech32Address, TxResponseStatus> {
        let paint_harvest_code = BytesValue::from(self.contract_code.paint_harvest);

        self.interactor
            .tx()
            .from(&self.wallet_address)
            .gas(60_000_000u64)
            .typed(PaintHarvestScProxy)
            .init(TokenIdentifier::from(&collection_token_id), is_open)
            .code(paint_harvest_code)
            .code_metadata(CodeMetadata::UPGRADEABLE)
            .returns(ReturnsHandledOrError::new().returns(ReturnsNewBech32Address))
            .prepare_async()
            .run()
            .await
    }
}
