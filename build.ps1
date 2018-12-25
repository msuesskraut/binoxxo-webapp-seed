cargo build --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/debug/binoxxo_webapp_seed.wasm --no-modules --out-dir ./pkg