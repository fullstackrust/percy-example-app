#!/bin/bash

cd $(dirname $0)

rm -rf dist/
mkdir -p dist/

cp favicon.ico dist/

wasm-pack +nightly build --target=web --no-typescript --out-dir ./dist --release
