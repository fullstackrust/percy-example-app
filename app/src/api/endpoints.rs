use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub enum Endpoint {
    Api,
    Jobs,
}

pub fn get_path(endpoint: &Endpoint) -> &'static str {
    match endpoint {
        Endpoint::Api => "/api",
        Endpoint::Jobs => "/api/jobs",
    }
}

pub fn get_endpoint(path: &str) -> Endpoint {
    match path {
        "/api/jobs" => Endpoint::Api,
        &_ => Endpoint::Api,
    }
}
