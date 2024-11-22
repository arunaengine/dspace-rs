use std::collections::HashMap;
use std::sync::Arc;
use axum::response::IntoResponse;
use tokio::sync::Mutex;
use edc_api::{TransferProcess};

type SharedState = Arc<Mutex<HashMap<String, TransferProcess>>>;

pub(crate) async fn initiate_transfer_process() -> impl IntoResponse {
    unimplemented!()
}

pub(crate) async fn request_transfer_processes() -> impl IntoResponse {
    unimplemented!()
}

pub(crate) async fn get_transfer_process() -> impl IntoResponse {
    unimplemented!()
}

pub(crate) async fn deprovision_transfer_process_resource() -> impl IntoResponse {
    unimplemented!()
}

pub(crate) async fn resume_transfer_process() -> impl IntoResponse {
    unimplemented!()
}

pub(crate) async fn get_transfer_process_state() -> impl IntoResponse {
    unimplemented!()
}

pub(crate) async fn suspend_transfer_process() -> impl IntoResponse {
    unimplemented!()
}

pub(crate) async fn terminate_transfer_process() -> impl IntoResponse {
    unimplemented!()
}