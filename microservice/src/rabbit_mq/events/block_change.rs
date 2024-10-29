use common::Coordinates;
use serde::{Deserialize, Serialize};

use crate::rabbit_mq::MessageEvent;
use crate::redis_local::Redis;

use super::Event;

#[derive(Serialize, Deserialize, Debug)]
pub struct BlockChanged {
    coordinates: Coordinates,
    colors: Vec<u8>,
}

impl Event for BlockChanged {
    fn from_message_event(event: &MessageEvent) -> Option<Self>
    where
        Self: Sized,
    {
        todo!()
    }

    async fn handle_event(&self, redis_client: &Redis) {
        todo!()
    }
}
