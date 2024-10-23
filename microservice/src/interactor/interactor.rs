use common::{Config, ContractCode, CONTRACT_CODE};
use common_non_wasm::ConfigNonWasm;
use imports::Address;
use multiversx_sc_snippets::*;

pub struct ContractInteract {
    pub interactor: Interactor,
    pub wallet_address: Address,
    pub contract_code: ContractCode,
    pub config: Config,
}

impl ContractInteract {
    pub async fn new() -> Self {
        let config = ConfigNonWasm::new().inner().clone();
        let mut interactor = HttpInteractor::new(config.gateway(), false).await;
        let wallet_address = interactor.register_wallet(test_wallets::mike()).await;

        ContractInteract {
            interactor,
            wallet_address,
            contract_code: CONTRACT_CODE,
            config,
        }
    }
}
