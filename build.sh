#!/bin/bash

set -euo pipefail

TARGET=web
BUILD_DIR=dist

cargo fmt
wasm-pack build --target $TARGET --no-typescript -d $BUILD_DIR
cd $BUILD_DIR
rm -rf .gitignore package.json README.md
ls -l *.wasm
cd ..