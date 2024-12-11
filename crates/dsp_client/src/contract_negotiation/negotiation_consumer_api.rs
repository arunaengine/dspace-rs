use crate::ResponseContent;
use crate::{configuration, Error};
use dsp_models::contract_negotiation::{
    ContractAgreementMessage, ContractNegotiation, ContractNegotiationEventMessage,
    ContractNegotiationTerminationMessage, ContractOfferMessage,
};
use reqwest;

/// All callback paths are relative to the callbackAddress base URL specified in the Contract Request Message that initiated a CN.
/// For example, if the callbackAddress is specified as https://consumer.com/callback and a callback path binding is negotiations/:consumerPid/offers,
/// the resolved URL will be https://consumer.com/callback/negotiations/:consumerPid/offers.

/// struct for typed errors of method [`offer_negotiation`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OfferNegotiationError {
    Status400(dsp_api::contract_negotiation::ContractNegotiationError),
    Status405(dsp_api::contract_negotiation::ContractNegotiationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`counter_offer_negotiation`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CounterOfferError {
    Status400(dsp_api::contract_negotiation::ContractNegotiationError),
    Status405(dsp_api::contract_negotiation::ContractNegotiationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`agree_negotiation`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AgreeNegotiationError {
    Status400(dsp_api::contract_negotiation::ContractNegotiationError),
    Status405(dsp_api::contract_negotiation::ContractNegotiationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`finalize_negotiation`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FinalizeNegotiationError {
    Status400(dsp_api::contract_negotiation::ContractNegotiationError),
    Status405(dsp_api::contract_negotiation::ContractNegotiationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`terminate_negotiation`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TerminateNegotiationError {
    Status400(dsp_api::contract_negotiation::ContractNegotiationError),
    Status405(dsp_api::contract_negotiation::ContractNegotiationError),
    UnknownValue(serde_json::Value),
}

/// A CN is started and placed in the OFFERED state when a Provider POSTs a Contract Offer Message to negotiations/offers
/// The Consumer must return an HTTP 201 (Created) response with a body containing the Contract Negotiation
pub async fn offer_negotiation(
    configuration: &configuration::Configuration,
    cb_address: &str,
    offer_message: ContractOfferMessage,
) -> Result<ContractNegotiation, Error<OfferNegotiationError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/negotiations/offers", cb_address);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&offer_message);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let val = serde_json::from_str(&local_var_content).map_err(Error::from)?;
        Ok(val)
    } else {
        let local_var_entity: Option<OfferNegotiationError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// A Provider may make an Offer by POSTing a Contract Offer Message to the negotiations/:consumerPid/offers callback
/// If the message is successfully processed, the Consumer must return an HTTP 200 (OK) response.
/// The response body is not specified and clients are not required to process it.
pub async fn counter_offer_negotiation(
    configuration: &configuration::Configuration,
    cb_address: &str,
    offer_message: ContractOfferMessage,
    consumer_pid: &str,
) -> Result<(), Error<CounterOfferError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/negotiations/{consumerPid}/offers",
        cb_address,
        consumerPid = crate::urlencode(consumer_pid)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&offer_message);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<CounterOfferError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// The Provider can POST a Contract Agreement Message to the negotiations/:consumerPid/agreement callback to create an Agreement.
/// If the CN's state is successfully transitioned, the Consumer must return an HTTP code 200 (OK).
/// The response body is not specified and clients are not required to process it.
pub async fn agree_negotiation(
    configuration: &configuration::Configuration,
    cb_address: &str,
    agreement_message: ContractAgreementMessage,
    consumer_pid: &str,
) -> Result<(), Error<AgreeNegotiationError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/negotiations/{consumerPid}/agreement",
        cb_address,
        consumerPid = crate::urlencode(consumer_pid)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&agreement_message);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<AgreeNegotiationError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// A Provider can POST a Contract Negotiation Event Message to the negotiations/:consumerPid/events callback with an eventType of FINALIZED to finalize an Agreement.
/// If the CN's state is successfully transitioned, the Consumer must return HTTP code 200 (OK).
/// The response body is not specified and clients are not required to process it.
pub async fn finalize_negotiation(
    configuration: &configuration::Configuration,
    cb_address: &str,
    negotiation_event_message: ContractNegotiationEventMessage,
    consumer_pid: &str,
) -> Result<(), Error<FinalizeNegotiationError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/negotiations/{consumerPid}/events",
        cb_address,
        consumerPid = crate::urlencode(consumer_pid)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&negotiation_event_message);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<FinalizeNegotiationError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// The Provider can POST a Contract Negotiation Termination Message to terminate a CN.
/// If the CN's state is successfully transitioned, the Consumer must return HTTP code 200 (OK).
/// The response body is not specified and clients are not required to process it.
pub async fn terminate_negotiation(
    configuration: &configuration::Configuration,
    cb_address: &str,
    terminate_message: ContractNegotiationTerminationMessage,
    consumer_pid: &str,
) -> Result<(), Error<TerminateNegotiationError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/negotiations/{consumerPid}/termination",
        cb_address,
        consumerPid = crate::urlencode(consumer_pid)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&terminate_message);

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
