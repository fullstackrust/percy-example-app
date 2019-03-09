extern crate bytevec;
extern crate chrono;
extern crate dirs;
extern crate isomorphic_app;
extern crate isomorphic_server;
extern crate juniper;
extern crate juniper_warp;
extern crate log;
extern crate serde;
extern crate serde_bytes;
extern crate serde_derive;
extern crate sled;
extern crate ulid;
extern crate warp;

use isomorphic_server::server;

fn main() {
    std::env::set_var("RUST_LOG", "debug");
    pretty_env_logger::init();
    server::serve();
}
