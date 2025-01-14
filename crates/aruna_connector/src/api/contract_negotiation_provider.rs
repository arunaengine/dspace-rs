use axum::extract::{ConnectInfo, Path, State};
use axum::http::{HeaderMap, HeaderValue, StatusCode};
use axum::response::IntoResponse;
use axum::{Extension, Json};
use dsp_api::contract_negotiation::contract_negotiation::NegotiationState;
use dsp_api::contract_negotiation::contract_negotiation_event_message::EventType;
use dsp_api::contract_negotiation::{
    AbstractPolicyRule, Action, Agreement, ContractAgreementMessage, ContractNegotiation,
    ContractNegotiationEventMessage, ContractNegotiationTerminationMessage, ContractOfferMessage,
    MessageOffer, Permission, PolicyClass, Target,
};
use dsp_client::configuration::Configuration;
use odrl::functions::state_machine::{ConsumerStateMachine, ProviderState, ProviderStateMachine};
use reqwest::header::{AUTHORIZATION, HOST};
use reqwest::Client;
use reqwest::Error;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::sync::MutexGuard;
use tokio::task;
use tokio::time::sleep;
use tracing::{debug, error, info};
use uuid::Uuid;
use crate::common::DEFAULT_CONTEXT;

// SharedState:
//     Key: PID of the contract negotiation
//     Value: Tuple of ContractNegotiation and ProviderStateMachine<ProviderState>, to keep track of the state of the negotiation
type SharedState =
    Arc<Mutex<HashMap<String, (ContractNegotiation, ProviderStateMachine<ProviderState>)>>>;

async fn send_agreement(
    agreement: ContractAgreementMessage,
    cb_address: String,
    pid: String,
) -> Result<(), Error> {
    let mut dest = cb_address
        .clone()
        .replace("consumer-connector", "localhost");
    dest = dest.replace("9194", "19194");
    let dest = format!("{}/negotiations/{}/agreement", dest, pid);
    debug!(
        "[DSP] Sending Contract Agreement Message to Consumer at {:#?}",
        dest.clone()
    );
    let http_client = Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, HeaderValue::from_static("123456"));
    headers.insert("x-api-key", HeaderValue::from_static("123456"));
    let response = http_client
        .post(dest)
        .json(&agreement)
        .headers(headers)
        .send()
        .await;
    debug!("[DSP] Response received from the Consumer: {:#?}", response);

    Ok(())
}

async fn send_finalization(event_message: ContractNegotiationEventMessage, dest: String) {
    debug!(
        "[DSP] Sending Contract Negotiation Finalization Message to Consumer at {:#?}",
        dest.clone()
    );
    let http_client = Client::new();
    let response = http_client.post(dest).json(&event_message).send().await;
    debug!("[DSP] Response received from the Consumer: {:#?}", response);
}

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

        let negotiation = ContractNegotiation {
            context: DEFAULT_CONTEXT.clone(),
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

        let policy_id = request_message["dspace:offer"]["@id"]
            .as_str()
            .unwrap()
            .to_string();
        let asset_id = request_message["dspace:offer"]["odrl:target"]["@id"]
            .as_str()
            .unwrap()
            .to_string();

        let agreement_message = ContractAgreementMessage {
            context: DEFAULT_CONTEXT.clone(),
            dsp_type: "dspace:ContractAgreementMessage".to_string(),
            provider_pid: pid.clone(),
            consumer_pid: request_message["dspace:consumerPid"]
                .as_str()
                .unwrap()
                .to_string(),
            agreement: Agreement {
                policy_class: PolicyClass {
                    abstract_policy_rule: AbstractPolicyRule {
                        assigner: Some("provider".to_string()),
                        assignee: Some("consumer".to_string()),
                    },
                    id: policy_id.clone(),
                    profile: vec![],
                    permission: vec![],
                    obligation: vec![],
                    target: Target {
                        id: asset_id.clone(),
                    },
                },
                odrl_type: "odrl:Agreement".to_string(),
                id: Uuid::new_v4().to_string(),
                target: asset_id.clone(),
                timestamp: Some(chrono::Utc::now().to_rfc3339()),
            },
            callback_address: "http://localhost:3000/".to_string(),
        };

        task::spawn(async move {
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            if let Err(e) = send_agreement(
                agreement_message,
                request_message["dspace:callbackAddress"]
                    .as_str()
                    .unwrap()
                    .to_string(),
                request_message["dspace:consumerPid"]
                    .as_str()
                    .unwrap()
                    .to_string(),
            )
            .await
            {
                error!("[DSP] Failed to send agreement: {}", e);
            }
        });

        return (StatusCode::OK, Json(negotiation.clone())).into_response();
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

pub async fn verify_agreement(
    headers: HeaderMap,
    State(state): State<SharedState>,
    Path(pid): Path<String>,
    Json(request_message): Json<Value>,
) -> impl IntoResponse {
    info!("[DSP] Verify Negotiation called");
    debug!(
        "[DSP] Received Negotiation verification request for pid {:#?}",
        pid.clone()
    );

    // TODO Verify ...

    // Send Finalization

    let finalize_message = ContractNegotiationEventMessage {
        context: DEFAULT_CONTEXT.clone(),
        dsp_type: "dspace:ContractNegotiationEventMessage".to_string(),
        provider_pid: request_message["dspace:providerPid"]
            .as_str()
            .unwrap()
            .to_string(),
        consumer_pid: request_message["dspace:consumerPid"]
            .as_str()
            .unwrap()
            .to_string(),
        event_type: EventType::FINALIZED,
    };

    let event_url = format!("{}", headers["host"].to_str().unwrap().to_string());

    send_finalization(
        finalize_message,
        headers["host"].to_str().unwrap().to_string(),
    )
    .await;

    StatusCode::OK
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
        Some((negotiation, state_machine)) => {
            let reason = termination_request["dspace:reason"].clone();
            debug!(
                "[DSP] Received Contract Negotiation termination for id {:#?} with reason {}",
                pid.clone(),
                reason.clone()
            );

            let host = headers["host"].to_str().unwrap();

            let partner = state_machine.negotiation_partner.as_str().clone();

            let negotiation = ContractNegotiation {
                context: DEFAULT_CONTEXT.clone(),
                dsp_type: "dspace:ContractNegotiation".to_string(),
                consumer_pid: termination_request["dspace:consumerPid"]
                    .to_string()
                    .clone(),
                provider_pid: pid.clone(),
                state: NegotiationState::TERMINATED,
            };

            let mut fsm = ProviderStateMachine::new(
                partner.clone(), // TODO
                host.clone(), // TODO: Change to a dynamic value
            );

            debug!(
                "[DSP] Contract Negotiation Provider PID: {:#?}",
                pid.clone()
            );

            let transition_message = format!(
                "Requesting Contract Negotiation Termination from {}",
                partner.clone()    // TODO
            );
            fsm.transition_to_terminating(transition_message.as_str());
            let transition_message = format!(
                "Terminated after Contract Negotiation Termination request from {} with reason {}",
                partner.clone(),   //TODO
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
