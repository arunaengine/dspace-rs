use std::collections::HashMap;
use std::sync::Arc;
use axum::response::IntoResponse;
use tokio::sync::Mutex;
use edc_api::{ContractAgreement};

type SharedState = Arc<Mutex<HashMap<String, ContractAgreement>>>;

pub(crate) async fn request_agreements() -> impl IntoResponse {
    unimplemented!()
}

pub(crate) async fn get_agreement() -> impl IntoResponse {
    unimplemented!()
}

pub(crate) async fn get_agreement_negotiation() -> impl IntoResponse {
    unimplemented!()
}