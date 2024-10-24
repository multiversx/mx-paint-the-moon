use multiversx_sc_snippets::sdk::data::sdk_address::SdkAddress;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub hash: String,
    pub shard_id: u64,
    pub timestamp: u64,
    #[serde(default)]
    pub events: Vec<MessageEvent>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageEvent {
    pub address: SdkAddress,
    pub identifier: String,
    #[serde(default)]
    pub topics: Option<Vec<String>>,
    #[serde(default)]
    pub data: Option<LogData>,
    pub tx_hash: String,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum LogData {
    #[default]
    Empty,
    String(String),
    Vec(Vec<String>),
}
