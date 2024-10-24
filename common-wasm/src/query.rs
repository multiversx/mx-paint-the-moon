use common::{QueryResponse, QueryResponseTypes};
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
