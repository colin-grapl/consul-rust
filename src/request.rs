use std::collections::HashMap;
use url::Url;

use std::str;
use std::str::FromStr;
use std::time::Instant;

use reqwest::Client as HttpClient;
use reqwest::RequestBuilder;
use reqwest::header::HeaderValue;
use reqwest::StatusCode;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::errors::{Result, ResultExt};
use crate::{Config, QueryMeta, QueryOptions, WriteMeta, WriteOptions};

fn add_config_options(builder: RequestBuilder, config: &Config) -> RequestBuilder {
    match &config.token {
        Some(val) => builder.header("X-Consul-Token", val),
        None => builder,
    }
}

pub async fn get_vec<R: DeserializeOwned>(
    path: &str,
    config: &Config,
    mut params: HashMap<String, String>,
    options: Option<&QueryOptions>,
) -> Result<(Vec<R>, QueryMeta)> {
    let datacenter: Option<&String> = options
        .and_then(|o| o.datacenter.as_ref())
        .or_else(|| config.datacenter.as_ref());

    if let Some(dc) = datacenter {
        params.insert(String::from("dc"), dc.to_owned());
    }
    if let Some(options) = options {
        if let Some(index) = options.wait_index {
            params.insert(String::from("index"), index.to_string());
        }
        if let Some(wait_time) = options.wait_time {
            params.insert(String::from("wait"), format!("{}s", wait_time.as_secs()));
        }
    }

    let url_str = format!("{}{}", config.address, path);
    let url =
        Url::parse_with_params(&url_str, params.iter()).chain_err(|| "Failed to parse URL")?;
    let start = Instant::now();
    let request_builder = add_config_options(config.http_client.get(url), &config);
    let response = request_builder.send();
    let r = response
        .await
        .chain_err(|| "HTTP request to consul failed")?;


    let x: Option<Result<u64>> = r
        .headers()
        .get("X-Consul-Index")
        .map(|value: &HeaderValue| value.as_bytes())
        .map(|bytes| {
            str::from_utf8(bytes)
                .chain_err(|| "Failed to parse valid UT8 for last index")
                .and_then(|s| {
                    u64::from_str(s)
                        .chain_err(|| "Failed to parse valid number for last index")
                })
        });
    let j = if r.status() != StatusCode::NOT_FOUND {
        r.json().await.chain_err(|| "Failed to parse JSON response")?
    } else {
        Vec::new()
    };
    let x = match x {
        Some(r) => (j, Some(r?)),
        None => (j, None),
    };
    Ok((
        x.0,
        QueryMeta {
            last_index: x.1,
            request_time: Instant::now() - start,
        },
    ))
}

pub async fn get<R: DeserializeOwned>(
    path: &str,
    config: &Config,
    mut params: HashMap<String, String>,
    options: Option<&QueryOptions>,
) -> Result<(R, QueryMeta)> {
    let datacenter: Option<&String> = options
        .and_then(|o| o.datacenter.as_ref())
        .or_else(|| config.datacenter.as_ref());

    if let Some(dc) = datacenter {
        params.insert(String::from("dc"), dc.to_owned());
    }
    if let Some(options) = options {
        if let Some(index) = options.wait_index {
            params.insert(String::from("index"), index.to_string());
        }
        if let Some(wait_time) = options.wait_time {
            params.insert(String::from("wait"), format!("{}s", wait_time.as_secs()));
        }
    }

    let url_str = format!("{}{}", config.address, path);
    let url =
        Url::parse_with_params(&url_str, params.iter()).chain_err(|| "Failed to parse URL")?;
    let start = Instant::now();
    let request_builder = add_config_options(config.http_client.get(url), &config);
    let response = request_builder.send();
    let r = response
        .await
        .chain_err(|| "HTTP request to consul failed")?;

    let x: Option<Result<u64>> =
        r.headers()
            .get("X-Consul-Index")
            .map(|bytes: &HeaderValue| -> Result<u64> {
                bytes
                    .to_str()
                    .chain_err(|| "Failed to parse valid UT8 for last index")
                    .and_then(|s: &str| -> Result<u64> {
                        u64::from_str(s)
                            .chain_err(|| "Failed to parse valid number for last index")
                    })
            });
    let j = r.json().await.chain_err(|| "Failed to parse JSON response")?;
    let x = match x {
        Some(r) => (j, Some(r?)),
        None => (j, None),
    };

    Ok((
        x.0,
        QueryMeta {
            last_index: x.1,
            request_time: Instant::now() - start,
        },
    ))
}

pub async fn delete<R: DeserializeOwned>(
    path: &str,
    config: &Config,
    params: HashMap<String, String>,
    options: Option<&WriteOptions>,
) -> Result<(R, WriteMeta)> {
    let req = |http_client: &HttpClient, url: Url| -> RequestBuilder { http_client.delete(url) };
    write_with_body(path, None as Option<&()>, config, params, options, req).await
}

/*
pub fn post<T: Serialize, R: DeserializeOwned>(path: &str,
                                               body: Option<&T>,
                                               config: &Config,
                                               options: Option<&WriteOptions>)
                                               -> Result<(R, WriteMeta)> {
    let req = |http_client: &HttpClient, url: Url| -> RequestBuilder { http_client.post(url) };
    write_with_body(path, body, config, options, req)
}
*/
pub async fn put<T: Serialize, R: DeserializeOwned>(
    path: &str,
    body: Option<&T>,
    config: &Config,
    params: HashMap<String, String>,
    options: Option<&WriteOptions>,
) -> Result<(R, WriteMeta)> {
    let req = |http_client: &HttpClient, url: Url| -> RequestBuilder { http_client.put(url) };
    write_with_body(path, body, config, params, options, req).await
}

pub async fn write_with_body<T: Serialize, R: DeserializeOwned, F>(
    path: &str,
    body: Option<&T>,
    config: &Config,
    mut params: HashMap<String, String>,
    options: Option<&WriteOptions>,
    req: F,
) -> Result<(R, WriteMeta)>
where
    F: Fn(&HttpClient, Url) -> RequestBuilder,
{
    let start = Instant::now();
    let datacenter: Option<&String> = options
        .and_then(|o| o.datacenter.as_ref())
        .or_else(|| config.datacenter.as_ref());

    if let Some(dc) = datacenter {
        params.insert(String::from("dc"), dc.to_owned());
    }

    let url_str = format!("{}{}", config.address, path);
    let url =
        Url::parse_with_params(&url_str, params.iter()).chain_err(|| "Failed to parse URL")?;
    let builder = req(&config.http_client, url);
    let builder = if let Some(b) = body {
        builder.json(b)
    } else {
        builder
    };
    let builder = add_config_options(builder, &config);
    let x = builder
        .send()
        .await
        .chain_err(|| "HTTP request to consul failed")?;
    let x = x.json().await.chain_err(|| "Failed to parse JSON")?;
    Ok((
        x,
        WriteMeta {
            request_time: Instant::now() - start,
        },
    ))
}
