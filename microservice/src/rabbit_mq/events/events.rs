use crate::rabbit_mq::MessageEvent;
use crate::Redis;

pub trait Event {
    fn from_message_event(event: &MessageEvent) -> Option<Self>
    where
        Self: Sized;
    async fn handle_event(&self, redis_client: &Redis);
}
