use common::{Color, Config, Coordinates, Point, Points, MAX_HEIGHT, MAX_WIDTH};
use multiversx_sc_snippets::imports::NestedDecode;
use multiversx_sdk::utils::base64_decode;
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
    async fn handle_event(&self, redis_client: &Redis);
}

impl Event for Splash {
    async fn handle_event(&self, redis_client: &Redis) {
        let mut con = redis_client.new_connection().await;
        let result: Result<Points, RedisError> = con.get("points").await;

        match result {
            Ok(mut points_cached_value) => {
                println!("Points fetched from Redis: {:?}", points_cached_value);
                // find the point to update
                if let Some(point) = points_cached_value
                    .0
                    .iter_mut()
                    .find(|p| p.coordinates() == self.coordinates)
                {
                    // update the color
                    println!("Point found, updating color to {:?}", self.new_color);
                    point.color = self.new_color;

                    // save the updated points back to Redis
                    let _: () = con.set("points", &points_cached_value).await.unwrap();
                    println!("Points updated in Redis.");
                } else {
                    println!("Point not found for coordinates: {:?}", self.coordinates);
                }
            }
            Err(err) => {
                println!("Failed to get points from Redis: {err:?}.");

                if self.coordinates.0 <= MAX_WIDTH && self.coordinates.1 <= MAX_HEIGHT {
                    println!("Point in bounds. Saving new key {:?}..", &self.coordinates);

                    //TODO: reconstruct entire map
                    let mut points = Vec::new();
                    points.push(Point {
                        x: self.coordinates.0,
                        y: self.coordinates.1,
                        color: self.new_color,
                    });
                    let _: () = con.set("points", &Points(points)).await.unwrap();
                    println!("New points saved in Redis.");
                }
            }
        }
    }

    fn from_message_event(message_event: &MessageEvent) -> Option<Self> {
        let config = Config::new();

        if let Some(topics) = &message_event.topics {
            if message_event.address.to_bech32_string().unwrap() == *config.paint_the_moon_address()
                && message_event.identifier == "paint"
            {
                println!("Paint event found");
                let coord_hex = String::from_utf8(base64_decode(topics[1].clone())).unwrap();
                let new_color_hex = String::from_utf8(base64_decode(topics[2].clone())).unwrap();

                let coordinates = Coordinates::dep_decode(&mut coord_hex.as_bytes());
                let new_color = Color::dep_decode(&mut new_color_hex.as_bytes());

                println!("Decoded Coordinates: {:?}", coordinates);
                println!("Decoded Color: {:?}", new_color);

                if let (Ok(coordinates), Ok(new_color)) = (coordinates, new_color) {
                    return Some(Self {
                        coordinates,
                        new_color,
                    });
                } else {
                    println!("Failed to decode coordinates or color");
                }
            }
        }

        None
    }
}
