# Demo Webassembly

WASM tutorial.

Install [wasmedge](https://wasmedge.org/docs/start/install#generic-linux-and-macos)

## Build

``` shell
cargo build --target wasm32-wasi --release
```

## Run

``` shell
wasmedge --dir .:. target/wasm32-wasi/release/wasi.wasm
```

