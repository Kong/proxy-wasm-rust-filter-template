Proxy-Wasm Rust Filter Template
===============================

This repository contains a [Proxy-Wasm](https://github.com/proxy-wasm/spec)
filter that can be used as a reference to quickly start implementing new
filters. The filter implements the entrypoints and function calls supported by
WasmX. Please refer to [WasmX docs](https://github.com/Kong/ngx_wasm_module/blob/main/docs/PROXY_WASM.md#supported-entrypoints)
for the list of supported entrypoints.

Requirements
============

* Rust
  * [rustup.rs](https://rustup.rs) is the easiest way to install Rust.
    * Then add the Wasm32-WASI target to your toolchain: `rustup target add wasm32-wasi`.

Build
=====

Once the environment is set up with `cargo` in your PATH,
you can build it with:

```
cargo build --release
```

This will produce a .wasm file in `target/wasm32-wasi/release/`.
