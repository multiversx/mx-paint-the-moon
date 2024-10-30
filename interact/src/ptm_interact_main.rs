extern crate ptm_interact;

#[tokio::main]
pub async fn main() {
    ptm_interact::main_cli().await;
}
