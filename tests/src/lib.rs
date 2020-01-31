#[cfg(test)]
use shellfn::shell;
use std::error::Error;

// GraphQL Endpoint Integration Tests

#[shell]
fn create_job() -> Result<String, Box<Error>> {
    r#"
        curl --request POST \
            --url http://127.0.0.1:8080/graphql \
            --header 'content-type: application/json' \
            --data '{"query":"mutation ($name: String!, $desc: String!, $user: String!, $rate: String!) {\n    createJob(job: { name: $name, desc: $desc, user: $user, rate: $rate }) {\n        id\n        dateAdded\n        jobDesc\n        jobName\n        jobRateType\n        jobRate\n        userId\n        userName\n    }\n}\n","variables":{"name":"Hunter","desc":"Another dude","user":"hunter","rate":"20/hr"}}'
    "#
}

#[test]
fn create_job_spec() {
    let res = create_job().unwrap();
    assert!(res.contains("\"userName\":\"hunter\""));
}

#[shell]
fn get_jobs() -> Result<String, Box<Error>> {
    r#"
        curl --request POST \
            --url http://127.0.0.1:8080/graphql \
            --header 'content-type: application/json' \
            --data '{"query":"{\n  jobs {\n    id\n    dateAdded\n    jobDesc\n    jobName\n    jobRateType\n    jobRate\n    userId\n    userName\n  }\n}","variables":{}}'
    "#
}

#[test]
fn get_jobs_spec() {
    let res = get_jobs().unwrap();
    assert!(res.contains("\"userName\":\"hunter\""));
}
