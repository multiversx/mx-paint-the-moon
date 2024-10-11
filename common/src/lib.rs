mod codec;
mod config;
mod data;
mod gloo_requests;
mod proxies;
mod wasm;

pub use codec::{decode_coordinates, encode_coordinates};
pub use config::{Config, QueryRoutes, Routes, SetupRoutes};
pub use data::*;
pub use proxies::{PaintHarvestScProxy, PaintTheMoonScProxy};
pub use wasm::{ContractCode, CONTRACT_CODE};

#[cfg(target_arch = "wasm32")]
pub use gloo_requests::{get_request, post_request};
