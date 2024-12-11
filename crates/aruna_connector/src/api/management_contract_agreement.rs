use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use edc_api::contract_negotiation::EnumType;
use edc_api::query_spec::SortOrder;
use edc_api::{ContractAgreement, ContractNegotiation, QuerySpec};
use reqwest::Client;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{debug, error, info};

type SharedState = Arc<Mutex<HashMap<String, ContractAgreement>>>;

fn evaluate_condition(
    contract: &ContractAgreement,
    operand_left: &serde_json::Value,
    operator: &str,
    operand_right: &serde_json::Value,
) -> bool {
    let field_name = operand_left.as_str().unwrap_or("");

    match field_name {
        "@id" => compare_values(contract.at_id.as_deref(), operator, operand_right.as_str()),
        "@type" => compare_values(
            contract.at_type.as_deref(),
            operator,
            operand_right.as_str(),
        ),
        "assetId" => compare_values(
            contract.asset_id.as_deref(),
            operator,
            operand_right.as_str(),
        ),
        "contractSigningDate" => compare_values(
            contract.contract_signing_date,
            operator,
            operand_right.as_i64(),
        ),
        "consumerId" => compare_values(
            contract.consumer_id.as_deref(),
            operator,
            operand_right.as_str(),
        ),
        "providerId" => compare_values(
            contract.provider_id.as_deref(),
            operator,
            operand_right.as_str(),
        ),
        _ => false, // Unknown field
    }
}

fn compare_values<T: PartialOrd>(
    field_value: Option<T>,
    operator: &str,
    operand_right: Option<T>,
) -> bool {
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

pub(crate) async fn request_agreements(
    State(state): State<SharedState>,
    Json(query): Json<QuerySpec>,
) -> impl IntoResponse {
    /// Gets all contract agreements according to a particular query
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
    /// 200 - The contract agreements matching the query
    /// 400 - Request body was malformed

    info!("Request Contract Agreement called");
    debug!(
        "Received Contract Agreement request for query: {:#?}",
        query
    );

    let mut state = state.lock().await;

    // Collect all contract agreements into a vector
    let mut output: Vec<ContractAgreement> = state.values().cloned().collect();

    let offset = query.offset.unwrap_or(0);
    let limit = query.limit.unwrap_or(i32::MAX);
    let sort_order = query.sort_order.unwrap_or(SortOrder::Asc);
    let sort_field = query.sort_field;
    let filter_expression = query.filter_expression;

    // Sort state hashmap by value for the given key (sort_field) and order (sort_order) if provided in the query and save the result in output
    if sort_field.is_some() {
        let sort_field = sort_field.unwrap();

        output.sort_by(|a: &ContractAgreement, b: &ContractAgreement| {
            let a_contract = &a;
            let b_contract = &b;

            let ordering = match sort_field.as_str() {
                "@id" => a_contract.at_id.cmp(&b_contract.at_id),
                "@type" => a_contract.at_type.cmp(&b_contract.at_type),
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
        output = output
            .into_iter()
            .filter(|(v)| {
                filter_expression.iter().all(|criterion| {
                    evaluate_condition(
                        v,
                        &criterion.operand_left,
                        &criterion.operator,
                        &criterion.operand_right,
                    )
                })
            })
            .collect();
    }

    // Return only the requested range of results (based on offset and limit)
    output = if offset > output.len() as i32 {
        Vec::new()
    } else {
        output
            .into_iter()
            .skip(offset as usize)
            .take(limit as usize)
            .collect()
    };

    (StatusCode::OK, Json(output)).into_response()
}

pub(crate) async fn get_agreement(
    State(state): State<SharedState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    /// Gets a contract agreement with the given ID
    ///
    /// # Example
    ///
    /// Request:
    /// GET /v2/contractagreements/{id}
    ///
    /// Parameter:
    /// id: String (required)  - The ID of the contract
    ///
    /// Responses:
    /// 200 - The contract agreement
    /// 400 - Request was malformed, e.g. id was null
    /// 404 - A contract agreement with the given ID does not exist

    info!("Get Contract Agreement called");
    debug!(
        "Received Contract Agreement request for id: {:#?}",
        id.clone()
    );

    let state = state.lock().await;
    match state.get(&id) {
        Some(output) => (StatusCode::OK, Json(output.clone())).into_response(),
        None => (
            StatusCode::NOT_FOUND,
            Json(error!(
                "A contract agreement with the given ID does not exist"
            )),
        )
            .into_response(),
    }
}

pub(crate) async fn get_agreement_negotiation(
    State(state): State<SharedState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    /// Gets a contract negotiation with the given contract agreement ID
    ///
    /// # Example
    ///
    /// Request:
    /// GET /v2/contractagreements/{id}/negotiation
    ///
    /// Parameter:
    /// id: String (required)  - The ID of the contract
    ///
    /// Responses:
    /// 200 - The contract negotiation
    /// 400 - Request was malformed, e.g. id was null
    /// 404 - A contract agreement with the given ID does not exist
    ///

    info!("Get Contract Negotiation by Agreement called");
    debug!(
        "Received Contract Negotiation by Agreement request for id: {:#?}",
        id.clone()
    );

    let state = state.lock().await;

    if state.contains_key(&id) {
        let url = format!(
            "http://localhost:3000/v2/contractnegotiations/{}",
            id.clone()
        );
        let http_client = Client::new();
        let response = http_client.get(url).send().await;

        match response {
            Ok(response) => {
                let negotiation: ContractNegotiation = response.json().await.unwrap();
                (StatusCode::OK, Json(negotiation)).into_response()
            }
            Err(e) => {
                error!("Error: {:#?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(error!(
                        "An error occurred while fetching the contract negotiation"
                    )),
                )
                    .into_response()
            }
        }
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(error!(
                "A contract agreement with the given ID does not exist"
            )),
        )
            .into_response()
    }
}
