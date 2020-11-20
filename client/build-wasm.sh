#!/bin/bash

cd $(dirname $0)

mkdir -p build/
mkdir -p dist/

cp favicon.ico build/

wasm-pack +nightly build --target=web --no-typescript --out-dir ./build
