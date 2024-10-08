use crate::interactor::ContractInteract;
use common::Point;
use multiversx_sc_snippets_dapp::imports::Bech32Address;

pub async fn deploy_paint_the_moon() -> Result<Bech32Address, String> {
    let mut contract_interact = ContractInteract::new().await;

    contract_interact.deploy_paint_the_moon().await
}

pub async fn initial_moon_setup(painted_points: Vec<Point>) -> Result<String, String> {
    let mut contract_interact = ContractInteract::new().await;

    contract_interact.initial_moon_setup(painted_points).await
}
