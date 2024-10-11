use multiversx_sc_snippets_dapp::imports::Bech32Address;
use serde::{Deserialize, Serialize};

#[cfg(not(target_arch = "wasm32"))]
use actix_web::HttpResponse;

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

pub trait SetupRequests {}

#[derive(Serialize, Deserialize)]
pub struct SetupRequestBody<T: SetupRequests> {
    pub body: T,
}

impl SetupRequests for PaintHarvestDeploy {}
impl SetupRequests for InitialMoonSetup {}

#[derive(Serialize, Deserialize)]
pub struct DeployResponse {
    pub new_address: Bech32Address,
}

impl DeployResponse {
    pub fn new(new_address: Bech32Address) -> Self {
        Self { new_address }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn response(self) -> Bech32Address {
        self.new_address
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn response(&self) -> HttpResponse {
        HttpResponse::Ok().json(self)
    }
}
