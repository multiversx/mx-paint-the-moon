use actix_web::web;
use actix_web::{post, Responder};
use base::InteractorPrepareAsync;
use common::{
    DeployResponse, InitialMoonSetup, PaintHarvestDeploy, PaintHarvestScProxy, PaintTheMoonScProxy,
    QueryResponse,
};
use imports::{Bech32Address, BytesValue, CodeMetadata, ReturnsNewBech32Address, TokenIdentifier};
use interactor::ContractInteract;

use multiversx_sc_snippets::*;
use redis::{AsyncCommands, Client};

#[post("/deploy_paint_the_moon")]
pub async fn deploy_paint_the_moon() -> impl Responder {
    let mut contract_interact = ContractInteract::new().await;

    let paint_the_moon_code = BytesValue::from(contract_interact.contract_code.paint_the_moon);

    let new_address = contract_interact
        .interactor
        .tx()
        .from(&contract_interact.wallet_address)
        .gas(60_000_000u64)
        .typed(PaintTheMoonScProxy)
        .init()
        .code(paint_the_moon_code)
        .code_metadata(CodeMetadata::UPGRADEABLE)
        .returns(ReturnsNewBech32Address)
        .prepare_async()
        .run()
        .await;

    contract_interact
        .config
        .set_paint_the_moon_address(new_address.to_bech32_string());

    DeployResponse::new(new_address).response()
}

#[post("/initial_moon_setup")]
pub async fn initial_moon_setup(
    body: web::Json<InitialMoonSetup>,
    redis_client: web::Data<Client>,
) -> impl Responder {
    let mut contract_interact = ContractInteract::new().await;
    let mut con = redis_client
        .get_multiplexed_async_connection()
        .await
        .unwrap();

    let points = body.0.painted_points;

    contract_interact
        .interactor
        .tx()
        .from(&contract_interact.wallet_address)
        .to(Bech32Address::from_bech32_string(
            contract_interact
                .config
                .paint_the_moon_address()
                .to_string(),
        ))
        .gas(60_000_000u64)
        .typed(PaintTheMoonScProxy)
        .initial_map_setup(points.0.clone())
        .prepare_async()
        .run()
        .await;

    let _: () = con.set("points", &points).await.unwrap();
    QueryResponse::new(points).response()
}

#[post("/deploy_paint_harvest")]
pub async fn deploy_paint_harvest(body: web::Json<PaintHarvestDeploy>) -> impl Responder {
    let mut contract_interact = ContractInteract::new().await;

    let paint_harvest_code = BytesValue::from(contract_interact.contract_code.paint_harvest);
    let params = body.0;

    let new_address = contract_interact
        .interactor
        .tx()
        .from(&contract_interact.wallet_address)
        .gas(60_000_000u64)
        .typed(PaintHarvestScProxy)
        .init(
            TokenIdentifier::from(&params.collection_token_id),
            params.is_open,
        )
        .code(paint_harvest_code)
        .code_metadata(CodeMetadata::UPGRADEABLE)
        .returns(ReturnsNewBech32Address)
        .prepare_async()
        .run()
        .await;

    contract_interact
        .config
        .set_paint_harvest_address(new_address.to_bech32_string());

    DeployResponse::new(new_address).response()
}

pub fn setup_configuration(cfg: &mut web::ServiceConfig) {
    cfg.service(deploy_paint_the_moon)
        .service(initial_moon_setup)
        .service(deploy_paint_harvest);
}
