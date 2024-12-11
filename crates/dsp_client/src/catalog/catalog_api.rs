use crate::ResponseContent;
use crate::{configuration, Error};
use reqwest;
use reqwest::header::AUTHORIZATION;

/// struct for typed errors of method [`request_catalog`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RequestCatalogError {
    Status400(dsp_api::catalog::CatalogError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_dataset`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetDatasetError {
    Status400(dsp_api::catalog::CatalogError),
    Status404(dsp_api::catalog::CatalogError),
    UnknownValue(serde_json::Value),
}

/// If the request is successful, the Catalog Service must return an HTTP 200 (OK) with a response body containing
/// a Catalog (which is a profiled DCAT Catalog type described by the Catalog Protocol).
pub async fn request_catalog(
    configuration: &configuration::Configuration,
    request_message: dsp_models::catalog::CatalogRequestMessage,
) -> Result<dsp_api::catalog::Catalog, Error<RequestCatalogError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/catalog/request", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&request_message);

    local_var_req_builder = local_var_req_builder.header(
        AUTHORIZATION,
        "{\"region\": \"any\", \"audience\": \"any\", \"clientId\": \"any\"}",
    );

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let val = serde_json::from_str(&local_var_content).map_err(Error::from)?;
        Ok(val)
    } else {
        let local_var_entity: Option<RequestCatalogError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// If the request is successful, the Catalog Service must return an HTTP 200 (OK) with a response body containing
/// a Dataset (which is a DCAT Dataset type described by the Catalog Protocol).
pub async fn get_dataset(
    configuration: &configuration::Configuration,
    id: &str,
) -> Result<dsp_api::catalog::Dataset, Error<GetDatasetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/catalog/datasets/{id}",
        local_var_configuration.base_path,
        id = crate::urlencode(id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    local_var_req_builder = local_var_req_builder.header(
        AUTHORIZATION,
        "{\"region\": \"any\", \"audience\": \"any\", \"clientId\": \"any\"}",
    );

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let val = serde_json::from_str(&local_var_content).map_err(Error::from)?;
        Ok(val)
    } else {
        let local_var_entity: Option<GetDatasetError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
