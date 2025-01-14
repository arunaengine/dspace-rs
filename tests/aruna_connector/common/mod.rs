extern crate dsp_client;

use edc_client::configuration::ApiKey;
use edc_client::configuration::Configuration;

pub const PROVIDER_PROTOCOL: &str = "http://localhost:3000";
pub const PROVIDER_ID: &str = "aruna-connector";
pub const DATASPACE_PROTOCOL: &str = "dataspace-protocol-http";

pub fn setup_provider_configuration() -> Configuration {
    let mut provider = Configuration::default();
    provider.base_path = "http://localhost:3000".to_string();
    provider.api_key = Some(ApiKey {
        prefix: Some("x-api-key".to_string()),
        key: "123456".to_owned(),
    });
    provider.with_headers()
}

pub async fn setup_consumer_configuration() -> Configuration {
    let mut consumer = Configuration::default();
    consumer.base_path = "http://localhost:9194/protocol".to_string();
    consumer.with_headers()
}

pub async fn setup_management_consumer() -> Configuration {
    let mut management_consumer = Configuration::default();
    management_consumer.base_path = "http://localhost:9193/management".to_owned();
    management_consumer.api_key = Some(ApiKey {
        prefix: Some("x-api-key".to_string()),
        key: "123456".to_owned(),
    });
    management_consumer.with_headers()
}

pub async fn setup_dsp_consumer_configuration() -> dsp_client::configuration::Configuration {
    let mut dsp_consumer = dsp_client::configuration::Configuration::default();
    dsp_consumer.base_path = "http://localhost:9194/protocol".to_owned();
    dsp_consumer.api_key = Some(dsp_client::configuration::ApiKey {
        prefix: Some("x-api-key".to_string()),
        key: "123456".to_owned(),
    });
    dsp_consumer.with_headers()
}

pub async fn setup_dsp_provider_configuration() -> dsp_client::configuration::Configuration {
    let mut provider = dsp_client::configuration::Configuration::default();
    provider.base_path = "http://localhost:3000".to_string();
    provider.api_key = Some(dsp_client::configuration::ApiKey {
        prefix: Some("x-api-key".to_string()),
        key: "123456".to_owned(),
    });
    provider.with_headers()
}
