extern crate ptm_benchmark_gas;

#[tokio::main]
pub async fn main() {
    ptm_benchmark_gas::adder_cli().await;
}
