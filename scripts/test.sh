#!/bin/bash

cd $(dirname $0)

cd ../tests
cargo +nightly test --tests -- --test-threads=1
