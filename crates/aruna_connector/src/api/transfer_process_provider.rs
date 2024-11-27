use std::collections::HashMap;
use std::sync::{Arc};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use tokio::sync::Mutex;
use axum::response::IntoResponse;
use tracing::{debug, error, info};
use dsp_api::transfer_process::TransferProcess;
use odrl::functions::state_machine::{ProviderState, ProviderStateMachine};

type SharedState = Arc<Mutex<HashMap<String, (TransferProcess, ProviderStateMachine<ProviderState>)>>>;

pub async fn get_transfer_process(State(state): State<SharedState>, Path(pid): Path<String>,) -> impl IntoResponse {
    info!("[DSP] Get Transfer Process called");
    debug!("[DSP] Received Transfer Process request for pid {:#?}", pid.clone());

    let state = state.lock().await;

    match state.get(&pid) {
        Some((transfer_process, _)) => {
            (StatusCode::OK, Json(transfer_process.clone())).into_response()
        },
        None => {
            let err_msg = format!("A Transfer Process with the given pid {:#?} does not exist.", pid.clone());
            error!("{}", err_msg);
            (StatusCode::NOT_FOUND, Json(err_msg)).into_response()
        }
    }
}

pub async fn request_transfer_processes(State(state): State<SharedState>,) -> impl IntoResponse {
    unimplemented!()
}

pub async fn start_transfer_process(State(state): State<SharedState>,) -> impl IntoResponse {
    unimplemented!()
}

pub async fn complete_transfer_process(State(state): State<SharedState>,) -> impl IntoResponse {
    unimplemented!()
}

pub async fn terminate_transfer_process(State(state): State<SharedState>,) -> impl IntoResponse {
    unimplemented!()
}

pub async fn suspend_transfer_process(State(state): State<SharedState>,) -> impl IntoResponse {
    unimplemented!()
}