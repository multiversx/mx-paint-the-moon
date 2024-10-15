#![allow(dead_code)]
use super::Event;
use crate::{redis_local::Redis, Config};
use futures::StreamExt;
use lapin::{
    options::{BasicAckOptions, BasicConsumeOptions, QueueDeclareOptions},
    types::FieldTable,
    Channel, Connection, ConnectionProperties,
};
use serde::de::DeserializeOwned;

pub struct RabbitMq {
    pub con: Connection,
}

impl RabbitMq {
    pub async fn new(config: &Config) -> Self {
        // take rabbitmq link from config
        let con = Connection::connect(config.rabbit_mq_url(), ConnectionProperties::default())
            .await
            .expect("Failed to connect to RabbitMQ");

        Self { con }
    }

    pub async fn new_channel(&self) -> Channel {
        self.con
            .create_channel()
            .await
            .expect("Failed to create channel")
    }

    pub async fn consume_rabbitmq_events<T: Event + DeserializeOwned>(
        &self,
        redis_client: &Redis,
        queue_name: &str,
    ) {
        let channel = self.new_channel().await;

        let _queue = channel
            .queue_declare(
                queue_name,
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await
            .expect("Failed to declare queue");

        let mut consumer = channel
            .basic_consume(
                queue_name,
                "points_consumer",
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await
            .expect("Failed to start the consumer");

        while let Some(delivery) = consumer.next().await {
            match delivery {
                Ok(delivery) => {
                    let json_str = String::from_utf8(delivery.data.clone())
                        .expect("Failed to convert delivery data to String, non utf8 compatible");

                    // deserialize the event from the owned string
                    let event: Result<T, _> = serde_json::from_str(&json_str);

                    match event {
                        Ok(event) => {
                            // handle the event
                            event.handle_event(redis_client).await;
                        }
                        Err(err) => {
                            eprintln!("Failed to deserialize event: {}", err);
                        }
                    }

                    // acknowledge the message
                    channel
                        .basic_ack(delivery.delivery_tag, BasicAckOptions::default())
                        .await
                        .expect("Failed to acknowledge message");
                }
                Err(err) => {
                    eprintln!("Error receiving message: {}", err);
                }
            }
        }
    }
}
