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
use edc_api::{IdResponse, QuerySpec, TransferProcess, TransferRequest};
use edc_api::query_spec::SortOrder;
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

fn evaluate_condition(contract: &TransferProcess, operand_left: &Value, operator: &str, operand_right: &Value,) -> bool {
    let field_name = operand_left.as_str().unwrap_or("");

    match field_name {
        "@id" => compare_values(contract.at_id.as_deref(), operator, operand_right.as_str()),
        "@type" => compare_values(contract.at_type.as_deref(), operator, operand_right.as_str()),
        "correlationId" => compare_values(contract.correlation_id.as_deref(), operator, operand_right.as_str()),
        "assetId" => compare_values(contract.asset_id.as_deref(), operator, operand_right.as_str()),
        "contractId" => compare_values(contract.contract_agreement_id.as_deref(), operator, operand_right.as_str()),
        "counterPartyAddress" => compare_values(contract.counter_party_address.as_deref(), operator, operand_right.as_str()),
        "counterPartyId" => compare_values(contract.counter_party_id.as_deref(), operator, operand_right.as_str()),
        "errorDetail" => compare_values(contract.error_detail.as_deref(), operator, operand_right.as_str()),
        "protocol" => compare_values(contract.protocol.as_deref(), operator, operand_right.as_str()),
        "state" => compare_values(Some(contract.state.clone()), operator, Some(serde_json::from_value(operand_right.clone()).unwrap())),
        "stateTimestamp" => compare_values(contract.state_timestamp, operator, Some(serde_json::from_value::<i64>(operand_right.clone()).unwrap())),
        "transferType" => compare_values(contract.transfer_type.as_deref(), operator, operand_right.as_str()),
        "type" => compare_values(contract.r#type, operator, Some(serde_json::from_value::<RHashType>(operand_right.clone()).unwrap())),
        _ => false, // Unknown field
    }
}

fn compare_values<T: PartialOrd>(field_value: Option<T>, operator: &str, operand_right: Option<T>) -> bool {
    match (field_value, operand_right) {
        (Some(field_value), Some(operand_right)) => match operator {
            "=" => field_value == operand_right,
            "!=" => field_value != operand_right,
            ">" => field_value > operand_right,
            ">=" => field_value >= operand_right,
            "<" => field_value < operand_right,
            "<=" => field_value <= operand_right,
            _ => false,
        },
        _ => false,
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

pub(crate) async fn request_transfer_processes(State(state): State<SharedState>, Json(query): Json<QuerySpec>,) -> impl IntoResponse {

    /// Returns all transfer process according to a query
    ///
    /// # Example
    ///
    /// Request Body:
    /// {
    ///   "@context": {
    ///     "@vocab": "https://w3id.org/edc/v0.0.1/ns/"
    ///   },
    ///   "@type": "QuerySpec",
    ///   "offset": 5,
    ///   "limit": 10,
    ///   "sortOrder": "DESC",
    ///   "sortField": "fieldName",
    ///   "filterExpression": []
    /// }
    ///
    /// Responses:
    /// 200 - The transfer processes matching the query
    /// 400 - Request was malformed

    info!("Request Transfer Process called");
    debug!("Received Transfer Process request for query: {:#?}", query);

    let state = state.lock().await;

    // Collect all transfer processes into a vector
    let mut output: Vec<TransferProcess> = state.values().map(|(transfer_process, _)| transfer_process.clone()).collect();

    let offset = query.offset.unwrap_or(0);
    let limit = query.limit.unwrap_or(i32::MAX);
    let sort_order = query.sort_order.unwrap_or(SortOrder::Asc);
    let sort_field = query.sort_field;
    let filter_expression = query.filter_expression;

    // Sort state hashmap by value for the given key (sort_field) and order (sort_order) if provided in the query and save the result in output
    if sort_field.is_some() {
        let sort_field = sort_field.unwrap();

        output.sort_by(|a: &TransferProcess, b: &TransferProcess| {
            let a_contract = &a;
            let b_contract = &b;

            let ordering = match sort_field.as_str() {
                "@id" => a_contract.at_id.cmp(&b_contract.at_id),
                "@type" => a_contract.at_type.cmp(&b_contract.at_type),
                "correlationId" => a_contract.correlation_id.cmp(&b_contract.correlation_id),
                "assetId" => a_contract.asset_id.cmp(&b_contract.asset_id),
                "contractId" => a_contract.contract_agreement_id.cmp(&b_contract.contract_agreement_id),
                "counterPartyAddress" => a_contract.counter_party_address.cmp(&b_contract.counter_party_address),
                "counterPartyId" => a_contract.counter_party_id.cmp(&b_contract.counter_party_id),
                "errorDetail" => a_contract.error_detail.cmp(&b_contract.error_detail),
                "protocol" => a_contract.protocol.cmp(&b_contract.protocol),
                "state" => a_contract.state.cmp(&b_contract.state),
                "stateTimestamp" => a_contract.state_timestamp.cmp(&b_contract.state_timestamp),
                "transferType" => a_contract.transfer_type.cmp(&b_contract.transfer_type),
                "type" => a_contract.r#type.cmp(&b_contract.r#type),
                _ => std::cmp::Ordering::Equal,
            };

            if sort_order == SortOrder::Asc {
                ordering
            } else {
                ordering.reverse()
            }
        });
    }

    // Filter the output based on the filter_expression
    if !filter_expression.is_empty() {
        output = output.into_iter().filter(|(v)| {
            filter_expression.iter().all(|criterion| {
                evaluate_condition(v, &criterion.operand_left, &criterion.operator, &criterion.operand_right)
            })
        }).collect();
    }

    // Return only the requested range of results (based on offset and limit)
    output = if offset > output.len() as i32 {
        Vec::new()
    } else {
        output.into_iter()
            .skip(offset as usize)
            .take(limit as usize)
            .collect()
    };

    (StatusCode::OK, Json(output)).into_response()
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