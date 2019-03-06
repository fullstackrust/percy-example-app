extern crate pretty_env_logger;

use crate::model::Database;
use crate::schema::{schema, Context};
use isomorphic_app::App;
use warp::Filter;

const HTML_PLACEHOLDER: &str = "#HTML_INSERTED_HERE_BY_SERVER#";
const CSS_PLACEHOLDER: &str = "#CSS_PATH#";
const STATE_PLACEHOLDER: &str = "#INITIAL_STATE_JSON#";

pub fn serve() {
    std::env::set_var("RUST_LOG", "warp=info");
    pretty_env_logger::init();

    // Development
    #[cfg(debug_assertions)]
    let files = warp::fs::dir("../client/build");;

    // Production
    #[cfg(not(debug_assertions))]
    let files = warp::fs::dir("../client/dist");;

    let index = warp::path::end().map(|| {
        let app = App::new();
        let state = app.store.borrow();

        let html = format!("{}", include_str!("./index.html"));
        let html = html.replacen(HTML_PLACEHOLDER, &app.render().to_string(), 1);
        let html = html.replacen(STATE_PLACEHOLDER, &state.to_json(), 1);

        // Development
        #[cfg(debug_assertions)]
        let html = html.replacen(CSS_PLACEHOLDER, "app.css", 2);

        // Production
        #[cfg(not(debug_assertions))]
        let html = html.replacen(CSS_PLACEHOLDER, "app.min.css", 2);

        warp::reply::html(html)
    });

    // let state = warp::any().map(move || Database::new());
    let graphql_filter = juniper_warp::make_graphql_filter(
        schema(),
        warp::any()
            .map(|| {
                (Context {
                    db: Database::new().unwrap(),
                })
            })
            .boxed(),
    );
    let graphql = warp::path("graphql").and(graphql_filter);

    let routes = index.or(files).or(graphql);

    warp::serve(routes).run(([127, 0, 0, 1], 7878));
}
