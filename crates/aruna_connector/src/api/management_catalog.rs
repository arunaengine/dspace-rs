use std::sync::Arc;
use axum::response::IntoResponse;
use tokio::sync::Mutex;
use dsp_api::catalog::Catalog;

type SharedState = Arc<Mutex<Catalog>>;

pub(crate) async fn request_catalog_dataset() -> impl IntoResponse {
    unimplemented!()
}

pub(crate) async fn request_catalog() -> impl IntoResponse {
    unimplemented!()
}