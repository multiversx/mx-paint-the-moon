use crate::redis_local::Redis;
use actix_web::{get, web, HttpResponse, Responder};
use common_non_wasm::{ConfigNonWasm, PointsNonWasm, QueryResponseNonWasm};
use interactor::ContractInteract;
use redis::{AsyncCommands, RedisError};

#[get("/get_config")]
pub async fn get_config() -> impl Responder {
    let config = ConfigNonWasm::new().inner().clone();
    QueryResponseNonWasm::new(config).response()
}

#[get("/get_points")]
pub async fn get_points(redis_client: web::Data<Redis>) -> impl Responder {
    let mut contract_interact = ContractInteract::new().await;
    let mut con = redis_client.new_connection().await;

    let points_cached_value: Result<PointsNonWasm, RedisError> = con.get("points").await;

    match points_cached_value {
        Ok(points) => QueryResponseNonWasm::new(points).response(),
        Err(_) => {
            // TODO: make a request that gets all state from the sc
            let result = Err("not yet");

            match result {
                Ok(points_vec) => {
                    let points = PointsNonWasm(points_vec);
                    let _: () = con.set("points", &points).await.unwrap();

                    QueryResponseNonWasm::new(points).response()
                }
                Err(err) => HttpResponse::InternalServerError().body(format!(
                    "Get points SC query failed with error message: {:#?}",
                    err
                )),
            }
        }
    }
}

pub fn query_configuration(cfg: &mut web::ServiceConfig) {
    cfg.service(get_config).service(get_points);
}
