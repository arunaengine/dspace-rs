use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::Json;
use dsp_api::transfer_process::transfer_process::{EDCTransferState, TransferStateType};
use dsp_api::transfer_process::TransferProcess;
use edc_api::transfer_state::TransferProcessState;
use odrl::functions::state_machine::{ProviderState, ProviderStateMachine};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{debug, error, info};
use uuid::Uuid;

// TODO: Add State Handling
// TODO: Implement TP-specific State Machine

type SharedState =
    Arc<Mutex<HashMap<String, (TransferProcess, ProviderStateMachine<ProviderState>)>>>;

pub async fn get_transfer_process(
    State(state): State<SharedState>,
    Path(pid): Path<String>,
) -> impl IntoResponse {
    info!("[DSP] Get Transfer Process called");
    debug!(
        "[DSP] Received Transfer Process request for pid {:#?}",
        pid.clone()
    );

    let state = state.lock().await;

    match state.get(&pid) {
        Some((transfer_process, _)) => {
            (StatusCode::OK, Json(transfer_process.clone())).into_response()
        }
        None => {
            let err_msg = format!(
                "A Transfer Process with the given pid {:#?} does not exist.",
                pid.clone()
            );
            error!("{}", err_msg);
            (StatusCode::NOT_FOUND, Json(err_msg)).into_response()
        }
    }
}

pub async fn request_transfer_processes(
    headers: HeaderMap,
    State(state): State<SharedState>,
    Json(request_message): Json<Value>,
) -> impl IntoResponse {
    info!("[DSP] Request Transfer Process called");
    debug!("[DSP] Request Body: {:#?}", request_message.clone());

    let mut state = state.lock().await;

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

    debug!(
        "[DSP] Initialized new Transfer Process State Machine; State: {}",
        fsm.state
    );

    let provider_pid = Uuid::new_v4().to_string();

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

    let transfer_process = TransferProcess {
        context: default_context,
        dsp_type: "dspace:TransferProcess".to_string(),
        provider_pid: provider_pid.clone(),
        consumer_pid: request_message["dspace:consumerPid"]
            .as_str()
            .unwrap()
            .to_string(),
        state: TransferStateType::EDCTransferState(EDCTransferState::Initial),
    };

    debug!(
        "[DSP] Contract Negotiation Provider PID: {:#?}",
        provider_pid.clone()
    );
    println!("[DSP] Transfer Process FSM: {:#?}", fsm.clone()); // only for demonstration purposes, remove later

    state.insert(
        provider_pid.clone(),
        (transfer_process.clone(), fsm.clone()),
    );

    (StatusCode::CREATED, Json(transfer_process.clone())).into_response()
}

pub async fn start_transfer_process(State(state): State<SharedState>) -> impl IntoResponse {
    unimplemented!()
}

pub async fn complete_transfer_process(State(state): State<SharedState>) -> impl IntoResponse {
    unimplemented!()
}

pub async fn terminate_transfer_process(State(state): State<SharedState>) -> impl IntoResponse {
    unimplemented!()
}

pub async fn suspend_transfer_process(State(state): State<SharedState>) -> impl IntoResponse {
    unimplemented!()
}
