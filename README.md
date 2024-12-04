# httpbin-rs 🦀

Rust http-client implementation for the [`httpbin.org`](https://httpbin.org). It's currently on the **WIP**
state and also purely for learning Rust's borrow checker, workspace / project structure, feature flags
and so on.

[WIP] Endpoints
---------

- HTTP Methods ✅
- Auth ⌛
- Auth methods ⌛
- Status codes ⌛
- Request inspection ⌛
- Response inspection ⌛
- Response formats ⌛
- Dynamic data ⌛
- Cookies ⌛
- Images ✅
- Redirects ⌛
- Anything ⌛

Getting started
---------------

Install the `rustup`
```
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Check that installation was successful
```
$ rustc --version
$ cargo --version
```

Update `rustc` and `cargo`  
```
$ rustup update stable
```

httpbin
-------

Some of the useful commands running the workspace.

Run unit tests 
```
$ cargo test
```

Run specific unit test 
```
$ cargo test test_library_len
```

Run test with println
```
$ cargo test http_methods_post_with_inputs 
```

Run tests only in a specific package `httpbin`
```
$ cargo test --package httpbin
```

Run tests with the clean result list
```
$ cargo test --package httpbin -- --show-output
```

Run example `hello-world` with the logging
```
$ RUST_LOG=reqwest=trace cargo run --example hello-world
```

Run example `hello-world` with the println
```
$ cargo run --example hello-world -- --nocapture 
```

Build release binary of the example
```
$ cargo build --release --example hello-world
```

rust-lsp
--------

Install the rust-lsp
```
$ rustup component add rust-analyzer
```

Configure the rust-analyzer
```
$ sudo ln -s $(rustup which rust-analyzer ) /usr/local/bin/rust-analyzer
```

rustfmt
-------

Create the .toml config file to. See more at [`rustfmt`](https://rust-lang.github.io/rustfmt/?version=v1.5.1&search=)
```
$ hx ~/Library/Application\ Support/rustfmt/rustfmt.toml
```

third-party-libs
----------------

Discover third party libraries more at [`lib.rs`](https://lib.rs/)
