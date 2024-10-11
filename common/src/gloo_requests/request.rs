#[cfg(target_arch = "wasm32")]
use gloo_net::http::Request;
#[cfg(target_arch = "wasm32")]
use serde::de::DeserializeOwned;
#[cfg(target_arch = "wasm32")]
use web_sys::wasm_bindgen::JsValue;

#[cfg(target_arch = "wasm32")]
pub async fn get_request<T: DeserializeOwned>(dest: &str) -> Result<T, JsValue> {
    let response = Request::get(dest)
        .send()
        .await
        .map_err(|err| format!("Get {dest:?} request failed: {:?}", err))?;

    if response.ok() {
        let body = response
            .json::<T>()
            .await
            .map_err(|err| format!("Failed to read response body: {:?}", err))?;
        Ok(body)
    } else {
        Err(JsValue::from(format!(
            "Server error: {:?}",
            response.status()
        )))
    }
}

#[cfg(target_arch = "wasm32")]
pub async fn post_request<T: DeserializeOwned>(
    dest: &str,
    req_body: Option<JsValue>,
) -> Result<T, JsValue> {
    let request_builder = Request::post(dest)
        .body(req_body.unwrap_or_default())
        .unwrap();

    let response = request_builder
        .send()
        .await
        .map_err(|err| format!("Post {dest:#?} request failed: {:?}", err))?;

    if response.ok() {
        let body = response
            .json::<T>()
            .await
            .map_err(|err| format!("Failed to read response body: {:?}", err))?;
        Ok(body)
    } else {
        Err(JsValue::from(format!(
            "Server error: {:?}",
            response.status()
        )))
    }
}
