use common::Config;
use redis::{aio::MultiplexedConnection, Client};

#[derive(Clone)]
pub struct Redis {
    pub client: Client,
}

impl Redis {
    pub async fn new(config: &Config) -> Self {
        let client = Client::open(config.redis_url().to_string())
            .expect("Failed to connect to Redis server");

        Self { client }
    }

    pub async fn new_connection(&self) -> MultiplexedConnection {
        self.client
            .get_multiplexed_async_connection()
            .await
            .unwrap()
    }

    pub async fn flush_all(&mut self) {
        let mut con = self.new_connection().await;

        let _: () = redis::cmd("FLUSHALL")
            .query_async(&mut con)
            .await
            .expect("Failed to flush Redis");
    }
}
