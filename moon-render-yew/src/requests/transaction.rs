use super::paint_the_moon_proxy::Color;
use crate::interactor::ContractInteract;

pub async fn paint(point: u64, color: Color) -> Result<String, String> {
    let mut contract_interact = ContractInteract::new().await;

    contract_interact.paint(point, color).await
}
