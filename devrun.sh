#!/bin/bash
set -ex
wasm-pack build --release
pushd www
rm -rf node_modules
yarn install
yarn start
popd
