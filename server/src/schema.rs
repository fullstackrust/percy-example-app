#[macro_use]
use juniper;
use juniper::FieldResult;

use crate::model::Database;

// #[derive(GraphQLObject)]
// #[graphql()]
// pub struct Job {
//     id: String,         // TODO type
//     date_added: String, // TODO type
//     name: String,
//     desc: String,
//     user_id: String,   // TODO user_id
//     rate: String,      // TODO type
//     rate_type: String, // TODO type
// }

#[derive(GraphQLObject)]
#[graphql()]
pub struct GraphQLJob {
    pub name: String,
    pub desc: String,
    pub user: String,
    pub rate: String,
}

#[derive(GraphQLInputObject)]
#[graphql()]
pub struct InputJob {
    pub name: String,
    pub desc: String,
    pub user: String,
    pub rate: String,
}

// Now, we create our root Query and Mutation types with resolvers by using the
// graphql_object! macro.
// Objects can have contexts that allow accessing shared state like a database
// pool.

pub struct Context {
    // Use your real database pool here.
    pub db: Database,
}

// To make our context usable by Juniper, we have to implement a marker trait.
impl juniper::Context for Context {}

pub struct Query;

graphql_object!(Query: Context |&self| {
    field apiVersion() -> &str {
        "1.0"
    }

    field jobs(&executor) -> FieldResult<Vec<GraphQLJob>> {
        let context = executor.context();
        let jobs = context.db.get_jobs().unwrap();
        Ok(jobs)
    }
});

pub struct Mutation;

graphql_object!(Mutation: Context |&self| {
    field createJob(&executor, job: InputJob) -> FieldResult<GraphQLJob> {
        let context = executor.context();
        let job: GraphQLJob = context.db.set_job(&job).unwrap();
        Ok(job)
    }
});

// A root schema consists of a query and a mutation.
// Request queries can be executed against a RootNode.
pub type Schema = juniper::RootNode<'static, Query, Mutation>;

pub fn schema() -> Schema {
    Schema::new(Query, Mutation)
}
