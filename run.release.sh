#!/bin/bash

cd $(dirname $0)

cd server

./target/x86_64-unknown-linux-gnu/release/isomorphic-server
