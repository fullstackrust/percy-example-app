#!/bin/bash

cd $(dirname $0)

cd ./client
./build-wasm.sh

cd ../server
RUSTFLAGS="$RUSTFLAGS -A dead_code" OUTPUT_CSS="$(pwd)/../client/build/app.css" cargo run -p isomorphic-server
