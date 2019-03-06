extern crate isomorphic_app;
extern crate isomorphic_server;
#[macro_use]
extern crate juniper;
extern crate dirs;
extern crate juniper_warp;
extern crate sled;
extern crate warp;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_bytes;
#[macro_use]
extern crate bytevec;
extern crate chrono;
#[macro_use]
extern crate log;
extern crate ulid;

use isomorphic_server::server;

fn main() {
    std::env::set_var("RUST_LOG", "debug");
    pretty_env_logger::init();
    server::serve();
}
