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

/// struct for typed errors of method [`deprovision_transfer_process`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeprovisionTransferProcessError {
    Status400(Vec<edc_api::ApiErrorDetail>),
    Status404(Vec<edc_api::ApiErrorDetail>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`resume_transfer_process`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ResumeTransferProcessError {
    Status400(Vec<edc_api::ApiErrorDetail>),
    Status404(Vec<edc_api::ApiErrorDetail>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`suspend_transfer_process`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SuspendTransferProcessError {
    Status400(Vec<edc_api::ApiErrorDetail>),
    Status404(Vec<edc_api::ApiErrorDetail>),
    Status409(Vec<edc_api::ApiErrorDetail>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_transfer_process`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTransferProcessError {
    Status400(Vec<edc_api::ApiErrorDetail>),
    Status404(Vec<edc_api::ApiErrorDetail>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_transfer_process_state`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTransferProcessStateError {
    Status400(Vec<edc_api::ApiErrorDetail>),
    Status404(Vec<edc_api::ApiErrorDetail>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`initiate_transfer_process`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InitiateTransferProcessError {
    Status400(Vec<edc_api::ApiErrorDetail>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`query_transfer_processes`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum QueryTransferProcessesError {
    Status400(Vec<edc_api::ApiErrorDetail>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`terminate_transfer_process`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TerminateTransferProcessError {
    Status400(Vec<edc_api::ApiErrorDetail>),
    Status404(Vec<edc_api::ApiErrorDetail>),
    Status409(Vec<edc_api::ApiErrorDetail>),
    UnknownValue(serde_json::Value),
}

/// Requests the deprovisioning of resources associated with a transfer process. Due to the asynchronous nature of transfers, a successful response only indicates that the request was successfully received. This may take a long time, so clients must poll the /{id}/state endpoint to track the state.
pub async fn deprovision_transfer_process(
    configuration: &configuration::Configuration,
    id: &str,
) -> Result<(), Error<DeprovisionTransferProcessError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v2/transferprocesses/{id}/deprovision",
        local_var_configuration.base_path,
        id = crate::urlencode(id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

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
        let local_var_entity: Option<DeprovisionTransferProcessError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Requests the resumption of a suspended transfer process. Due to the asynchronous nature of transfers, a successful response only indicates that the request was successfully received. This may take a long time, so clients must poll the /{id}/state endpoint to track the state.
pub async fn resume_transfer_process(
    configuration: &configuration::Configuration,
    id: &str,
) -> Result<(), Error<ResumeTransferProcessError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v2/transferprocesses/{id}/resume",
        local_var_configuration.base_path,
        id = crate::urlencode(id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

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
        let local_var_entity: Option<ResumeTransferProcessError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Requests the suspension of a transfer process. Due to the asynchronous nature of transfers, a successful response only indicates that the request was successfully received. This may take a long time, so clients must poll the /{id}/state endpoint to track the state.
pub async fn suspend_transfer_process(
    configuration: &configuration::Configuration,
    id: &str,
    suspend_transfer: Option<edc_api::SuspendTransfer>,
) -> Result<(), Error<SuspendTransferProcessError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v2/transferprocesses/{id}/suspend",
        local_var_configuration.base_path,
        id = crate::urlencode(id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&suspend_transfer);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<SuspendTransferProcessError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Gets an transfer process with the given ID
pub async fn get_transfer_process(
    configuration: &configuration::Configuration,
    id: &str,
) -> Result<edc_api::TransferProcess, Error<GetTransferProcessError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v2/transferprocesses/{id}",
        local_var_configuration.base_path,
        id = crate::urlencode(id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

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
        let local_var_entity: Option<GetTransferProcessError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Gets the state of a transfer process with the given ID
pub async fn get_transfer_process_state(
    configuration: &configuration::Configuration,
    id: &str,
) -> Result<edc_api::TransferState, Error<GetTransferProcessStateError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v2/transferprocesses/{id}/state",
        local_var_configuration.base_path,
        id = crate::urlencode(id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

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
        let local_var_entity: Option<GetTransferProcessStateError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Initiates a data transfer with the given parameters. Due to the asynchronous nature of transfers, a successful response only indicates that the request was successfully received. This may take a long time, so clients must poll the /{id}/state endpoint to track the state.
pub async fn initiate_transfer_process(
    configuration: &configuration::Configuration,
    transfer_request: Option<edc_api::TransferRequest>,
) -> Result<edc_api::IdResponse, Error<InitiateTransferProcessError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/transferprocesses", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&transfer_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let mut val = serde_json::from_str(&local_var_content).map_err(Error::from)?;
        val = remove_prefixes_from_value(val);
        serde_json::from_value(val).map_err(Error::from)
    } else {
        let local_var_entity: Option<InitiateTransferProcessError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns all transfer process according to a query
pub async fn query_transfer_processes(
    configuration: &configuration::Configuration,
    query_spec: Option<edc_api::QuerySpec>,
) -> Result<Vec<edc_api::TransferProcess>, Error<QueryTransferProcessesError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v2/transferprocesses/request",
        local_var_configuration.base_path
    );
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
        let local_var_entity: Option<QueryTransferProcessesError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Requests the termination of a transfer process. Due to the asynchronous nature of transfers, a successful response only indicates that the request was successfully received. This may take a long time, so clients must poll the /{id}/state endpoint to track the state.
pub async fn terminate_transfer_process(
    configuration: &configuration::Configuration,
    id: &str,
    terminate_transfer: Option<edc_api::TerminateTransfer>,
) -> Result<(), Error<TerminateTransferProcessError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v2/transferprocesses/{id}/terminate",
        local_var_configuration.base_path,
        id = crate::urlencode(id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&terminate_transfer);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<TerminateTransferProcessError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
