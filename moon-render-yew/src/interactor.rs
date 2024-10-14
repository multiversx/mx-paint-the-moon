use common::{Color, Config, PaintTheMoonScProxy};

use multiversx_sc_snippets_dapp::imports::{test_wallets, Address, Bech32Address};
use multiversx_sc_snippets_dapp::{imports::*, DappInteractor};

pub struct ContractInteract {
    pub interactor: DappInteractor,
    pub wallet_address: Address,
    pub config: Config,
}

impl ContractInteract {
    pub async fn new() -> Self {
        let config = Config::new();
        let mut interactor = DappInteractor::new(config.gateway(), false).await;
        let wallet_address = interactor.register_wallet(test_wallets::mike()).await;

        ContractInteract {
            interactor,
            wallet_address,
            config,
        }
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
}