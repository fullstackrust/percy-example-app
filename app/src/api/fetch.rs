use crate::api::endpoints::{get_path, Endpoint};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, Response};

pub async fn get(endpoint: &Endpoint) -> Result<JsValue, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");

    let request = Request::new_with_str_and_init(get_path(endpoint), &opts).unwrap();

    request.headers().set("Accept", "application/json").unwrap();

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into().unwrap();
    let json = JsFuture::from(resp.json()?).await?;

    Ok(json)
}

pub async fn post(endpoint: &Endpoint, data: Option<&JsValue>) -> Result<JsValue, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.body(data);

    let request = Request::new_with_str_and_init(get_path(endpoint), &opts).unwrap();

    request.headers().set("Accept", "application/json").unwrap();
    request
        .headers()
        .set("Content-Type", "application/json")
        .unwrap();

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into().unwrap();
    let json = JsFuture::from(resp.json()?).await?;

    Ok(json)
}

// #[wasm_bindgen]
// pub fn fetch(request: Request) -> JsFuture {
//     let window = web_sys::window().unwrap();
//     let request_promise = window.fetch_with_request(&request);

//     JsFuture::from(request_promise)
// }
