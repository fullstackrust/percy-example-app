use crate::api::schema;
// use crate::api::{endpoints, fetch, models};
use crate::store::Store;
// use futures::prelude::{r#await, *};
// use graphql_client::graphql_query_derive::*;
use futures::Future;
use graphql_client_web::{Client, ClientError, GraphQLQuery, Response};
use std::cell::{Ref, RefCell};
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "../graphql/job_schema.graphql",
    query_path = "../graphql/job_query.graphql"
)]
struct JobQuery;

pub fn get_field(store: Ref<Store>, name: &str) -> String {
    store.form().get(name).unwrap_or(&"".to_string()).to_owned()
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn job_alert(job_name: String, job_desc: String, job_user: String, job_rate: String) {
    alert(&format!(
        "Job name: {}\nJob description: {}\nJob user: {}\nJob rate: {}",
        job_name, job_desc, job_user, job_rate
    ));
}

// Result<Response<job_query::ResponseData>, ClientError>
// #[r#async]
pub fn post_job(store: Rc<RefCell<Store>>) -> impl Future<Item = (), Error = JsValue> {
    let name = get_field(store.borrow(), "job_name");
    let desc = get_field(store.borrow(), "job_desc");
    let user = get_field(store.borrow(), "job_user");
    let rate = get_field(store.borrow(), "job_rate");

    // request::post_job(job_name, job_desc, job_user, job_rate).unwrap();
    // job_alert(job_name, job_desc, job_user, job_rate);
    // schema::create_job();

    let variables = job_query::Variables {
        name,
        desc,
        user,
        rate,
    };

    // #[cfg(feature = "logging")]
    // web_sys::console::log_1(&"beef ore".into());

    // // // // job_alert(name, desc, user, rate);

    // let response =
    //     r#await!(Client::new("http://127.0.0.1:8080/graphql").call(JobQuery, variables)).unwrap();

    Client::new("http://127.0.0.1:8080/graphql")
        .call(JobQuery, variables)
        .map(|_response| {})
        .map_err(|_err| {})
        .then(|_| Ok(()))

    // #[cfg(feature = "logging")]
    // web_sys::console::log_1(&"aft her".into());
    // .and_then(|response| {});
}

// pub fn get_jobs() {
//     ""
// }
