## consul-rust

[![Build Status](https://github.com/pierresouchay/consul-rust/actions/workflows/rust.yml/badge.svg)](https://github.com/pierresouchay/consul-rust/actions?query=branch%3Amaster)
[![](https://img.shields.io/crates/v/consul.svg)](https://crates.io/crates/consul)

[Documentation here](https://docs.rs/consul/).

Rust client libray for [Consul](http://consul.io/) HTTP API

### Usage

```rust
    use std::collections::HashMap;
    use consul::{Client, Config, QueryMeta};
    use consul::catalog::Catalog;

    #[tokio::main]
    async fn main() -> Result<(), Box<dyn std::error::Error>> {
        let config = Config::new().unwrap();
        let client = Client::new(config);
		let services: (HashMap<String, String>, QueryMeta) = client.services(None).unwrap();
		println!("{:?}", services);
		Ok((
    }
```


For more examples, see the **[tests](https://github.com/stusmall/consul-rust/blob/master/tests)** .

### Installation

Simply include the consul-rust in your Cargo dependencies.

```
[dependencies]
consul = "0.5"
```
