use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use common::Config;
use rabbit_mq::{RabbitMq, Splash};
use redis_local::Redis;
use routes::{query_configuration, setup_configuration};

mod rabbit_mq;
mod redis_local;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::new();
    println!("{:#?}", config);

    let mut redis = Redis::new(&config).await;

    redis.flush_all().await;

    let rabbit_mq = RabbitMq::new().await;
    // declare a durable queue
    // will be reused if created
    rabbit_mq.declare_durable_queue("points").await;

    let redis_client_effect = redis.clone();

    actix_rt::spawn(async move {
        rabbit_mq
            .consume_rabbitmq_events::<Splash>(&redis_client_effect, "points")
            .await; // catch points updates based on events
    });

    // start the Actix server
    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .supports_credentials(),
            )
            .app_data(web::Data::new(redis.clone()))
            .service(web::scope("/setup").configure(setup_configuration))
            .service(web::scope("/query").configure(query_configuration))
    })
    .bind("127.0.0.1:8089")?
    .run()
    .await
}
