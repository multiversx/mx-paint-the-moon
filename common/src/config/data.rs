use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Clone)]
pub struct Config {
    gateway: String,
    microservice_url: String,
    redis_url: String,
    paint_the_moon_address: String,
    paint_harvest_address: String,
}

impl Config {
    pub fn set_redis_url(&mut self, redis_url: String) {
        self.redis_url = redis_url;
    }

    pub fn set_microservice_url(&mut self, url: String) {
        self.microservice_url = url;
    }

    pub fn set_paint_the_moon_address(&mut self, address: String) {
        self.paint_the_moon_address = address;
    }

    pub fn set_paint_harvest_address(&mut self, address: String) {
        self.paint_harvest_address = address;
    }

    pub fn redis_url(&self) -> &String {
        &self.redis_url
    }

    pub fn microservice_url(&self) -> &String {
        &self.microservice_url
    }

    pub fn paint_the_moon_address(&self) -> &String {
        &self.paint_the_moon_address
    }

    pub fn paint_harvest_address(&self) -> &String {
        &self.paint_harvest_address
    }

    pub fn gateway(&self) -> &String {
        &self.gateway
    }
}
