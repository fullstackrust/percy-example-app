use fullstackrust_percy_server::server;

fn main() {
    std::env::set_var("RUST_LOG", "debug");
    pretty_env_logger::init();
    server::serve();
}
