#!/bin/bash

cd $(dirname $0)

cd server

./target/debug/isomorphic-server
