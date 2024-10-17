use multiversx_sc_snippets_dapp::imports::*;
use serde::{Deserialize, Serialize};

#[cfg(not(target_arch = "wasm32"))]
use std::{
    io::{Read, Write},
    path::Path,
};

#[cfg(not(target_arch = "wasm32"))]
const STATE_FILE: &str = "common/src/config/state.toml";

#[cfg(target_arch = "wasm32")]
const STATE_CONTENT: &str = include_str!("state.toml");

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Clone)]
pub struct Config {
    gateway: String,
    microservice_url: String,
    redis_url: String,
    paint_the_moon_address: String,
    paint_harvest_address: String,
}

impl Config {
    // wasm target, get file content
    #[cfg(target_arch = "wasm32")]
    pub fn new() -> Self {
        toml::from_str(STATE_CONTENT).unwrap_or_default()
    }

    // for non wasm target
    // read file content
    #[cfg(not(target_arch = "wasm32"))]
    pub fn new() -> Self {
        if Path::new(STATE_FILE).exists() {
            let mut file = std::fs::File::open(STATE_FILE).unwrap();
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();
            toml::from_str(&content).unwrap()
        } else {
            Self::default()
        }
    }

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

// write back to TOML file in the non-WASM environment
#[cfg(not(target_arch = "wasm32"))]
impl Drop for Config {
    // serializes state to file
    fn drop(&mut self) {
        let mut file = std::fs::File::create(STATE_FILE).unwrap();
        file.write_all(toml::to_string(&self).unwrap().as_bytes())
            .unwrap();
    }
}
