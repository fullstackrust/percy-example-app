#!/bin/bash

rustup component add rls rust-analysis rust-src
cargo +nightly install cargo-edit minifier wasm-bindgen-cli wasm-pack
# cargo install graphql_client_cli --force
