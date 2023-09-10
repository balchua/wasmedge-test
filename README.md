# Demo Webassembly

WASM tutorial using Rust and Wasmedge.  This example serves an endpoint at 8080.

Install [wasmedge](https://wasmedge.org/docs/start/install#generic-linux-and-macos)

## Build

``` shell
cargo build --target wasm32-wasi --release
wasmedge compile target/wasm32-wasi/release/wasm-tut.wasm wasm_tut.wasm
```

## Run

``` shell
wasmedge --dir .:. wasm_tut.wasm
```

## Test

``` shell
curl http://localhost:8080/hello?user=The%20quick -X POST

# With unicode characters
curl http://localhost:8080/hello?user=%E7%A7%8B%E6%94%B6%E5%86%AC%E8%97%8F%E5%A5%B3 -X POST
Hello 秋收冬藏女

# Using utf8 encoded :poop: emoji
# Example emojis and its utf8 encoding
# :smile: is %F0%9F%98%81
# :rofl: is %F0%9F%A4%A3
# :poop: is %F0%9F%92%A9
curl http://localhost:8080/hello?user=%F0%9F%92%A9 -X POST
Hello 💩
```

Or reverse a string

``` shell
curl -vv  http://localhost:8080/echo/reversed -X POST -d 'yellow'
```

Or echo the string "echo"

``` shell
curl http://localhost:8080/echo/echo
```


