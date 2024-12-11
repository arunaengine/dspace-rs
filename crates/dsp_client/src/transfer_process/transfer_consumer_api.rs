use crate::ResponseContent;
use crate::{configuration, Error};
use dsp_models::transfer_process::{
    TransferCompletionMessage, TransferStartMessage, TransferSuspendMessage,
    TransferTerminationMessage,
};
use reqwest;

/// struct for typed errors of method [`start_transfer`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StartTransferError {
    Status400(dsp_api::transfer_process::TransferError),
    Status404(dsp_api::transfer_process::TransferError),
    Status409(dsp_api::transfer_process::TransferError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`complete_transfer`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ConsumerSideCompleteTransferError {
    Status400(dsp_api::transfer_process::TransferError),
    Status404(dsp_api::transfer_process::TransferError),
    Status409(dsp_api::transfer_process::TransferError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`terminate_transfer`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ConsumerSideTerminateTransferError {
    Status400(dsp_api::transfer_process::TransferError),
    Status404(dsp_api::transfer_process::TransferError),
    Status409(dsp_api::transfer_process::TransferError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`suspend_transfer`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ConsumerSideSuspendTransferError {
    Status400(dsp_api::transfer_process::TransferError),
    Status404(dsp_api::transfer_process::TransferError),
    Status409(dsp_api::transfer_process::TransferError),
    UnknownValue(serde_json::Value),
}

/// The Provider can POST a Transfer Start Message to indicate the start of a TP
/// If the TPs state is successfully transitioned, the Consumer must return HTTP code 200 (OK). The response body is not specified and clients are not required to process it.
pub async fn start_transfer(
    configuration: &configuration::Configuration,
    start_message: TransferStartMessage,
    consumer_pid: &str,
) -> Result<(), Error<StartTransferError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/transfers/{consumerPid}/start",
        local_var_configuration.base_path,
        consumerPid = crate::urlencode(consumer_pid)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    local_var_req_builder = local_var_req_builder.json(&start_message);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<StartTransferError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// The Provider can POST a Transfer Completion Message to complete a TP
/// If the TPs state is successfully transitioned, the Consumer must return HTTP code 200 (OK). The response body is not specified and clients are not required to process it.
pub async fn complete_transfer(
    configuration: &configuration::Configuration,
    completion_message: TransferCompletionMessage,
    consumer_pid: &str,
) -> Result<(), Error<ConsumerSideCompleteTransferError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/transfers/{consumerPid}/completion",
        local_var_configuration.base_path,
        consumerPid = crate::urlencode(consumer_pid)
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
        let local_var_entity: Option<ConsumerSideCompleteTransferError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// The Provider can POST a Transfer Termination Message to terminate a TP
/// If the TPs state is successfully transitioned, the Consumer must return HTTP code 200 (OK). The response body is not specified and clients are not required to process it.
pub async fn terminate_transfer(
    configuration: &configuration::Configuration,
    termination_message: TransferTerminationMessage,
    consumer_pid: &str,
) -> Result<(), Error<ConsumerSideTerminateTransferError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/transfers/{consumerPid}/termination",
        local_var_configuration.base_path,
        consumerPid = crate::urlencode(consumer_pid)
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
        let local_var_entity: Option<ConsumerSideTerminateTransferError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// The Provider can POST a Transfer Suspension Message to suspend a TP
/// If the TPs state is successfully transitioned, the Consumer must return HTTP code 200 (OK). The response body is not specified and clients are not required to process it.
pub async fn suspend_transfer(
    configuration: &configuration::Configuration,
    suspension_message: TransferSuspendMessage,
    consumer_pid: &str,
) -> Result<(), Error<ConsumerSideSuspendTransferError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/transfers/{consumerPid}/suspension",
        local_var_configuration.base_path,
        consumerPid = crate::urlencode(consumer_pid)
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
        let local_var_entity: Option<ConsumerSideSuspendTransferError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
