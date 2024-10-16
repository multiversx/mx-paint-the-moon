#![allow(dead_code)]
use super::{Event, RabbitMqConfig};
use crate::{rabbit_mq::Message, redis_local::Redis};
use futures::StreamExt;
use lapin::{
    options::{BasicAckOptions, BasicConsumeOptions, QueueDeclareOptions},
    types::FieldTable,
    Channel, Connection, ConnectionProperties, Consumer,
};
use serde::de::DeserializeOwned;

pub struct RabbitMq {
    pub con: Connection,
}

pub async fn new_consumer(channel: &Channel, queue_name: &str) -> Consumer {
    channel
        .basic_consume(
            queue_name,
            &format!("{queue_name:#?}_consumer"),
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await
        .unwrap_or_else(|_| panic!("Failed to start the {queue_name:#?} consumer"))
}

impl RabbitMq {
    // new RabbitMQ connection
    pub async fn new() -> Self {
        let config = RabbitMqConfig::new();
        let con = Connection::connect(&config.url(), ConnectionProperties::default())
            .await
            .expect("Failed to connect to RabbitMQ");

        Self { con }
    }

    // create a new channel (used per operation)
    pub async fn new_channel(&self) -> Channel {
        self.con
            .create_channel()
            .await
            .expect("Failed to create channel")
    }

    // declare a durable queue (gets created once, uses previously created instance afterwards)
    pub async fn declare_durable_queue(&self, queue_name: &str) {
        let channel = self.new_channel().await;
        channel
            .queue_declare(
                queue_name,
                QueueDeclareOptions {
                    durable: true,
                    ..Default::default()
                },
                FieldTable::default(),
            )
            .await
            .expect("Failed to declare durable queue");
    }

    pub async fn consume_rabbitmq_events<T: Event + DeserializeOwned>(
        &self,
        redis_client: &Redis,
        queue_name: &str,
    ) {
        // new channel per consumer
        let channel = self.new_channel().await;
        let mut consumer = new_consumer(&channel, queue_name).await;

        // process each message received from the queue
        while let Some(delivery) = consumer.next().await {
            match delivery {
                Ok(delivery) => {
                    let json_str = String::from_utf8(delivery.data)
                        .expect("Failed to convert delivery data to String, not utf8 compatible");

                    let message = serde_json::from_str::<Message>(&json_str)
                        .expect("Failed to convert str into Message");
                    println!("message is {message:#?}");

                    // check if there's any T event
                    for message_event in message.events {
                        // custom deserialize the event to type T
                        if let Some(event) = T::from_message_event(&message_event) {
                            event.handle_event(redis_client).await;
                        }
                    }

                    // acknowledge the message
                    channel
                        .basic_ack(delivery.delivery_tag, BasicAckOptions::default())
                        .await
                        .unwrap_or_else(|_| {
                            panic!("Failed to acknowledge message on queue {queue_name:#?}")
                        });
                }
                Err(err) => {
                    eprintln!("Error receiving message on queue {queue_name:#?}: {err:#?}");
                }
            }
        }
    }
}
