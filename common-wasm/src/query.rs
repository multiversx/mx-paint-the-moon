use common::{DeployResponse, QueryResponse, QueryResponseTypes};
use multiversx_sc_snippets_dapp::imports::Bech32Address;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct QueryResponseWasm<T: QueryResponseTypes>(QueryResponse<T>);

impl<T: QueryResponseTypes + Serialize> QueryResponseWasm<T> {
    pub fn new(response: T) -> Self {
        Self(QueryResponse::new(response))
    }
    pub fn response(self) -> T {
        self.0.response
    }
}

pub struct DeployReponseWasm(DeployResponse);

impl DeployReponseWasm {
    pub fn new(new_address: Bech32Address) -> Self {
        Self(DeployResponse::new(new_address))
    }
    pub fn response(self) -> Bech32Address {
        self.0.new_address
    }
}
