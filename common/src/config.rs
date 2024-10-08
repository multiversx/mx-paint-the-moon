use multiversx_sc_snippets_dapp::{imports::*, sdk};
use serde::{Deserialize, Serialize};

const GATEWAY: &str = sdk::core::gateway::DEVNET_GATEWAY;
const CONTRACT_ADDRESS: &str = "erd1qqqqqqqqqqqqqpgqf8snmxxg4tkq8fg7hl8uqamkgdwy29fga4sqjg2set";

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    gateway: String,
    contract_address: String,
}

impl Config {
    // Deserializes state from file
    pub fn new() -> Self {
        Config {
            gateway: GATEWAY.to_string(),
            contract_address: CONTRACT_ADDRESS.to_string(),
        }
    }

    /// Sets the contract address
    #[allow(unused)]
    pub fn set_address(&mut self, address: Bech32Address) {
        self.contract_address = address.to_string()
    }

    /// Returns the contract address
    pub fn current_address(&self) -> &String {
        &self.contract_address
    }

    pub fn gateway(&self) -> &String {
        &self.gateway
    }
}
