#![allow(non_snake_case)]
#![allow(unused_doc_comments)]

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate serde_derive;

pub mod agent;
pub mod catalog;
pub mod connect_ca;
pub mod errors;
pub mod health;
pub mod kv;
pub mod session;

mod request;

use std::env;

use std::time::Duration;

use reqwest::Client as HttpClient;
use reqwest::ClientBuilder;

use errors::{Result, ResultExt};

#[derive(Clone, Debug)]
pub struct Client {
    config: Config,
}

impl Client {
    pub fn new(config: Config) -> Self {
        Client { config }
    }
}

#[derive(Clone, Debug)]
pub struct Config {
    pub address: String,
    pub datacenter: Option<String>,
    pub http_client: HttpClient,
    pub token: Option<String>,
    pub wait_time: Option<Duration>,
}

impl Config {
    pub fn new_from_env() -> Result<Config> {
        let consul_addr = env::var("CONSUL_HTTP_ADDR")
            .chain_err(|| "CONSUL_HTTP_ADDR is not set")?;

        let consul_token = env::var("CONSUL_HTTP_TOKEN").ok();

        ClientBuilder::new()
            .build()
            .chain_err(|| "Failed to build reqwest client")
            .map(|client| Config {
                address: consul_addr,
                datacenter: None,
                http_client: client,
                token: consul_token,
                wait_time: None,
            })
    }

    pub fn new_from_consul_host(
        host: &str,
        port: Option<u16>,
        token: Option<String>,
    ) -> Result<Config> {
        ClientBuilder::new()
            .build()
            .chain_err(|| "Failed to build reqwest client")
            .map(|client| Config {
                address: format!("{}:{}", host, port.unwrap_or(8500)),
                datacenter: None,
                http_client: client,
                token,
                wait_time: None,
            })
    }
}

#[derive(Clone, Debug, Default)]
pub struct QueryOptions {
    pub datacenter: Option<String>,
    pub wait_index: Option<u64>,
    pub wait_time: Option<Duration>,
}

#[derive(Clone, Debug)]
pub struct QueryMeta {
    pub last_index: Option<u64>,
    pub request_time: Duration,
}

#[derive(Clone, Debug, Default)]
pub struct WriteOptions {
    pub datacenter: Option<String>,
}

#[derive(Clone, Debug)]
pub struct WriteMeta {
    pub request_time: Duration,
}
