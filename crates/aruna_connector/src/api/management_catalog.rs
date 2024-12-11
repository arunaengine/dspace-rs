use axum::response::IntoResponse;
use dsp_api::catalog::Catalog;
use std::sync::Arc;
use tokio::sync::Mutex;

type SharedState = Arc<Mutex<Catalog>>;

pub(crate) async fn request_catalog_dataset() -> impl IntoResponse {
    unimplemented!()
}

pub(crate) async fn request_catalog() -> impl IntoResponse {
    unimplemented!()
}
