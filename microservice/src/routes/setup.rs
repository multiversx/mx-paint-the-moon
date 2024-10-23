use actix_web::{post, Responder};
use actix_web::{web, HttpResponse};
use common::PaintHarvestDeploy;
use common_non_wasm::DeployResponseNonWasm;
use imports::MultiValueEncoded;
use interactor::ContractInteract;
use multiversx_sc_snippets::*;

#[post("/deploy_paint_the_moon")]
pub async fn deploy_paint_the_moon() -> impl Responder {
    let mut contract_interact = ContractInteract::new().await;

    //TODO: setup paint the moon req body if necessary
    let result = contract_interact
        .deploy_paint_the_moon(MultiValueEncoded::new())
        .await;

    match result {
        Ok(new_address) => {
            contract_interact
                .config
                .set_paint_the_moon_address(new_address.to_bech32_string());

            DeployResponseNonWasm::new(new_address).response()
        }
        Err(err) => HttpResponse::InternalServerError().body(format!(
            "Deploy Paint the Moon SC transaction failed with error: {:#?}",
            err.message
        )),
    }
}

// #[post("/initial_moon_setup")]
// pub async fn initial_moon_setup(
//     body: web::Json<InitialMoonSetup>,
//     redis_client: web::Data<Redis>,
// ) -> impl Responder {
//     let mut contract_interact = ContractInteract::new().await;
//     let mut con = redis_client.new_connection().await;

//     let points = body.0.painted_points;
//     let result = contract_interact.initial_moon_setup(points.0.clone()).await;

//     match result {
//         Ok(_) => {
//             let _: () = con.set("points", &points).await.unwrap();
//             QueryResponseNonWasm::new(points).response()
//         }
//         Err(err) => HttpResponse::InternalServerError().body(format!(
//             "Initial moon setup SC transaction failed with error: {:#?}.",
//             err.message
//         )),
//     }
// }

#[post("/deploy_paint_harvest")]
pub async fn deploy_paint_harvest(body: web::Json<PaintHarvestDeploy>) -> impl Responder {
    let mut contract_interact = ContractInteract::new().await;

    let params = body.0;
    let result = contract_interact
        .deploy_paint_harvest(params.collection_token_id, params.is_open)
        .await;

    match result {
        Ok(new_address) => {
            contract_interact
                .config
                .set_paint_harvest_address(new_address.to_bech32_string());

            DeployResponseNonWasm::new(new_address).response()
        }
        Err(err) => HttpResponse::InternalServerError().body(format!(
            "Deploy Paint Harvest SC transaction failed with error: {:#?}.",
            err.message
        )),
    }
}

pub fn setup_configuration(cfg: &mut web::ServiceConfig) {
    cfg.service(deploy_paint_the_moon)
        // .service(initial_moon_setup)
        .service(deploy_paint_harvest);
}
