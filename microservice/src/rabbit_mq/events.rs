use common::{Color, Coordinates, Points};
use redis::{AsyncCommands, RedisError};
use serde::{Deserialize, Serialize};

use crate::redis_local::Redis;

use super::MessageEvent;

#[derive(Serialize, Deserialize, Debug)]
pub struct Splash {
    coordinates: Coordinates,
    new_color: Color,
}

pub trait Event {
    fn from_message_event(event: &MessageEvent) -> Option<Self>
    where
        Self: Sized;
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
            // maybe here do some logic
            // if point is inside the map bounds and we are here
            // that means that the key is not yet available but the point is valid
            // reconstruct the map and create the redis key, or throw error if the point is out of bounds
            Err(err) => eprintln!("Failed to get points from Redis: {}", err),
        }
    }

    // TODO: see how to identify a Splash event
    fn from_message_event(_message_event: &MessageEvent) -> Option<Self> {
        None
    }
}
