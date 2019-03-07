// use futures::Future;
// use graphql_client_web::{Client, ClientError, GraphQLQuery, Response};
// use wasm_bindgen::JsValue;

// #[derive(GraphQLQuery)]
// #[graphql(
//     schema_path = "../graphql/star_wars_schema.graphql",
//     query_path = "../graphql/star_wars_query.graphql"
// )]
// struct StarWarsQuery;

// pub fn create_job() -> impl Future<Item = Response, Error = ClientError> {
//     Client::new("http://127.0.0.1:8080/graphql").call(StarWarsQuery, StarWarsQuery::Variables)
//     // .map(|response| {

//     // })
//     // .map_err(|err| {
//     //     panic!("{:?}", err);
//     // })
// }
