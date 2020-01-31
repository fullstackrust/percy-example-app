#!/bin/bash

rustup component add rls rust-analysis rust-src
cargo install cargo-edit minifier wasm-bindgen-cli wasm-pack
