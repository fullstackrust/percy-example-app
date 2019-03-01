#!/bin/bash

apt-get update
apt-get upgrade
apt-get install build-essential git

git clone git@github.com:WebAssembly/binaryen.git
cd binaryen
cmake .
make
make install

git clone git@github.com:innotradeplatform/innotrade-contractor.git
cd innotrade-contractor

./bootstrap.sh
./build.release.sh
