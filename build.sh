#!/bin/bash
set -e

rustup target add wasm32-unknown-unknown
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
npm ci --prefix frontend
npm run build --prefix frontend
