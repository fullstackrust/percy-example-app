extern crate pretty_env_logger;

use isomorphic_app::api::models;
use isomorphic_app::App;
use warp::path;
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

    let jobs_post = warp::post2()
        .and(path!("api" / "jobs"))
        .and(warp::body::json())
        .map(|job: models::Job| {
            let name = job.name;
            warp::reply::json(&name)
        });

    let routes = index.or(files).or(jobs_post);

    warp::serve(routes).run(([127, 0, 0, 1], 7878));
}
