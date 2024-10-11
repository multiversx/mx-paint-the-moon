mod request;

#[cfg(target_arch = "wasm32")]
pub use request::{get_request, post_request};
