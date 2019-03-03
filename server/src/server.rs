extern crate actix_web;
extern crate env_logger;
use self::actix_web::{fs, http, middleware, HttpRequest, HttpResponse, Json, Responder, Result};

use isomorphic_app::api::{endpoints, models};
use isomorphic_app::App;

const HTML_PLACEHOLDER: &str = "#HTML_INSERTED_HERE_BY_SERVER#";
const CSS_PLACEHOLDER: &str = "#CSS_PATH#";
const STATE_PLACEHOLDER: &str = "#INITIAL_STATE_JSON#";

fn index(req: &HttpRequest) -> impl Responder {
    let app = App::new(
        req.query()
            .get("init")
            .map(|string| string.parse().expect("bad param"))
            .unwrap_or(1001),
    );
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

    HttpResponse::Ok().content_type("text/html").body(html)
}

fn jobs_handler(job: Json<models::Job>) -> Result<String> {
    Ok(format!("{}", job.name))
}

pub fn serve() {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let server = actix_web::server::new(|| {
        let app = actix_web::App::new().middleware(middleware::Logger::default());
        let app = app.resource("/", |r| r.f(index));

        // Development
        #[cfg(debug_assertions)]
        let app = app.handler("/", fs::StaticFiles::new("../client/build").unwrap());

        // Production
        #[cfg(not(debug_assertions))]
        let app = app.handler("/", fs::StaticFiles::new("../client/dist").unwrap());

        // API
        let jobs_path = endpoints::get_path(&endpoints::Endpoint::Jobs);
        let app = app.resource(jobs_path, |r| {
            r.method(http::Method::GET).with(jobs_handler)
        });

        app
    });

    let path = std::env::current_dir().unwrap();
    println!("The current directory is {}", path.display());

    let server = server.bind("0.0.0.0:7878").unwrap();

    println!("Listening on port 7878");
    server.run();
}
