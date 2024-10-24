use common::Config;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct ConfigWasm(pub Config);

const STATE_CONTENT: &str = include_str!("../../common/src/config/state.toml");

impl ConfigWasm {
    pub fn new() -> Self {
        ConfigWasm(toml::from_str::<Config>(STATE_CONTENT).unwrap_or_default())
    }

    pub fn inner(&self) -> &Config {
        &self.0
    }
}
