use crate::api::endpoints::Endpoint;
use crate::api::fetch;
use crate::api::models::{new_job, Job};
use futures::{future, Async, Future};
use serde_json::Error;
use wasm_bindgen::prelude::*;

pub fn post_job(
    name: String,
    desc: String,
    user: String,
    rate: String,
) -> Result<Async<JsValue>, JsValue> {
    let data = JsValue::from(new_job(name, desc, user, rate).unwrap());

    fetch::post(&Endpoint::Jobs, Some(&data))
}
