use serde::Deserialize;
use std::io::Read;

const CONFIG_FILE: &str = "microservice/src/rabbit_mq/rabbit_mq_config.toml";

#[derive(Deserialize)]
pub struct RabbitMqConfig {
    rabbit_mq_url: String,
}

impl RabbitMqConfig {
    pub fn new() -> Self {
        let mut file = std::fs::File::open(CONFIG_FILE).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        toml::from_str(&content).unwrap()
    }

    pub fn url(self) -> String {
        self.rabbit_mq_url
    }
}
