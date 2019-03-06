
extern crate isomorphic_app;
extern crate isomorphic_server;
#[macro_use] extern crate juniper;
extern crate juniper_warp;
extern crate warp;
extern crate sled;
extern crate dirs;
#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_bytes;
#[macro_use] extern crate bytevec;

use isomorphic_server::server;

fn main() {
    server::serve();
}
