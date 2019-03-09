#!/bin/bash

cd $(dirname $0)

mkdir -p build/
mkdir -p dist/

cp favicon.ico build/

wasm-pack build --target no-modules --no-typescript --out-dir ./build
