#!/bin/bash

set -euo pipefail

TARGET=web

cargo fmt
wasm-pack build --target $TARGET --no-typescript
rm -rf pkg/.gitignore pkg/package.json pkg/README.md
