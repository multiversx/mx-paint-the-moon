use common::{Color, Coordinates, Points};
use redis::{AsyncCommands, RedisError};
use serde::{Deserialize, Serialize};

use crate::redis_local::Redis;

#[derive(Serialize, Deserialize, Debug)]
pub struct Splash {
    coordinates: Coordinates,
    new_color: Color,
}

pub trait Event {
    fn handle_event(&self, redis_client: &Redis) -> impl std::future::Future<Output = ()> + Send;
}

impl Event for Splash {
    async fn handle_event(&self, redis_client: &Redis) {
        let mut con = redis_client.new_connection().await;
        let result: Result<Points, RedisError> = con.get("points").await;

        match result {
            Ok(mut points_cached_value) => {
                // find the point to update
                if let Some(point) = points_cached_value
                    .0
                    .iter_mut()
                    .find(|p| p.coordinates() == self.coordinates)
                {
                    // update the color
                    point.color = self.new_color;

                    // save the updated points back to Redis
                    let _: () = con.set("points", &points_cached_value).await.unwrap();
                } else {
                    eprintln!("Point not found for coordinates: {:?}", self.coordinates);
                }
            }
            Err(err) => eprintln!("Failed to get points from Redis: {}", err),
        }
    }
}
