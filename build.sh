#!/bin/bash

set -euo pipefail

NAME=wasm_perocator.wasm
TARGET=wasm32-unknown-unknown
BINARY=target/$TARGET/release/$NAME
DIR=dist

cargo build --target $TARGET --release
wasm-strip $BINARY

mkdir -p $DIR

wasm-opt -o $DIR/$NAME -Oz $BINARY

ls -l $DIR/$NAME