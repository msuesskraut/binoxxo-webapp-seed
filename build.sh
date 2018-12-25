#!/usr/bin/env bash
cargo build --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/debug/binoxxo_webapp_seed.wasm --no-modules --out-dir ./pkg
sass ./assets/main.scss ./dist/main.css
cp index.html ./dist
cp assets/favicon.png ./dist