#!/bin/bash

set -euo pipefail
NAME=wasm_perocator.wasm
TARGET=wasm32-unknown-unknown
BINARY=target/$TARGET/release/$NAME

cargo build --target $TARGET --release
wasm-strip $BINARY
mkdir -p www
wasm-opt -o www/$NAME -Oz $BINARY
ls -l www/$NAME