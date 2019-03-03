extern crate isomorphic_app;
extern crate isomorphic_server;
extern crate warp;

use isomorphic_server::server;

fn main() {
    server::serve();
}
