use crate::api::endpoints::{get_path, Endpoint};
use futures::{future, Async, Future};
use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

pub fn get(endpoint: &Endpoint) -> Result<Async<JsValue>, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");

    let request = Request::new_with_str_and_init(get_path(endpoint), &opts).unwrap();

    request.headers().set("Accept", "application/json").unwrap();

    let mut request_promise = fetch(request)
        .and_then(|resp_value| {
            // `resp_value` is a `Response` object.
            assert!(resp_value.is_instance_of::<Response>());
            let resp: Response = resp_value.dyn_into().unwrap();
            resp.json()
        })
        .and_then(|json_value: Promise| {
            // Convert this other `Promise` into a rust `Future`.
            JsFuture::from(json_value)
        })
        .and_then(|json| {
            // Use serde to parse the JSON into a struct.
            // let input: Result<Input> = json.into_serde().unwrap();
            // let output: JsValue = JsValue::from_serde(&input).unwrap();

            // Send the `Branch` struct back to JS as an `Object`.
            future::ok(json)
        });

    match request_promise.poll() {
        Ok(Async::Ready(result)) => Ok(Async::Ready(result)),
        Ok(Async::NotReady) => Ok(Async::NotReady),
        Err(e) => Err(e),
    }
}

pub fn post(endpoint: &Endpoint, data: Option<&JsValue>) -> Result<Async<JsValue>, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.body(data);

    let request = Request::new_with_str_and_init(get_path(endpoint), &opts).unwrap();

    request.headers().set("Accept", "application/json").unwrap();

    let mut request_promise = fetch(request)
        .and_then(|resp_value| {
            // `resp_value` is a `Response` object.
            assert!(resp_value.is_instance_of::<Response>());
            let resp: Response = resp_value.dyn_into().unwrap();
            resp.json()
        })
        .and_then(|json_value: Promise| {
            // Convert this other `Promise` into a rust `Future`.
            JsFuture::from(json_value)
        })
        .and_then(|json| {
            // Use serde to parse the JSON into a struct.
            // let input: Result<Input> = json.into_serde().unwrap();
            // let output: JsValue = JsValue::from_serde(&input).unwrap();

            // Send the `Branch` struct back to JS as an `Object`.
            future::ok(json)
        });

    match request_promise.poll() {
        Ok(Async::Ready(result)) => Ok(Async::Ready(result)),
        Ok(Async::NotReady) => Ok(Async::NotReady),
        Err(e) => Err(e),
    }
}

// #[wasm_bindgen]
pub fn fetch(request: Request) -> JsFuture {
    let window = web_sys::window().unwrap();
    let request_promise = window.fetch_with_request(&request);

    JsFuture::from(request_promise)
}
