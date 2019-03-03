use crate::store::Store;
use std::cell::{Ref, RefCell};
use std::rc::Rc;
use wasm_bindgen::prelude::*;

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

pub fn post_job(store: Rc<RefCell<Store>>) {
    let job_name = get_field(store.borrow(), "job_name");
    let job_desc = get_field(store.borrow(), "job_desc");
    let job_user = get_field(store.borrow(), "job_user");
    let job_rate = get_field(store.borrow(), "job_rate");

    job_alert(job_name, job_desc, job_user, job_rate);
}

// pub fn get_jobs() {
//     ""
// }
