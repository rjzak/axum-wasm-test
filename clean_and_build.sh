#!/bin/bash

echo "Cleaning..."
cargo clean
echo "Updating..."
cargo update
echo "Building..."
if cargo build --target=wasm32-wasi; then
    cargo build --target=x86_64-unknown-linux-gnu
else
    echo "Build error!"
fi
