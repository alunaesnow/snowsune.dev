#!/bin/bash
cargo build --release --target=wasm32-unknown-unknown &&
wasm-bindgen target/wasm32-unknown-unknown/release/rs.wasm --out-dir pkg &&
wasm-opt -Os --enable-bulk-memory --all-features -o out.wasm pkg/rs_bg.wasm &&
rm -rf ../src/lib/pkg
cp -r pkg ../src/lib/pkg
eza -l pkg