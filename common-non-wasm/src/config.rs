use std::{
    env,
    fs::{read_to_string, File},
    io::{BufWriter, Error, ErrorKind, Write},
    path::PathBuf,
};

use common::Config;

#[derive(Default)]
pub struct ConfigNonWasm(pub Config);

const STATE_FILE: &str = "common/src/config/state.toml";

impl ConfigNonWasm {
    pub fn inner(&self) -> &Config {
        &self.0
    }

    pub fn new() -> Self {
        match Self::resolve_state_path() {
            Ok(path) => match read_to_string(&path) {
                Ok(content) => {
                    let config = toml::from_str::<Config>(&content).unwrap_or_else(|err| {
                        eprintln!("Failed to parse TOML from {}: {:#?}", path.display(), err);
                        Config::default()
                    });
                    Self(config)
                }
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

impl Drop for ConfigNonWasm {
    fn drop(&mut self) {
        if let Ok(path) = Self::resolve_state_path() {
            match File::create(&path) {
                Ok(file) => {
                    let mut writer = BufWriter::new(file);
                    match toml::to_string(&self.0) {
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
