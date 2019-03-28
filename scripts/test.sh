#!/bin/bash

cd $(dirname $0)

cd ../tests
cargo test --tests -- --test-threads=1
