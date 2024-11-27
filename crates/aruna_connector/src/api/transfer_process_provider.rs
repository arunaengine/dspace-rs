use std::collections::HashMap;
use std::sync::{Arc};
use tokio::sync::Mutex;
use axum::response::IntoResponse;
use dsp_api::transfer_process::TransferProcess;
use odrl::functions::state_machine::{ProviderState, ProviderStateMachine};

type SharedState = Arc<Mutex<HashMap<String, (TransferProcess, ProviderStateMachine<ProviderState>)>>>;

pub async fn get_transfer_process() -> impl IntoResponse {
    unimplemented!()
}

pub async fn request_transfer_processes() -> impl IntoResponse {
    unimplemented!()
}

pub async fn start_transfer_process() -> impl IntoResponse {
    unimplemented!()
}

pub async fn complete_transfer_process() -> impl IntoResponse {
    unimplemented!()
}

pub async fn terminate_transfer_process() -> impl IntoResponse {
    unimplemented!()
}

pub async fn suspend_transfer_process() -> impl IntoResponse {
    unimplemented!()
}