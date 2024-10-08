use crate::interactor::ContractInteract;
use common::Point;

pub async fn get_all_points() -> Result<Vec<Point>, String> {
    let mut contract_interact = ContractInteract::new().await;

    contract_interact.get_all_points().await
}
