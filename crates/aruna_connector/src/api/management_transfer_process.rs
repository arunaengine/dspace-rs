use std::collections::HashMap;
use std::sync::Arc;
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::Json;
use axum::response::IntoResponse;
use chrono::Utc;
use serde_json::Value;
use tokio::sync::Mutex;
use tracing::{debug, info};
use uuid::Uuid;
use dsp_api::transfer_process::TransferRequestMessage;
use edc_api::{IdResponse, TransferProcess, TransferRequest};
use edc_api::transfer_process::RHashType;
use edc_api::transfer_state::TransferProcessState;
use odrl::functions::state_machine::{ConsumerState, ConsumerStateMachine};

type SharedState = Arc<Mutex<HashMap<String, (TransferProcess, ConsumerStateMachine<ConsumerState>)>>>;

async fn input2output(input: TransferRequest) -> TransferProcess {
    TransferProcess {
        context: input.context,
        at_type: input.at_type,
        at_id: Some(Uuid::new_v4().to_string()),
        correlation_id: None,
        callback_addresses: input.callback_addresses.unwrap_or_else(|| vec![]),
        asset_id: Some(input.asset_id),
        contract_agreement_id: Some(input.contract_id),
        counter_party_address: Some(input.counter_party_address),
        counter_party_id: None,
        data_destination: Some(input.data_destination),
        error_detail: None,
        private_properties: input.private_properties,
        protocol: Some(input.protocol),
        state: Some(TransferProcessState::Initial),
        state_timestamp: Some(Utc::now().timestamp()),
        transfer_type: Some(input.transfer_type),
        r#type: Some(RHashType::Consumer),  // if the management API is called, the role is always consumer
    }
}

async fn tp_request_management2dsp(management_tp: TransferRequest, consumer_pid: String) -> TransferRequestMessage {
    let default_context = HashMap::from([
        ("@vocab".to_string(), Value::String("https://w3id.org/edc/v0.0.1/ns/".to_string())),
        ("edc".to_string(), Value::String("https://w3id.org/edc/v0.0.1/ns/".to_string())),
        ("dcat".to_string(), Value::String("http://www.w3.org/ns/dcat#".to_string())),
        ("dct".to_string(), Value::String("http://purl.org/dc/terms/".to_string())),
        ("odrl".to_string(), Value::String("http://www.w3.org/ns/odrl/2/".to_string())),
        ("dspace".to_string(), Value::String("https://w3id.org/dspace/v0.8/".to_string()))
    ]);

    TransferRequestMessage {
        context: default_context,
        dsp_type: "dspace:TransferRequestMessage".to_string(),
        agreement_id: management_tp.contract_id,
        dct_format: management_tp.transfer_type,
        data_address: None,
        callback_address: management_tp.callback_addresses.unwrap().clone().get(0).clone().unwrap().clone().uri.unwrap(),
        consumer_pid,
    }
}

pub(crate) async fn initiate_transfer_process(headers: HeaderMap, State(state): State<SharedState>, Json(input): Json<TransferRequest>,) -> impl IntoResponse {

    /// Initiates a data transfer with the given parameters. Due to the asynchronous nature of transfers,
    /// a successful response only indicates that the request was successfully received.
    /// This may take a long time, so clients must poll the /{id}/state endpoint to track the state.
    ///
    /// # Example
    ///
    /// Request Body:
    /// {
    ///   "@context": {
    ///     "@vocab": "https://w3id.org/edc/v0.0.1/ns/"
    ///   },
    ///   "@type": "https://w3id.org/edc/v0.0.1/ns/TransferRequest",
    ///   "protocol": "dataspace-protocol-http",
    ///   "counterPartyAddress": "http://provider-address",
    ///   "contractId": "contract-id",
    ///   "assetId": "asset-id",
    ///   "transferType": "transferType",
    ///   "dataDestination": {
    ///     "type": "data-destination-type"
    ///   },
    ///   "privateProperties": {
    ///     "private-key": "private-value"
    ///   },
    ///   "callbackAddresses": [
    ///     {
    ///       "transactional": false,
    ///       "uri": "http://callback/url",
    ///       "events": [
    ///         "contract.negotiation",
    ///         "transfer.process"
    ///       ],
    ///       "authKey": "auth-key",
    ///       "authCodeId": "auth-code-id"
    ///     }
    ///   ]
    /// }
    ///
    /// Responses:
    /// 200 - The transfer was successfully initiated. Returns the transfer process ID and created timestamp
    ///       {
    ///            "@context": {
    ///                "@vocab": "https://w3id.org/edc/v0.0.1/ns/"
    ///            },
    ///            "@id": "id-value",
    ///            "createdAt": 1688465655
    ///        }
    /// 400 - Request was malformed

    info!("Initiate Transfer Process called");
    debug!("Request Body: {:#?}", input.clone());

    let mut state = state.lock().await;
    let id = uuid::Uuid::new_v4().to_string();

    let transfer_process = input2output(input).await;

    let consumer_fsm = ConsumerStateMachine::new(headers.get("host").unwrap().to_str().unwrap(), input.counter_party_address.clone().as_str());

    debug!("Transfer Process state machine initialized for consumer: {:#?}", consumer_fsm);

    state.insert(id.clone(), (transfer_process.clone(), consumer_fsm));

    let id_response = IdResponse {
        at_id: transfer_process.clone().at_id,
        created_at: transfer_process.clone().state_timestamp,
    };

    // TODO: Add calling of the dsp endpoint

    (StatusCode::OK, Json(id_response)).into_response()

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