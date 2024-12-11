use crate::ResponseContent;
use crate::{configuration, Error};
use dsp_models::transfer_process::{
    TransferCompletionMessage, TransferProcess, TransferRequestMessage, TransferStartMessage,
    TransferSuspendMessage, TransferTerminationMessage,
};
use reqwest;

/// struct for typed errors of method [`get_negotiation`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetNegotiationError {
    Status400(dsp_api::transfer_process::TransferError),
    Status404(dsp_api::transfer_process::TransferError), // must be used for "object not found" and "unauthorized access"
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`request_transfer`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RequestTransferError {
    Status400(dsp_api::transfer_process::TransferError),
    Status404(dsp_api::transfer_process::TransferError),
    Status409(dsp_api::transfer_process::TransferError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`restart_transfer`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RestartTransferError {
    Status400(dsp_api::transfer_process::TransferError),
    Status404(dsp_api::transfer_process::TransferError),
    Status409(dsp_api::transfer_process::TransferError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`complete_transfer`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProviderSideCompleteTransferError {
    Status400(dsp_api::transfer_process::TransferError),
    Status404(dsp_api::transfer_process::TransferError),
    Status409(dsp_api::transfer_process::TransferError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`terminate_transfer`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProviderSideTerminateTransferError {
    Status400(dsp_api::transfer_process::TransferError),
    Status404(dsp_api::transfer_process::TransferError),
    Status409(dsp_api::transfer_process::TransferError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`suspend_transfer`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProviderSideSuspendTransferError {
    Status400(dsp_api::transfer_process::TransferError),
    Status404(dsp_api::transfer_process::TransferError),
    Status409(dsp_api::transfer_process::TransferError),
    UnknownValue(serde_json::Value),
}

/// A CN can be accessed by a Consumer or Provider sending a GET request to transfers/:providerPid
/// If the TP is found and the client is authorized, the Provider must return an HTTP 200 (OK) response and a body containing the Transfer Process
pub async fn get_negotiation(
    configuration: &configuration::Configuration,
    provider_pid: &str,
) -> Result<TransferProcess, Error<GetNegotiationError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/transfers/{providerPid}",
        local_var_configuration.base_path,
        providerPid = crate::urlencode(provider_pid)
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
        let val = serde_json::from_str(&local_var_content).map_err(Error::from)?;
        Ok(val)
    } else {
        let local_var_entity: Option<GetNegotiationError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// A TP is started and placed in the REQUESTED state when a Consumer POSTs a Transfer Request Message to transfers/request
/// The Provider must return an HTTP 201 (Created) response with a body containing the Transfer Process
pub async fn request_transfer(
    configuration: &configuration::Configuration,
    transfer_request_message: TransferRequestMessage,
) -> Result<TransferProcess, Error<RequestTransferError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/transfers/request", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    local_var_req_builder = local_var_req_builder.json(&transfer_request_message);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let val = serde_json::from_str(&local_var_content).map_err(Error::from)?;
        Ok(val)
    } else {
        let local_var_entity: Option<RequestTransferError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

// TODO: Check if this should be a transfer request message instead of a transfer start message
/// The Consumer can POST a Transfer Start Message to attempt to start a TP after it has been suspended
/// If the TPs state is successfully transitioned, the Provider must return HTTP code 200 (OK). The response body is not specified and clients are not required to process it.
pub async fn restart_transfer(
    configuration: &configuration::Configuration,
    transfer_start_message: TransferStartMessage,
    provider_pid: &str,
) -> Result<(), Error<RestartTransferError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/transfers/{providerPid}/start",
        local_var_configuration.base_path,
        providerPid = crate::urlencode(provider_pid)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    local_var_req_builder = local_var_req_builder.json(&transfer_start_message);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<RestartTransferError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// The Consumer can POST a Transfer Completion Message to complete a TP
/// If the TPs state is successfully transitioned, the Provider must return HTTP code 200 (OK). The response body is not specified and clients are not required to process it.
pub async fn complete_transfer(
    configuration: &configuration::Configuration,
    completion_message: TransferCompletionMessage,
    provider_pid: &str,
) -> Result<(), Error<ProviderSideCompleteTransferError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/transfers/{providerPid}/completion",
        local_var_configuration.base_path,
        providerPid = crate::urlencode(provider_pid)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    local_var_req_builder = local_var_req_builder.json(&completion_message);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<ProviderSideCompleteTransferError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// The Consumer can POST a Transfer Termination Message to terminate a TP
/// If the TPs state is successfully transitioned, the Provider must return HTTP code 200 (OK). The response body is not specified and clients are not required to process it.
pub async fn terminate_transfer(
    configuration: &configuration::Configuration,
    termination_message: TransferTerminationMessage,
    provider_pid: &str,
) -> Result<(), Error<ProviderSideTerminateTransferError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/transfers/{providerPid}/termination",
        local_var_configuration.base_path,
        providerPid = crate::urlencode(provider_pid)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    local_var_req_builder = local_var_req_builder.json(&termination_message);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<ProviderSideTerminateTransferError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// The Consumer can POST a Transfer Suspension Message to suspend a TP
/// If the TPs state is successfully transitioned, the Provider must return HTTP code 200 (OK). The response body is not specified and clients are not required to process it.
pub async fn suspend_transfer(
    configuration: &configuration::Configuration,
    suspension_message: TransferSuspendMessage,
    provider_pid: &str,
) -> Result<(), Error<ProviderSideSuspendTransferError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/transfers/{providerPid}/suspension",
        local_var_configuration.base_path,
        providerPid = crate::urlencode(provider_pid)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    local_var_req_builder = local_var_req_builder.json(&suspension_message);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<ProviderSideSuspendTransferError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
