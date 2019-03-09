use juniper;
use juniper::FieldResult;

use crate::model::Database;

#[derive(GraphQLObject)]
#[graphql()]
pub struct OutputJob {
    pub id: String,         // TODO type
    pub date_added: String, // TODO type
    pub job_desc: String,
    pub job_name: String,
    pub job_rate_type: String, // TODO type
    pub job_rate: String,      // TODO type
    pub user_id: String,       // TODO user_id
    pub user_name: String,
}

#[derive(GraphQLInputObject)]
#[graphql()]
pub struct InputJob {
    pub name: String,
    pub desc: String,
    pub user: String,
    pub rate: String,
}

pub struct Context {
    pub db: Database,
}

impl juniper::Context for Context {}

pub struct Query;

graphql_object!(Query: Context |&self| {
    field apiVersion() -> &str {
        "1.0"
    }

    field jobs(&executor) -> FieldResult<Vec<OutputJob>> {
        let context = executor.context();
        let jobs = context.db.get_jobs().unwrap();
        Ok(jobs)
    }
});

pub struct Mutation;

graphql_object!(Mutation: Context |&self| {
    field createJob(&executor, job: InputJob) -> FieldResult<OutputJob> {
        let context = executor.context();
        let result: OutputJob = context.db.set_job(&job).unwrap();
        Ok(result)
    }
});

// A root schema consists of a query and a mutation.
// Request queries can be executed against a RootNode.
pub type Schema = juniper::RootNode<'static, Query, Mutation>;

pub fn schema() -> Schema {
    Schema::new(Query, Mutation)
}
