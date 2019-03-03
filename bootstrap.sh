#!/bin/bash

git submodule init
git submodule update

rustup install nightly-2019-02-27
rustup default nightly-2019-02-27
rustup component add rls rust-analysis rust-src
cargo install cargo-edit minifier wasm-bindgen-cli wasm-pack
