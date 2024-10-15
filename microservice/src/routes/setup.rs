use crate::redis_local::Redis;
use actix_web::{post, Responder};
use actix_web::{web, HttpResponse};
use base::InteractorPrepareAsync;
use common::{
    DeployResponse, InitialMoonSetup, PaintHarvestDeploy, PaintHarvestScProxy, PaintTheMoonScProxy,
    QueryResponse,
};
use imports::{
    Bech32Address, BytesValue, CodeMetadata, ReturnsHandledOrError, ReturnsNewBech32Address,
    ReturnsTxHash, TokenIdentifier,
};
use interactor::ContractInteract;
use multiversx_sc_snippets::*;
use redis::AsyncCommands;

#[post("/deploy_paint_the_moon")]
pub async fn deploy_paint_the_moon() -> impl Responder {
    let mut contract_interact = ContractInteract::new().await;

    let paint_the_moon_code = BytesValue::from(contract_interact.contract_code.paint_the_moon);

    let (result, tx_hash) = contract_interact
        .interactor
        .tx()
        .from(&contract_interact.wallet_address)
        .gas(60_000_000u64)
        .typed(PaintTheMoonScProxy)
        .init()
        .code(paint_the_moon_code)
        .code_metadata(CodeMetadata::UPGRADEABLE)
        .returns(ReturnsHandledOrError::new().returns(ReturnsNewBech32Address))
        .returns(ReturnsTxHash)
        .prepare_async()
        .run()
        .await;

    match result {
        Ok(new_address) => {
            contract_interact
                .config
                .set_paint_the_moon_address(new_address.to_bech32_string());

            DeployResponse::new(new_address).response()
        }
        Err(err) => HttpResponse::InternalServerError().body(format!(
            "Deploy Paint the Moon SC transaction failed with error: {:#?}. Tx hash: {:#?}",
            err.message, tx_hash
        )),
    }
}

#[post("/initial_moon_setup")]
pub async fn initial_moon_setup(
    body: web::Json<InitialMoonSetup>,
    redis_client: web::Data<Redis>,
) -> impl Responder {
    let mut contract_interact = ContractInteract::new().await;
    let mut con = redis_client.new_connection().await;

    let points = body.0.painted_points;

    let (result, tx_hash) = contract_interact
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
        .returns(ReturnsHandledOrError::new())
        .returns(ReturnsTxHash)
        .prepare_async()
        .run()
        .await;

    match result {
        Ok(_) => {
            let _: () = con.set("points", &points).await.unwrap();
            QueryResponse::new(points).response()
        }
        Err(err) => HttpResponse::InternalServerError().body(format!(
            "Initial moon setup SC transaction failed with error: {:#?}. Tx hash: {:#?}",
            err.message, tx_hash
        )),
    }
}

#[post("/deploy_paint_harvest")]
pub async fn deploy_paint_harvest(body: web::Json<PaintHarvestDeploy>) -> impl Responder {
    let mut contract_interact = ContractInteract::new().await;

    let paint_harvest_code = BytesValue::from(contract_interact.contract_code.paint_harvest);
    let params = body.0;

    let (result, tx_hash) = contract_interact
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
        .returns(ReturnsHandledOrError::new().returns(ReturnsNewBech32Address))
        .returns(ReturnsTxHash)
        .prepare_async()
        .run()
        .await;

    match result {
        Ok(new_address) => {
            contract_interact
                .config
                .set_paint_harvest_address(new_address.to_bech32_string());

            DeployResponse::new(new_address).response()
        }
        Err(err) => HttpResponse::InternalServerError().body(format!(
            "Deploy Paint Harvest SC transaction failed with error: {:#?}. Tx hash: {:#?}",
            err.message, tx_hash
        )),
    }
}

pub fn setup_configuration(cfg: &mut web::ServiceConfig) {
    cfg.service(deploy_paint_the_moon)
        .service(initial_moon_setup)
        .service(deploy_paint_harvest);
}
