use crate::{Config, Point};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq)]
pub struct Points(pub Vec<Point>);

pub trait QueryResponseTypes {}

#[derive(Deserialize, Serialize)]
pub struct QueryResponse<T: QueryResponseTypes> {
    pub response: T,
}

impl<T: QueryResponseTypes + Serialize> QueryResponse<T> {
    pub fn new(response: T) -> Self {
        Self { response }
    }
}

impl QueryResponseTypes for String {}
impl QueryResponseTypes for Points {}
impl QueryResponseTypes for Config {}
