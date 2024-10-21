use crate::redis_local::Redis;
use actix_web::{get, web, HttpResponse, Responder};
use common::{Config, Points, QueryResponse};
use interactor::ContractInteract;
use redis::{AsyncCommands, RedisError};

#[get("/get_config")]
pub async fn get_config() -> impl Responder {
    QueryResponse::new(Config::new()).response()
}

#[get("/get_points")]
pub async fn get_points(redis_client: web::Data<Redis>) -> impl Responder {
    let mut contract_interact = ContractInteract::new().await;
    let mut con = redis_client.new_connection().await;

    let points_cached_value: Result<Points, RedisError> = con.get("points").await;

    match points_cached_value {
        Ok(points) => QueryResponse::new(points).response(),
        Err(_) => {
            let result = contract_interact.get_points().await;

            match result {
                Ok(points_vec) => {
                    let points = Points(points_vec);
                    let _: () = con.set("points", &points).await.unwrap();

                    QueryResponse::new(points).response()
                }
                Err(err) => HttpResponse::InternalServerError().body(format!(
                    "Get points SC query failed with error message: {:#?}",
                    err.message
                )),
            }
        }
    }
}

pub fn query_configuration(cfg: &mut web::ServiceConfig) {
    cfg.service(get_config).service(get_points);
}
