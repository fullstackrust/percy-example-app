#!/bin/bash

cd $(dirname $0)

cd ./client
./build-wasm.prod.sh
wasm-bindgen ./dist/isomorphic_client_bg.wasm --out-dir ./dist
wasm-opt -Oz -o ./dist/isomorphic_client_bg.wasm ./dist/isomorphic_client_bg_gc.wasm

cd ../server
OUTPUT_CSS="$(pwd)/../client/dist/app.css" cargo +nightly-2019-02-27 build -p isomorphic-server --release --target x86_64-unknown-linux-gnu
minifier "$(pwd)/../client/dist/app.css"
