use crate::model::Database;
use crate::schema::{schema, Context};
use isomorphic_app::{App, Msg};
use warp::{path::FullPath, Filter};

const HTML_PLACEHOLDER: &str = "#HTML_INSERTED_HERE_BY_SERVER#";
const CSS_PLACEHOLDER: &str = "#CSS_PATH#";
const STATE_PLACEHOLDER: &str = "#INITIAL_STATE_JSON#";

pub fn serve() {
    // Development
    #[cfg(debug_assertions)]
    let files = warp::fs::dir("../client/build");

    // Production
    #[cfg(not(debug_assertions))]
    let files = warp::fs::dir("../client/dist");

    let index = warp::path::full().map(|path: FullPath| {
        let app = App::new();

        info!("path: {}", path.as_str());
        // &state.msg(&Msg::Path(path.as_str().to_string()));

        let path = path.as_str().to_string();

        let html = format!("{}", include_str!("./index.html"));
        let html = html.replacen(HTML_PLACEHOLDER, &app.render(path).to_string(), 1);

        let state = app.store.borrow();
        let html = html.replacen(STATE_PLACEHOLDER, &state.to_json(), 1);

        // Development
        #[cfg(debug_assertions)]
        let html = html.replacen(CSS_PLACEHOLDER, "app.css", 2);

        // Production
        #[cfg(not(debug_assertions))]
        let html = html.replacen(CSS_PLACEHOLDER, "app.min.css", 2);

        warp::reply::html(html)
    });

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

    let routes = files.or(graphql).or(index);

    // Development
    #[cfg(debug_assertions)]
    warp::serve(routes).run(([127, 0, 0, 1], 8080));

    // Production
    #[cfg(not(debug_assertions))]
    warp::serve(routes).run(([165, 227, 77, 114], 80));
}
