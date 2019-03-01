#!/bin/bash

git submodule init
git submodule update

rustup install nightly-2019-02-27
rustup default nightly-2019-02-27
rustup component add rls rust-analysis rust-src
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sudo sh
cargo install cargo-edit minifier wasm-bindgen-cli 
