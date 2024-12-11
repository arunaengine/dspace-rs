use crate::ResponseContent;
use crate::{configuration, Error};
use dsp_models::contract_negotiation::{
    ContractAgreementVerificationMessage, ContractNegotiationEventMessage,
    ContractNegotiationTerminationMessage, ContractRequestMessage,
};
use reqwest;

/// struct for typed errors of method [`get_negotiation`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetNegotiationError {
    Status400(dsp_api::contract_negotiation::ContractNegotiationError),
    Status404(dsp_api::contract_negotiation::ContractNegotiationError), // must be used for "object not found" and "unauthorized access"
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`request_negotiation`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RequestNegotiationError {
    Status400(dsp_api::contract_negotiation::ContractNegotiationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`make_offer`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MakeOfferError {
    Status400(dsp_api::contract_negotiation::ContractNegotiationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`accept_offer`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AcceptOfferError {
    Status400(dsp_api::contract_negotiation::ContractNegotiationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`verify_agreement`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum VerifyAgreementError {
    Status400(dsp_api::contract_negotiation::ContractNegotiationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`terminate_negotiation`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TerminateNegotiationError {
    Status400(dsp_api::contract_negotiation::ContractNegotiationError),
    UnknownValue(serde_json::Value),
}

/// A CN can be accessed by a Consumer or Provider sending a GET request to negotiation endpoint on the provider-side.
/// If the CN is found and the client is authorized, the Provider must return an HTTP 200 (OK) response and a body containing the Contract Negotiation
pub async fn get_negotiation(
    configuration: &configuration::Configuration,
    provider_pid: &str,
) -> Result<dsp_api::contract_negotiation::ContractNegotiation, Error<GetNegotiationError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/negotiations/{providerPid}",
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

/// A CN is started and placed in the REQUESTED state when a Consumer POSTs an initiating Contract Request Message to negotiations/request
/// The Provider must return an HTTP 201 (Created) response with a body containing the Contract Negotiation:
pub async fn request_negotiation(
    configuration: &configuration::Configuration,
    request_message: ContractRequestMessage,
) -> Result<dsp_api::contract_negotiation::ContractNegotiation, Error<RequestNegotiationError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/negotiations/request", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&request_message);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let val = serde_json::from_str(&local_var_content).map_err(Error::from)?;
        Ok(val)
    } else {
        let local_var_entity: Option<RequestNegotiationError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// A Consumer may make an Offer by POSTing a Contract Request Message to negotiations/:providerPid/request
/// If the message is successfully processed, the Provider must return an HTTP 200 (OK) response.
/// The response body is not specified and clients are not required to process it.
pub async fn make_offer(
    configuration: &configuration::Configuration,
    request_message: ContractRequestMessage,
    provider_pid: &str,
) -> Result<(), Error<MakeOfferError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/negotiations/{providerPid}/request",
        local_var_configuration.base_path,
        providerPid = crate::urlencode(provider_pid)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&request_message);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<MakeOfferError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// A Consumer can POST a Contract Negotiation Event Message to negotiations/:providerPid/events to accept the current Provider's Offer.
/// If the CN's state is successfully transitioned, the Provider must return an HTTP code 200 (OK).
/// The response body is not specified and clients are not required to process it.
/// If the current Offer was created by the Consumer, the Provider must return an HTTP code 400 (Bad Request) with a Contract Negotiation Error in the response body.
pub async fn accept_offer(
    configuration: &configuration::Configuration,
    event_message: ContractNegotiationEventMessage,
    provider_pid: &str,
) -> Result<(), Error<AcceptOfferError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/negotiations/{providerPid}/events",
        local_var_configuration.base_path,
        providerPid = crate::urlencode(provider_pid)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&event_message);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<AcceptOfferError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// The Consumer can POST a Contract Agreement Verification Message to verify an Agreement.
/// If the CN's state is successfully transitioned, the Provider must return an HTTP code 200 (OK). The response body is not specified and clients are not required to process it.
pub async fn verify_agreement(
    configuration: &configuration::Configuration,
    verification_message: ContractAgreementVerificationMessage,
    provider_pid: &str,
) -> Result<(), Error<VerifyAgreementError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/negotiations/{providerPid}/agreement/verification",
        local_var_configuration.base_path,
        providerPid = crate::urlencode(provider_pid)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&verification_message);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<VerifyAgreementError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// The Consumer can POST a Contract Negotiation Termination Message to terminate a CN.
/// If the CN's state is successfully transitioned, the Provider must return HTTP code 200 (OK). The response body is not specified and clients are not required to process it.
pub async fn terminate_negotiation(
    configuration: &configuration::Configuration,
    termination_message: ContractNegotiationTerminationMessage,
    provider_pid: &str,
) -> Result<(), Error<TerminateNegotiationError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/negotiations/{providerPid}/termination",
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
        let local_var_entity: Option<TerminateNegotiationError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
