#!/bin/bash

cd $(dirname $0)

cd ./client
./build-wasm.prod.sh

cd ../server
OUTPUT_CSS="$(pwd)/../client/dist/app.css" cargo build -p fullstackrust-percy-server --release --target x86_64-unknown-linux-gnu
minifier "$(pwd)/../client/dist/app.css"
