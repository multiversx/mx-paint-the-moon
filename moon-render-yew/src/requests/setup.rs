use super::post_request;
use common::{
    Config, DeployResponse, InitialMoonSetup, Points, QueryResponse, Routes, SetupRoutes,
};
use multiversx_sc_snippets_dapp::imports::Bech32Address;

pub async fn deploy_paint_the_moon() -> Result<Bech32Address, String> {
    let config = Config::new();
    let dest = format!(
        "{}{}",
        config.microservice_url(),
        Routes::Setup(SetupRoutes::DeployPaintTheMoon).as_str()
    );

    let response = post_request::<DeployResponse>(&dest, None).await;

    match response {
        Ok(deploy_response) => Ok(deploy_response.new_address),
        Err(err) => Err(format!("Deploy paint the moon failed with error: {err:?}")),
    }
}

pub async fn _initial_moon_setup(painted_points: Points) -> Result<Points, String> {
    let config = Config::new();
    let dest = format!(
        "{}{}",
        config.microservice_url(),
        Routes::Setup(SetupRoutes::InitialMoonSetup).as_str()
    );
    let deploy_body = InitialMoonSetup::new(painted_points);
    let formatted_body = serde_wasm_bindgen::to_value(&deploy_body).unwrap();

    let response = post_request::<QueryResponse<Points>>(&dest, Some(formatted_body)).await;

    match response {
        Ok(query_response) => Ok(query_response.response()),
        Err(err) => Err(format!("Deploy paint the moon failed with error: {err:?}")),
    }
}
