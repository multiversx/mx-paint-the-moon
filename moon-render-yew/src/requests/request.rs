// use gloo_net::http::Request;

// requests to the microservice
// pub async fn get_request(name: &str, full_endpoint: &str) -> Result<String, String> {
//     let response = Request::get(full_endpoint)
//         .send()
//         .await
//         .map_err(|err| format!("{name} request failed: {:?}", err))?;

//     if response.ok() {
//         let body = response
//             .text()
//             .await
//             .map_err(|err| format!("Failed to read response body: {:?}", err))?;
//         Ok(body)
//     } else {
//         Err(format!("Server error: {:?}", response.status()))
//     }
// }

// pub async fn post_request(name: &str, full_endpoint: &str) -> Result<String, String> {
//     let response = Request::post(full_endpoint)
//         .send()
//         .await
//         .map_err(|err| format!("{name} request failed: {:?}", err))?;

//     if response.ok() {
//         let body = response
//             .text()
//             .await
//             .map_err(|err| format!("Failed to read response body: {:?}", err))?;
//         Ok(body)
//     } else {
//         Err(format!("Server error: {:?}", response.status()))
//     }
// }
