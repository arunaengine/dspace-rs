/*
 * management-api
 *
 * REST API documentation for the Eclipse EDC management-api.
 * https://app.swaggerhub.com/apis/eclipse-edc-bot/management-api/
 * Version: 0.7.0
 *
 */

use reqwest;

use super::{configuration, Error};
use crate::utils::remove_prefixes_from_value;
use crate::ResponseContent;

/// struct for typed errors of method [`create_asset`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateAssetError {
    Status400(Vec<edc_api::ApiErrorDetail>),
    Status409(Vec<edc_api::ApiErrorDetail>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_asset`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAssetError {
    Status400(Vec<edc_api::ApiErrorDetail>),
    Status404(Vec<edc_api::ApiErrorDetail>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`remove_asset`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RemoveAssetError {
    Status400(Vec<edc_api::ApiErrorDetail>),
    Status404(Vec<edc_api::ApiErrorDetail>),
    Status409(Vec<edc_api::ApiErrorDetail>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`request_assets`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RequestAssetsError {
    Status400(Vec<edc_api::ApiErrorDetail>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_asset`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateAssetError {
    Status400(Vec<edc_api::ApiErrorDetail>),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// Creates a new asset together with a data address
pub async fn create_asset(
    configuration: &configuration::Configuration,
    asset_entry: Option<edc_api::AssetInput>,
) -> Result<edc_api::IdResponse, Error<CreateAssetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v3/assets", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&asset_entry);

    println!("Request: {:#?}", local_var_req_builder);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let mut val = serde_json::from_str(&local_var_content).map_err(Error::from)?;
        val = remove_prefixes_from_value(val);
        serde_json::from_value(val).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateAssetError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Gets an asset with the given ID
pub async fn get_asset(
    configuration: &configuration::Configuration,
    id: &str,
) -> Result<edc_api::AssetOutput, Error<GetAssetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v3/assets/{id}",
        local_var_configuration.base_path,
        id = crate::urlencode(id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    println!("\nRequest: {:#?}\n", local_var_req_builder);

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;

    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let mut val = serde_json::from_str(&local_var_content).map_err(Error::from)?;
        val = remove_prefixes_from_value(val);
        serde_json::from_value(val).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetAssetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Removes an asset with the given ID if possible. Deleting an asset is only possible if that asset is not yet referenced by a contract agreement, in which case an error is returned. DANGER ZONE: Note that deleting assets can have unexpected results, especially for contract offers that have been sent out or ongoing or contract negotiations.
pub async fn remove_asset(
    configuration: &configuration::Configuration,
    id: &str,
) -> Result<(), Error<RemoveAssetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v3/assets/{id}",
        local_var_configuration.base_path,
        id = crate::urlencode(id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    println!("Request: {:#?}", local_var_req_builder);

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<RemoveAssetError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Request all assets according to a particular query
pub async fn request_assets(
    configuration: &configuration::Configuration,
    query_spec: Option<edc_api::QuerySpec>,
) -> Result<Vec<edc_api::AssetOutput>, Error<RequestAssetsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v3/assets/request", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&query_spec);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let mut val = serde_json::from_str(&local_var_content).map_err(Error::from)?;
        val = remove_prefixes_from_value(val);
        serde_json::from_value(val).map_err(Error::from)
    } else {
        let local_var_entity: Option<RequestAssetsError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Updates an asset with the given ID if it exists. If the asset is not found, no further action is taken. DANGER ZONE: Note that updating assets can have unexpected results, especially for contract offers that have been sent out or are ongoing in contract negotiations.
pub async fn update_asset(
    configuration: &configuration::Configuration,
    asset: Option<edc_api::AssetInput>,
) -> Result<(), Error<UpdateAssetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v3/assets", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&asset);

    println!("Request: {:#?}", local_var_req_builder);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<UpdateAssetError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
