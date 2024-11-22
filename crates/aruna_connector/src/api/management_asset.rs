use std::collections::HashMap;
use std::sync::{Arc};
use tokio::sync::Mutex;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use chrono::Utc;
use tracing::{debug, error, info};
use uuid::Uuid;
use edc_api::{AssetInput, AssetOutput, IdResponse, QuerySpec};
use edc_api::query_spec::SortOrder;

type SharedState = Arc<Mutex<HashMap<String, AssetOutput>>>;

async fn input2output(input: AssetInput, id: String, created_at: i64) -> AssetOutput {
    AssetOutput {
        context: input.context.clone(),
        at_id: Some(id.clone()),
        at_type: input.at_type.clone(),
        created_at: Some(created_at.clone()),
        data_address: Some(input.data_address.clone()),
        private_properties: input.private_properties.clone(),
        properties: Some(input.properties.clone()),
    }
}

fn evaluate_condition(asset: &AssetOutput, operand_left: &serde_json::Value, operator: &str, operand_right: &serde_json::Value,) -> bool {
    let field_name = operand_left.as_str().unwrap_or("");

    match field_name {

        // TODO: Add filtering of properties / privateproperties ?

        "@id" => compare_values(asset.at_id.as_deref(), operator, operand_right.as_str()),
        "@type" => compare_values(asset.at_type.as_deref(), operator, operand_right.as_str()),
        "createdAt" => {
            if let Some(parsed_value) = operand_right.as_i64() {
                compare_values(asset.created_at, operator, Some(parsed_value))
            } else {
                false
            }
        }
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

pub(crate) async fn update_asset(State(state): State<SharedState>, Json(input): Json<AssetInput>,) -> impl IntoResponse {

    /// Updates an asset with the given ID if it exists. If the asset is not found, no further action is taken.
    /// DANGER ZONE: Note that updating assets can have unexpected results,
    /// especially for contract offers that have been sent out or are ongoing in contract negotiations.
    ///
    /// # Example
    ///
    /// Request Body:
    /// {
    ///   "@context": {
    ///     "@vocab": "https://w3id.org/edc/v0.0.1/ns/"
    ///   },
    ///   "@id": "asset-id",
    ///   "properties": {
    ///     "key": "value"
    ///   },
    ///   "privateProperties": {
    ///     "privateKey": "privateValue"
    ///   },
    ///   "dataAddress": {
    ///     "type": "HttpData",
    ///     "baseUrl": "https://jsonplaceholder.typicode.com/todos"
    ///   }
    /// }
    ///
    /// Responses:
    /// 204 - Asset was updated successfully
    /// 400 - Request was malformed, e.g. id was null
    /// 404 - Asset could not be updated, because it does not exist.

    info!("Update Asset called");

    if input.at_id.is_none() {
        return (StatusCode::BAD_REQUEST, Json(error!("Request was malformed, id was null"))).into_response();
    }

    let id = input.at_id.clone().unwrap();

    debug!("Received asset update for id {:#?}", id.clone());
    debug!("Update information: {:#?}", input.clone());

    let mut state = state.lock().await;

    if state.contains_key(&id) {
        let created_at = Utc::now().timestamp();

        let asset_output = input2output(input.clone(), id.clone(), created_at.clone()).await;

        state.insert(id.clone(), asset_output);
        StatusCode::NO_CONTENT.into_response()
    } else {
        (StatusCode::NOT_FOUND, Json(error!("Asset could not be updated, because an Asset with the given id does not exist."))).into_response()
    }
}

pub(crate) async fn create_asset(State(state): State<SharedState>, Json(input): Json<AssetInput>,) -> impl IntoResponse {

    /// Creates a new asset together with a data address
    /// If no @id is provided, a new random UUID will be generated
    ///
    /// # Example
    ///
    /// Request Body:
    /// {
    ///   "@context": {
    ///     "@vocab": "https://w3id.org/edc/v0.0.1/ns/"
    ///   },
    ///   "@id": "asset-id",
    ///   "properties": {
    ///     "key": "value"
    ///   },
    ///   "privateProperties": {
    ///     "privateKey": "privateValue"
    ///   },
    ///   "dataAddress": {
    ///     "type": "HttpData",
    ///     "baseUrl": "https://jsonplaceholder.typicode.com/todos"
    ///   }
    /// }
    ///
    /// Responses:
    /// 200 - Asset was created successfully. Returns the asset Id and created timestamp
    ///       {
    ///            "@context": {
    ///                "@vocab": "https://w3id.org/edc/v0.0.1/ns/"
    ///            },
    ///            "@id": "id-value",
    ///            "createdAt": 1688465655
    ///       }
    /// 400 - Request body was malformed
    /// 404 - Could not create asset, because an asset with that ID already exists

    info!("Create Asset called");
    debug!("Request Body: {:#?}", input.clone());

    let mut state = state.lock().await;

    let id = input.at_id.clone().unwrap_or_else(|| Uuid::new_v4().to_string());

    if state.contains_key(&id) {
        return (StatusCode::CONFLICT, Json(error!("Could not create asset, because an asset with the given ID already exists"))).into_response();
    }

    let created_at = Utc::now().timestamp();

    let asset_output = input2output(input.clone(), id.clone(), created_at.clone()).await;

    state.insert(id.clone(), asset_output.clone());

    let id_response = IdResponse {
        at_id: Some(id.clone()),
        created_at: Some(created_at.clone()),
    };

    (StatusCode::OK, Json(id_response)).into_response()

}

pub(crate) async fn request_assets(State(state): State<SharedState>, Json(query): Json<QuerySpec>,) -> impl IntoResponse {

    /// Request all assets according to a particular query
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
    /// 200 - The assets matching the query
    /// 400 - Request was malformed

    info!("Request Assets called");
    debug!("Received Asset request for query: {:#?}", query);

    let state = state.lock().await;

    // Collect all assets into a vector
    let mut output: Vec<AssetOutput> = state.values().cloned().collect();

    let offset = query.offset.unwrap_or(0);
    let limit = query.limit.unwrap_or(i32::MAX);
    let sort_order = query.sort_order.unwrap_or(SortOrder::Asc);
    let sort_field = query.sort_field;
    let filter_expression = query.filter_expression;

    // Sort state hashmap by value for the given key (sort_field) and order (sort_order) if provided in the query and save the result in output
    if sort_field.is_some() {
        let sort_field = sort_field.unwrap();

        output.sort_by(|a: &AssetOutput, b: &AssetOutput| {
            let a_contract = &a;
            let b_contract = &b;

            let ordering = match sort_field.as_str() {
                "@id" => a_contract.at_id.cmp(&b_contract.at_id),
                "@type" => a_contract.at_type.cmp(&b_contract.at_type),
                "createdAt" => a_contract.created_at.cmp(&b_contract.created_at),
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

pub(crate) async fn get_asset(State(state): State<SharedState>, Path(id): Path<String>,) -> impl IntoResponse {

    /// Gets an asset with the given ID
    ///
    /// # Example
    ///
    /// GET /v3/assets/{id}
    ///
    /// Parameter:
    /// id: String (required)  - The ID of the Asset
    ///
    /// Responses:
    /// 200 - The asset
    /// 400 - Request was malformed, e.g. id was null
    /// 404 - An asset with the given ID does not exist

    info!("Get Asset called");
    debug!("Received Asset request for id: {:#?}", id.clone());

    let state = state.lock().await;
    match state.get(&id) {
        Some(output) => (StatusCode::OK, Json(output.clone())).into_response(),
        None => (StatusCode::NOT_FOUND, Json(error!("An Asset with the given ID does not exist"))).into_response(),
    }
}

pub(crate) async fn delete_asset(State(state): State<SharedState>, Path(id): Path<String>,) -> impl IntoResponse {

    /// Removes an asset with the given ID if possible. Deleting an asset is only possible
    /// if that asset is not yet referenced by a contract agreement, in which case an error is returned.
    /// DANGER ZONE: Note that deleting assets can have unexpected results,
    /// especially for contract offers that have been sent out or ongoing or contract negotiations.
    ///
    /// # Example
    ///
    /// DELETE /v3/assets/{id}
    ///
    /// Parameter:
    /// id: String (required)  - The ID of the Asset
    ///
    /// Responses:
    /// 204 - Asset was deleted successfully
    /// 400 - Request was malformed, e.g. id was null
    /// 404 - An asset with the given ID does not exist

    info!("Delete Asset called");
    debug!("Received Asset deletion request for Asset with id: {:#?}", id.clone());

    let mut state = state.lock().await;
    if state.remove(&id).is_some() {
        StatusCode::NO_CONTENT.into_response()
    } else {
        (StatusCode::NOT_FOUND, Json(error!("Asset could not be deleted, because an Asset with the given id does not exist."))).into_response()
    }
}