use multiversx_sc_snippets_dapp::{imports::*, sdk};
use serde::{Deserialize, Serialize};
use std::{
    io::{Read, Write},
    path::Path,
};

const GATEWAY: &str = sdk::core::gateway::DEVNET_GATEWAY;
const STATE_FILE: &str = "state.toml";
const _STATE_CONTENT: &str = include_str!("state.toml");

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Destination {
    pub microservice_url: String,
    pub paint_the_moon_address: String,
    pub paint_harvest_address: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    gateway: String,
    dest: Destination,
}

impl Config {
    // wasm target, get file content
    #[cfg(target_arch = "wasm32")]
    pub fn new() -> Self {
        Self {
            gateway: GATEWAY.to_string(),
            dest: toml::from_str(&_STATE_CONTENT).unwrap_or_default(),
        }
    }

    // for non wasm target
    // read file content
    #[cfg(not(target_arch = "wasm32"))]
    pub fn new() -> Self {
        if Path::new(STATE_FILE).exists() {
            let mut file = std::fs::File::open(STATE_FILE).unwrap();
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();
            Self {
                gateway: GATEWAY.to_string(),
                dest: toml::from_str(&content).unwrap(),
            }
        } else {
            Self::default()
        }
    }

    pub fn set_dest(&mut self, dest: Destination) {
        self.dest = dest;
    }

    pub fn set_microservice_url(&mut self, url: String) {
        self.dest.microservice_url = url;
    }

    pub fn set_paint_the_moon_address(&mut self, address: String) {
        self.dest.paint_the_moon_address = address;
    }

    pub fn set_paint_harvest_address(&mut self, address: String) {
        self.dest.paint_harvest_address = address;
    }

    pub fn dest(&self) -> &Destination {
        &self.dest
    }

    pub fn microservice_url(&self) -> &String {
        &self.dest.microservice_url
    }

    pub fn paint_the_moon_address(&self) -> &String {
        &self.dest.paint_the_moon_address
    }

    pub fn paint_harvest_address(&self) -> &String {
        &self.dest.paint_harvest_address
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
        file.write_all(toml::to_string(&self.dest).unwrap().as_bytes())
            .unwrap();
    }
}