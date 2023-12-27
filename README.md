# reqwestplus

A fork of [reqwestplus](github.com/ignassew/reqwestplus) and [reqwest-impersonate](https://github.com/4JX/reqwest-impersonate) containing further patches updating the libraries patched more mainstream.

This fork has features designed to match my specific usecase, and although open source, it isn't intended for general use. I can't guarantee API stability, or that any of additional features will make sense to you.

## Features added
- Impersonate Chrome's TLS and HTTP/2 fingerprint
- Change proxy on an initialized client
- Custom header order with custom implementation of http's HeaderMap that preserves the insert order, and `header_order` option in ClientBuilder
- Ability to disable default `accept` header with `accept_header` option in ClientBuilder


It is currently missing HTTP/2 `PRIORITY` support. (PRs to [h2](https://github.com/hyperium/h2) are welcome)

**Notice:** This crate depends on patched dependencies. To use it, please add the following to your `Cargo.toml`.

```toml
[patch.crates-io]
hyper = { git = "https://github.com/shrimpyuk/hyperplus.git", branch = "0.14.28-patched" }
http = { git = "https://github.com/shrimpyuk/http-ordered.git" }
```

These patches were made specifically for `reqwestplus` to work, but I would appreciate if someone took the time to PR more "proper" versions to the parent projects.

## Example

`Cargo.toml`

```toml
reqwest = { git = "https://github.com/shrimpyuk/reqwestplus", package = "reqwestplus", branch = "master", default-features = false, features = [
    "chrome",
    "blocking",
] }
```

`main.rs`

```rs
use reqwestplus::browser::ChromeVersion;

fn main() {
    // Build a client to mimic Chrome 104
    let client = reqwestplus::blocking::Client::builder()
        .chrome_builder(ChromeVersion::V104)
        .build()
        .unwrap();

    // Use the API you're already familiar with
    match client.get("https://yoururl.com").send() {
        Ok(res) => {
            println!("{:?}", res.text().unwrap());
        }
        Err(err) => {
            dbg!(err);
        }
    };
}
```

## Original readme

[![crates.io](https://img.shields.io/crates/v/reqwest.svg)](https://crates.io/crates/reqwest)
[![Documentation](https://docs.rs/reqwest/badge.svg)](https://docs.rs/reqwest)
[![MIT/Apache-2 licensed](https://img.shields.io/crates/l/reqwest.svg)](./LICENSE-APACHE)
[![CI](https://github.com/seanmonstar/reqwest/workflows/CI/badge.svg)](https://github.com/seanmonstar/reqwest/actions?query=workflow%3ACI)

An ergonomic, batteries-included HTTP Client for Rust.

- Plain bodies, JSON, urlencoded, multipart
- Customizable redirect policy
- HTTP Proxies
- HTTPS via system-native TLS (or optionally, rustls)
- Cookie Store
- WASM
- [Changelog](CHANGELOG.md)

## Example

This asynchronous example uses [Tokio](https://tokio.rs) and enables some
optional features, so your `Cargo.toml` could look like this:

```toml
[dependencies]
reqwest = { version = "0.11", features = ["json"] }
tokio = { version = "1", features = ["full"] }
```

And then the code:

```rust,no_run
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp);
    Ok(())
}
```

## Blocking Client

There is an optional "blocking" client API that can be enabled:

```toml
[dependencies]
reqwest = { version = "0.11", features = ["blocking", "json"] }
```

```rust,no_run
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get("https://httpbin.org/ip")?
        .json::<HashMap<String, String>>()?;
    println!("{:#?}", resp);
    Ok(())
}
```

## Requirements

On Linux:

- OpenSSL 1.0.1, 1.0.2, 1.1.0, or 1.1.1 with headers (see <https://github.com/sfackler/rust-openssl>)

On Windows and macOS:

- Nothing.

Reqwest uses [rust-native-tls](https://github.com/sfackler/rust-native-tls),
which will use the operating system TLS framework if available, meaning Windows
and macOS. On Linux, it will use OpenSSL 1.1.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
