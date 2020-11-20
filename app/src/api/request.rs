use crate::api::endpoints::Endpoint;
use crate::api::fetch;
use crate::api::models::new_job;
use wasm_bindgen::prelude::*;

pub async fn post_job(
    name: String,
    desc: String,
    user: String,
    rate: String,
) -> Result<JsValue, JsValue> {
    let data = JsValue::from(new_job(name, desc, user, rate).unwrap());

    fetch::post(&Endpoint::Jobs, Some(&data)).await;

    Ok(JsValue::NULL)
}
