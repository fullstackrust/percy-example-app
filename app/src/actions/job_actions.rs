use crate::store::Store;
use graphql_client::{web::Client, GraphQLQuery};
use std::cell::{Ref, RefCell};
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

#[derive(GraphQLQuery)]
#[graphql(
    query_path = "../graphql/job_query.graphql",
    schema_path = "../graphql/job_schema.graphql"
)]
struct Job;

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

fn log(s: &str) {
    web_sys::console::log_1(&JsValue::from_str(s))
}

pub async fn post_job(store: Rc<RefCell<Store>>) -> Result<JsValue, JsValue> {
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

    let client = Client::new("http://127.0.0.1:8080/graphql");

    client.call(Job, variables).await.map_err(|err| {
        log(&format!(
            "Could not fetch jobs. graphql_client_web error: {:?}",
            err
        ));
        JsValue::NULL
    })?;

    Ok(JsValue::NULL)

    // #[cfg(feature = "logging")]
    // web_sys::console::log_1(&"aft her".into());
    // .and_then(|response| {});
}

// pub fn get_jobs() {
//     ""
// }
