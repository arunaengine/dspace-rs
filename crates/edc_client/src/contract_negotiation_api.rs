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

/// struct for typed errors of method [`get_agreement_for_negotiation`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAgreementForNegotiationError {
    Status400(Vec<edc_api::ApiErrorDetail>),
    Status404(Vec<edc_api::ApiErrorDetail>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_negotiation`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetNegotiationError {
    Status400(Vec<edc_api::ApiErrorDetail>),
    Status404(Vec<edc_api::ApiErrorDetail>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_negotiation_state`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetNegotiationStateError {
    Status400(Vec<edc_api::ApiErrorDetail>),
    Status404(Vec<edc_api::ApiErrorDetail>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`initiate_contract_negotiation`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InitiateContractNegotiationError {
    Status400(Vec<edc_api::ApiErrorDetail>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`query_negotiations`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum QueryNegotiationsError {
    Status400(Vec<edc_api::ApiErrorDetail>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`terminate_negotiation`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TerminateNegotiationError {
    Status400(Vec<edc_api::ApiErrorDetail>),
    Status404(Vec<edc_api::ApiErrorDetail>),
    UnknownValue(serde_json::Value),
}

/// Gets a contract agreement for a contract negotiation with the given ID
pub async fn get_agreement_for_negotiation(
    configuration: &configuration::Configuration,
    id: &str,
) -> Result<edc_api::ContractAgreement, Error<GetAgreementForNegotiationError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v2/contractnegotiations/{id}/agreement",
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
        let local_var_entity: Option<GetAgreementForNegotiationError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Gets a contract negotiation with the given ID
pub async fn get_negotiation(
    configuration: &configuration::Configuration,
    id: &str,
) -> Result<edc_api::ContractNegotiation, Error<GetNegotiationError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v2/contractnegotiations/{id}",
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

/// Gets the state of a contract negotiation with the given ID
pub async fn get_negotiation_state(
    configuration: &configuration::Configuration,
    id: &str,
) -> Result<edc_api::NegotiationState, Error<GetNegotiationStateError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v2/contractnegotiations/{id}/state",
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
        let local_var_entity: Option<GetNegotiationStateError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Initiates a contract negotiation for a given offer and with the given counter part. Please note that successfully invoking this endpoint only means that the negotiation was initiated. Clients must poll the /{id}/state endpoint to track the state
pub async fn initiate_contract_negotiation(
    configuration: &configuration::Configuration,
    contract_request: Option<edc_api::ContractRequest>,
) -> Result<edc_api::IdResponse, Error<InitiateContractNegotiationError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v2/contractnegotiations",
        local_var_configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&contract_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let mut val = serde_json::from_str(&local_var_content).map_err(Error::from)?;
        val = remove_prefixes_from_value(val);
        serde_json::from_value(val).map_err(Error::from)
    } else {
        let local_var_entity: Option<InitiateContractNegotiationError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns all contract negotiations according to a query
pub async fn query_negotiations(
    configuration: &configuration::Configuration,
    query_spec: Option<edc_api::QuerySpec>,
) -> Result<Vec<edc_api::ContractNegotiation>, Error<QueryNegotiationsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v2/contractnegotiations/request",
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
        let local_var_entity: Option<QueryNegotiationsError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Terminates the contract negotiation.
pub async fn terminate_negotiation(
    configuration: &configuration::Configuration,
    id: &str,
    terminate_negotiation_schema: Option<edc_api::TerminateNegotiationSchema>,
) -> Result<(), Error<TerminateNegotiationError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v2/contractnegotiations/{id}/terminate",
        local_var_configuration.base_path,
        id = crate::urlencode(id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&terminate_negotiation_schema);

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
