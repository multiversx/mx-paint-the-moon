use multiversx_sc_snippets_dapp::imports::Bech32Address;
use serde::{Deserialize, Serialize};

use super::Points;

#[derive(Serialize, Deserialize)]
pub struct InitialMoonSetup {
    pub painted_points: Points,
}

impl InitialMoonSetup {
    pub fn new(painted_points: Points) -> Self {
        Self { painted_points }
    }
}

#[derive(Serialize, Deserialize)]
pub struct PaintHarvestDeploy {
    pub collection_token_id: String,
    pub is_open: bool,
}

#[derive(Serialize, Deserialize)]
pub struct DeployResponse {
    pub new_address: Bech32Address,
}

impl DeployResponse {
    pub fn new(new_address: Bech32Address) -> Self {
        Self { new_address }
    }
}
