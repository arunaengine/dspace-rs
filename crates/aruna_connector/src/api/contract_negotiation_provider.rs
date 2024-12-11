use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::Json;
use dsp_api::contract_negotiation::contract_negotiation::NegotiationState;
use dsp_api::contract_negotiation::{
    AbstractPolicyRule, Action, ContractNegotiation, ContractNegotiationTerminationMessage,
    ContractOfferMessage, MessageOffer, Permission, PolicyClass, Target,
};
use dsp_client::configuration::Configuration;
use odrl::functions::state_machine::{ConsumerStateMachine, ProviderState, ProviderStateMachine};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::sync::MutexGuard;
use tracing::{debug, error, info};
use uuid::Uuid;

// SharedState:
//     Key: PID of the contract negotiation
//     Value: Tuple of ContractNegotiation and ProviderStateMachine<ProviderState>, to keep track of the state of the negotiation
type SharedState =
    Arc<Mutex<HashMap<String, (ContractNegotiation, ProviderStateMachine<ProviderState>)>>>;

pub async fn get_negotiation(
    State(state): State<SharedState>,
    Path(pid): Path<String>,
) -> impl IntoResponse {
    info!("[DSP] Get Negotiation called");
    debug!(
        "[DSP] Received Negotiation request for pid {:#?}",
        pid.clone()
    );

    let state = state.lock().await;

    match state.get(&pid) {
        Some((negotiation, _)) => (StatusCode::OK, Json(negotiation.clone())).into_response(),
        None => {
            let err_msg = format!(
                "A Contract Negotiation with the given pid {:#?} does not exist.",
                pid.clone()
            );
            error!("{}", err_msg);
            (StatusCode::NOT_FOUND, Json(err_msg)).into_response()
        }
    }
}

pub async fn request_negotiation(
    headers: HeaderMap,
    State(state): State<SharedState>,
    Json(request_message): Json<Value>,
) -> impl IntoResponse {
    info!("[DSP] Request Contract Negotiation called");
    debug!("[DSP] Request Body: {:#?}", request_message.clone());

    let mut state = state.lock().await;

    if let Some(provider_pid) = request_message["provider_pid"].as_str() {
        // TODO: Implement response to a Offer sent by the provider (provider pid provided)
    } else {
        let host = headers["host"].to_str().unwrap();

        if request_message["dspace:callbackAddress"].is_null() {
            let err_msg = "The callbackAddress is required and should be a URL indicating where messages to the Consumer should be sent in asynchronous settings.".to_string();
            error!("{}", err_msg);
            return (StatusCode::UNPROCESSABLE_ENTITY, Json(err_msg)).into_response();
        }

        let mut fsm = ProviderStateMachine::new(
            host.clone(),
            request_message["dspace:callbackAddress"].as_str().unwrap(),
        );

        debug!("[DSP] Initialized new State Machine; State: {}", fsm.state);

        let pid = Uuid::new_v4().to_string();

        let default_context = HashMap::from([
            (
                "@vocab".to_string(),
                Value::String("https://w3id.org/edc/v0.0.1/ns/".to_string()),
            ),
            (
                "edc".to_string(),
                Value::String("https://w3id.org/edc/v0.0.1/ns/".to_string()),
            ),
            (
                "dcat".to_string(),
                Value::String("http://www.w3.org/ns/dcat#".to_string()),
            ),
            (
                "dct".to_string(),
                Value::String("http://purl.org/dc/terms/".to_string()),
            ),
            (
                "odrl".to_string(),
                Value::String("http://www.w3.org/ns/odrl/2/".to_string()),
            ),
            (
                "dspace".to_string(),
                Value::String("https://w3id.org/dspace/v0.8/".to_string()),
            ),
        ]);

        let negotiation = ContractNegotiation {
            context: default_context,
            dsp_type: "dspace:ContractNegotiation".to_string(),
            consumer_pid: request_message["dspace:consumerPid"]
                .as_str()
                .unwrap()
                .to_string(),
            provider_pid: pid.clone(),
            state: NegotiationState::REQUESTED,
        };

        debug!(
            "[DSP] Contract Negotiation Provider PID: {:#?}",
            pid.clone()
        );

        let transition_message = format!(
            "Requesting Contract Negotiation from {:#?}",
            request_message["dspace:callbackAddress"].as_str().unwrap()
        );
        fsm.receive_contract_request(transition_message);

        debug!("[DSP] State Machine transitioned to: {}", fsm.state);
        println!("[DSP] FSM: {:#?}", fsm.clone()); // only for demonstration purposes, remove later

        state.insert(pid.clone(), (negotiation.clone(), fsm.clone()));

        return (StatusCode::CREATED, Json(negotiation.clone())).into_response();
    }

    debug!("[DSP] I'm a Teapot!");

    (StatusCode::IM_A_TEAPOT, Json("I'm a Teapot".clone())).into_response()
}

pub async fn make_offer() -> impl IntoResponse {
    unimplemented!()
}

pub async fn accept_offer() -> impl IntoResponse {
    unimplemented!()
}

pub async fn verify_agreement() -> impl IntoResponse {
    unimplemented!()
}

pub async fn terminate_negotiation(
    headers: HeaderMap,
    State(state): State<SharedState>,
    Path(pid): Path<String>,
    Json(termination_request): Json<Value>,
) -> impl IntoResponse {
    info!("[DSP] Terminate Negotiation called");
    debug!(
        "[DSP] Received Negotiation termination request for pid {:#?}",
        pid.clone()
    );

    let mut state = state.lock().await;

    match state.get(&pid) {
        Some((negotiation, _)) => {
            let reason = termination_request["dspace:reason"].clone();
            debug!(
                "[DSP] Received Contract Negotiation termination for id {:#?} with reason {}",
                pid.clone(),
                reason.clone()
            );

            let host = headers["host"].to_str().unwrap();

            let default_context = HashMap::from([
                (
                    "@vocab".to_string(),
                    Value::String("https://w3id.org/edc/v0.0.1/ns/".to_string()),
                ),
                (
                    "edc".to_string(),
                    Value::String("https://w3id.org/edc/v0.0.1/ns/".to_string()),
                ),
                (
                    "dcat".to_string(),
                    Value::String("http://www.w3.org/ns/dcat#".to_string()),
                ),
                (
                    "dct".to_string(),
                    Value::String("http://purl.org/dc/terms/".to_string()),
                ),
                (
                    "odrl".to_string(),
                    Value::String("http://www.w3.org/ns/odrl/2/".to_string()),
                ),
                (
                    "dspace".to_string(),
                    Value::String("https://w3id.org/dspace/v0.8/".to_string()),
                ),
            ]);

            let negotiation = ContractNegotiation {
                context: default_context,
                dsp_type: "dspace:ContractNegotiation".to_string(),
                consumer_pid: termination_request["dspace:consumerPid"]
                    .to_string()
                    .clone(),
                provider_pid: pid.clone(),
                state: NegotiationState::TERMINATED,
            };

            let mut fsm = ProviderStateMachine::new(
                host.clone(),
                "localhost:3000", // TODO: Change to a dynamic value
            );

            debug!(
                "[DSP] Contract Negotiation Provider PID: {:#?}",
                pid.clone()
            );

            let transition_message = format!(
                "Requesting Contract Negotiation Termination from {}",
                host.clone()
            );
            fsm.transition_to_terminating(transition_message.as_str());
            let transition_message = format!(
                "Terminated after Contract Negotiation Termination request from {} with reason {}",
                host.clone(),
                reason.clone()
            );
            fsm.transition_to_terminated(transition_message.as_str());

            debug!("[DSP] State Machine transitioned to: {}", fsm.state);
            println!("[DSP] FSM: {:#?}", fsm.clone()); // only for demonstration purposes, remove later

            state.insert(pid.clone(), (negotiation.clone(), fsm.clone()));

            StatusCode::OK.into_response()
        }
        None => {
            let err_msg = format!(
                "A Contract Negotiation with the given pid {:#?} does not exist.",
                pid.clone()
            );
            error!("{}", err_msg);
            (StatusCode::NOT_FOUND, Json(err_msg)).into_response()
        }
    }
}
