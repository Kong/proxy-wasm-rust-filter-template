Proxy-Wasm Rust Filter Template
===============================

This repository contains a [Proxy-Wasm](https://github.com/proxy-wasm/spec)
filter that can be used as a reference to quickly start implementing new
filters in Rust.

This is a "hello-world" type filter performing a minimal operation and showing
the basics of filter configuration: it simply adds a new header `X-Greeting`
to a proxied response. You can pass the filter a JSON configuration in the
format `{ "my_greeting": "Howdy!" }` to change the value of the injected
header.

Please refer to [WasmX
docs](https://github.com/Kong/ngx_wasm_module/blob/main/docs/PROXY_WASM.md#supported-entrypoints)
for the list of supported functions and entrypoints.

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

Testing
=======

In the `test/` folder you will find a shell script called `demo.sh` which
builds the filter, then uses Docker to spin up a [Kong
Gateway](https://konghq.com) container with the filter enabled, and issues a
request using [httpie](https://httpie.io). It uses the configuration
`test/config`. You should see the `X-Greeting` header defined in that
configuration in the httpie output.
