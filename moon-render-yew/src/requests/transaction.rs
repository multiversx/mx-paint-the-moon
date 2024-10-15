use crate::interactor::ContractInteract;
use common::Point;

pub async fn paint(point: Point) -> Result<String, String> {
    let mut contract_interact = ContractInteract::new().await;

    contract_interact.paint(point).await
}
