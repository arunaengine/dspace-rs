#[macro_use]
extern crate serde_derive;
extern crate dsp_api as dsp_models;

pub use dsp_models::common::DspaceVersion;

pub use dsp_models::catalog::Catalog;
pub use dsp_models::catalog::CatalogError;
pub use dsp_models::catalog::CatalogRequestMessage;
pub use dsp_models::catalog::Dataset;
pub use dsp_models::catalog::DatasetRequestMessage;

use std::error;
use std::fmt;

pub mod configuration;
pub mod utils;

pub mod common {
    pub mod dspace_version;
}

pub mod contract_negotiation {
    pub mod negotiation_consumer_api;
    pub mod negotiation_provider_api;
}

pub mod catalog {
    pub mod catalog_api;
}

pub mod transfer_process {
    pub mod transfer_consumer_api;
    pub mod transfer_provider_api;
}

pub const PROVIDER_PROTOCOL: &str = "http://provider-connector:9194/protocol";
pub const PROVIDER_ID: &str = "provider";
pub const PROVIDER_DSP_HOST: &str = "provider-connector:9194";
pub const CONSUMER_PROTOCOL: &str = "http://consumer-connector:9194/protocol";
pub const CONSUMER_ID: &str = "consumer";
pub const CONSUMER_DSP_HOST: &str = "consumer-connector:9194";

#[derive(Debug, Clone)]
pub struct ResponseContent<T> {
    pub status: reqwest::StatusCode,
    pub content: String,
    pub entity: Option<T>,
}

#[derive(Debug)]
pub enum Error<T> {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
    ResponseError(ResponseContent<T>),
}

impl<T> fmt::Display for Error<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (module, e) = match self {
            Error::Reqwest(e) => ("reqwest", e.to_string()),
            Error::Serde(e) => ("serde", e.to_string()),
            Error::Io(e) => ("IO", e.to_string()),
            Error::ResponseError(e) => ("response", format!("status code {}", e.status)),
        };
        write!(f, "error in {}: {}", module, e)
    }
}

impl<T: fmt::Debug> error::Error for Error<T> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(match self {
            Error::Reqwest(e) => e,
            Error::Serde(e) => e,
            Error::Io(e) => e,
            Error::ResponseError(_) => return None,
        })
    }
}

impl<T> From<reqwest::Error> for Error<T> {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl<T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl<T> From<std::io::Error> for Error<T> {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}
