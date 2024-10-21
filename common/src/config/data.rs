use multiversx_sc_snippets_dapp::imports::*;
use serde::{Deserialize, Serialize};

#[cfg(not(target_arch = "wasm32"))]
use std::{
    env,
    fs::{read_to_string, File},
    io::{BufWriter, Error, ErrorKind, Write},
    path::PathBuf,
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

    // wasm target, get file content
    #[cfg(target_arch = "wasm32")]
    pub fn new() -> Self {
        toml::from_str(STATE_CONTENT).unwrap_or_default()
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn new() -> Self {
        match Self::resolve_state_path() {
            Ok(path) => match read_to_string(&path) {
                Ok(content) => toml::from_str(&content).unwrap_or_else(|err| {
                    eprintln!("Failed to parse TOML from {}: {:#?}", path.display(), err);
                    Self::default()
                }),
                Err(err) => {
                    eprintln!(
                        "Failed to read the state file at {}: {:#?}",
                        path.display(),
                        err
                    );
                    Self::default()
                }
            },
            Err(err) => {
                eprintln!("Error resolving state file path: {err:#?}");
                Self::default()
            }
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn resolve_state_path() -> Result<PathBuf, Error> {
        let mut current_dir = env::current_dir()?;

        loop {
            let cargo_toml_path = current_dir.join("Cargo.toml");

            if cargo_toml_path.exists() {
                let state_path = current_dir.join(STATE_FILE);
                if state_path.exists() {
                    return Ok(state_path);
                };
            }

            // stop at root
            if !current_dir.pop() {
                break;
            }
        }

        Err(Error::new(
            ErrorKind::NotFound,
            "Failed to find top-level project directory with Cargo.toml",
        ))
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Drop for Config {
    fn drop(&mut self) {
        if let Ok(path) = Self::resolve_state_path() {
            match File::create(&path) {
                Ok(file) => {
                    let mut writer = BufWriter::new(file);
                    match toml::to_string(&self) {
                        Ok(toml_content) => {
                            if let Err(err) = writer.write_all(toml_content.as_bytes()) {
                                eprintln!("Failed to write state file: {err:#?}");
                            }
                        }
                        Err(err) => eprintln!("Failed to serialize config: {:#?}", err),
                    }
                }
                Err(err) => eprintln!(
                    "Failed to create state file at {}: {:#?}",
                    path.display(),
                    err
                ),
            }
        }
    }
}
