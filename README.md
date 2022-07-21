# Test code for Axum, Tokio, WASI

## Compiling:
1. Build with Tokio unstable flag:
    * `RUSTFLAGS="--cfg tokio_unstable" cargo build`

## Run with Enarx:
1. Get [Enarx](https://github.com/enarx/enarx)
2. Run: `CARGO_TARGET_WASM32_WASI_RUNNER="enarx run --wasmcfgfile Enarx.toml" cargo run --target=wasm32-wasi`

## Run with Wasmtime:
1. Get [Wasmtime](https://wasmtime.dev/)
2. Run: `CARGO_TARGET_WASM32_WASI_RUNNER="wasmtime run --tcplisten 127.0.0.1:8080 --env FD_COUNT=3" cargo run --target=wasm32-wasi`
