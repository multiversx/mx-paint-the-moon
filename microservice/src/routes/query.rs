use actix_web::{get, web, Responder};
use base::InteractorPrepareAsync;
use common::{Config, PaintTheMoonScProxy, Points, QueryResponse};
use imports::{Bech32Address, ReturnsResultUnmanaged};
use interactor::ContractInteract;
use multiversx_sc_snippets::*;
use redis::{AsyncCommands, Client, RedisError};

#[get("/get_config")]
pub async fn get_config() -> impl Responder {
    QueryResponse::new(Config::new()).response()
}

#[get("/get_points")]
pub async fn get_points(redis_client: web::Data<Client>) -> impl Responder {
    let mut contract_interact = ContractInteract::new().await;
    let mut con = redis_client
        .get_multiplexed_async_connection()
        .await
        .unwrap();

    let points_cached_value: Result<Points, RedisError> = con.get("points").await;

    match points_cached_value {
        Ok(points) => QueryResponse::new(points).response(),
        Err(_) => {
            let current_address = contract_interact.config.paint_the_moon_address();

            let points_vec = contract_interact
                .interactor
                .query()
                .to(Bech32Address::from_bech32_string(
                    current_address.to_string(),
                ))
                .typed(PaintTheMoonScProxy)
                .get_all_points()
                .returns(ReturnsResultUnmanaged)
                .prepare_async()
                .run()
                .await;

            let points = Points(points_vec);
            let _: () = con.set("points", &points).await.unwrap();

            QueryResponse::new(points).response()
        }
    }
}

pub fn query_configuration(cfg: &mut web::ServiceConfig) {
    cfg.service(get_config).service(get_points);
}
