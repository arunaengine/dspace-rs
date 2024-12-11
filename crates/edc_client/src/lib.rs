#[macro_use]
extern crate serde_derive;
extern crate edc_api as api_models;

pub mod application_observability_api;
pub mod asset_api;
pub mod catalog_api;
pub mod configuration;
pub mod contract_agreement_api;
pub mod contract_definition_api;
pub mod contract_negotiation_api;
pub mod dataplane_selector_api;
pub mod edr_cache_api;
pub mod http_provisioner_webhook_api;
pub mod policy_definition_api;
pub mod secret_api;
pub mod transfer_process_api;
mod utils;

pub use api_models::ApiErrorDetail;
pub use api_models::AssetInput;
pub use api_models::AssetOutput;
pub use api_models::CallbackAddress;
pub use api_models::CatalogRequest;
pub use api_models::ContractAgreement;
pub use api_models::ContractDefinitionInput;
pub use api_models::ContractDefinitionOutput;
pub use api_models::ContractNegotiation;
pub use api_models::ContractOfferDescription;
pub use api_models::ContractRequest;
pub use api_models::Criterion;
pub use api_models::DataAddress;
pub use api_models::DataPlaneInstanceSchema;
pub use api_models::DatasetRequest;
pub use api_models::DeprovisionedResource;
pub use api_models::EndpointDataReferenceEntry;
pub use api_models::Failure;
pub use api_models::HealthCheckResult;
pub use api_models::HealthStatus;
pub use api_models::IdResponse;
pub use api_models::NegotiationState;
pub use api_models::Offer;
pub use api_models::PolicyDefinitionInput;
pub use api_models::PolicyDefinitionOutput;
pub use api_models::ProvisionerWebhookRequest;
pub use api_models::QuerySpec;
pub use api_models::SecretInput;
pub use api_models::SecretOutput;
pub use api_models::SelectionRequestSchema;
pub use api_models::TerminateNegotiationSchema;
pub use api_models::TerminateTransfer;
pub use api_models::TransferProcess;
pub use api_models::TransferRequest;
pub use api_models::TransferState;

use std::error;
use std::fmt;

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
