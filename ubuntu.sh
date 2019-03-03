#!/bin/bash

apt-get update
apt-get upgrade
apt-get install -y build-essential git cmake python pkg-config libssl-dev

git clone git@github.com:WebAssembly/binaryen.git
cd binaryen
cmake .
make
make install

cd ..

git clone git@github.com:innotradeplatform/innotrade-contractor.git
cd innotrade-contractor

curl https://sh.rustup.rs -sSf | sh

./bootstrap.sh
./build.release.sh
