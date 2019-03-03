use serde::{Deserialize, Serialize};
use serde_json::{to_string, Result};

#[derive(Debug, Serialize, Deserialize)]
pub struct Job {
    pub name: String,
    pub desc: String,
    pub user: String,
    pub rate: String,
}

pub fn new_job(name: String, desc: String, user: String, rate: String) -> Result<String> {
    let job = Job {
        name,
        desc,
        user,
        rate,
    };

    to_string(&job)
}
