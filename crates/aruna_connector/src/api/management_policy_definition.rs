use std::collections::HashMap;
use std::sync::{Arc};
use tokio::sync::Mutex;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use chrono::Utc;
use serde_json::json;
use tracing::{debug, error, info};
use uuid::Uuid;
use reqwest::Client;
use edc_api::{ContractDefinitionOutput, Criterion, IdResponse, PolicyDefinitionInput, PolicyDefinitionOutput, QuerySpec};
use edc_api::query_spec::SortOrder;

type SharedState = Arc<Mutex<HashMap<String, PolicyDefinitionOutput>>>;

async fn input2output(input: PolicyDefinitionInput, id: String, created_at: Option<i64>) -> PolicyDefinitionOutput {
    PolicyDefinitionOutput {
        context: Default::default(),
        at_id: Some(id.clone()),
        at_type: input.at_type.clone(),
        policy: Some(input.policy.clone()),
        created_at: Some(created_at.unwrap_or_else(|| Utc::now().timestamp())),
    }
}

fn evaluate_condition(policy: &PolicyDefinitionOutput, operand_left: &serde_json::Value, operator: &str, operand_right: &serde_json::Value,) -> bool {
    let field_name = operand_left.as_str().unwrap_or("");

    match field_name {

        // TODO: Add filtering of policy field ?

        "@id" => compare_values(policy.at_id.as_deref(), operator, operand_right.as_str()),
        "@type" => compare_values(policy.at_type.as_deref(), operator, operand_right.as_str()),
        "createdAt" => {
            if let Some(parsed_value) = operand_right.as_i64() {
                compare_values(policy.created_at, operator, Some(parsed_value))
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

pub(crate) async fn create_policy_definition(State(state): State<SharedState>, Json(input): Json<PolicyDefinitionInput>,) -> impl IntoResponse {

    /// Creates a new policy definition
    /// If no @id is provided, a new random UUID will be generated
    ///
    /// # Example
    ///
    /// Request Body:
    /// {
    ///   "@context": {
    ///     "@vocab": "https://w3id.org/edc/v0.0.1/ns/"
    ///   },
    ///   "@id": "definition-id",
    ///   "policy": {
    ///     "@context": "http://www.w3.org/ns/odrl.jsonld",
    ///     "@type": "Set",
    ///     "uid": "http://example.com/policy:1010",
    ///     "permission": [
    ///       {
    ///         "target": "http://example.com/asset:9898.movie",
    ///         "action": "display",
    ///         "constraint": [
    ///           {
    ///             "leftOperand": "spatial",
    ///             "operator": "eq",
    ///             "rightOperand": "https://www.wikidata.org/wiki/Q183",
    ///             "comment": "i.e Germany"
    ///           }
    ///         ]
    ///       }
    ///     ]
    ///   }
    /// }
    ///
    /// Responses:
    /// 200 -  policy definition was created successfully. Returns the Policy Definition Id and created timestamp
    ///        {
    ///             "@context": {
    ///                 "@vocab": "https://w3id.org/edc/v0.0.1/ns/"
    ///             },
    ///             "@id": "id-value",
    ///             "createdAt": 1688465655
    ///         }
    /// 400 - Request body was malformed
    /// 404 - Could not create policy definition, because a contract definition with that ID already exists

    info!("Create Policy Definition called");
    debug!("Request Body: {:#?}", input.clone());

    let mut state = state.lock().await;

    let id = input.at_id.clone().unwrap_or_else(|| Uuid::new_v4().to_string());

    if state.contains_key(&id) {
        return (StatusCode::CONFLICT, Json(error!("Could not create policy definition, because a policy definition with the given ID already exists"))).into_response();
    }

    let created_at = Utc::now().timestamp();

    let policy_definition = input2output(input.clone(), id.clone(), Some(created_at)).await;

    state.insert(id.clone(), policy_definition.clone());

    let id_response = IdResponse {
        at_id: Some(id.clone()),
        created_at: Some(created_at.clone()),
    };

    (StatusCode::OK, Json(id_response)).into_response()
}

pub(crate) async fn request_policy_definitions(State(state): State<SharedState>, Json(query): Json<QuerySpec>,) -> impl IntoResponse {

    /// Returns all policy definitions according to a query
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
    /// 200 - The policy definitions matching the query
    /// 400 - Request was malformed

    info!("Request Policy Definition called");
    debug!("Received Policy Definition request for query: {:#?}", query);

    let state = state.lock().await;

    // Collect all policy definitions into a vector
    let mut output: Vec<PolicyDefinitionOutput> = state.values().cloned().collect();

    let offset = query.offset.unwrap_or(0);
    let limit = query.limit.unwrap_or(i32::MAX);
    let sort_order = query.sort_order.unwrap_or(SortOrder::Asc);
    let sort_field = query.sort_field;
    let filter_expression = query.filter_expression;

    // Sort state hashmap by value for the given key (sort_field) and order (sort_order) if provided in the query and save the result in output
    if sort_field.is_some() {
        let sort_field = sort_field.unwrap();

        output.sort_by(|a: &PolicyDefinitionOutput, b: &PolicyDefinitionOutput| {
            let a_policy = &a;
            let b_policy = &b;

            let ordering = match sort_field.as_str() {
                "@id" => a_policy.at_id.cmp(&b_policy.at_id),
                "@type" => a_policy.at_type.cmp(&b_policy.at_type),
                "createdAt" => a_policy.created_at.cmp(&b_policy.created_at),
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
            filter_expression.iter().any(|criterion| {
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

pub(crate) async fn get_policy_definition(State(state): State<SharedState>, Path(id): Path<String>,) -> impl IntoResponse {

    /// Gets a policy definition with the given ID
    ///
    /// # Example
    ///
    /// GET /v2/policydefinitions/{id}
    ///
    /// Parameter:
    /// id: String (required)  - The ID of the policy definition
    ///
    /// Responses:
    /// 200 - The policy definition
    /// 400 - Request was malformed, e.g. id was null
    /// 404 - An policy definition with the given ID does not exist

    info!("Get Policy Definition called");
    debug!("Received Policy Definition request for id: {:#?}", id.clone());

    let state = state.lock().await;
    match state.get(&id) {
        Some(output) => (StatusCode::OK, Json(output.clone())).into_response(),
        None => (StatusCode::NOT_FOUND, Json(error!("A Policy Definition with the given ID does not exist"))).into_response(),
    }

}

pub(crate) async fn update_policy_definition(State(state): State<SharedState>, Json(input): Json<PolicyDefinitionInput>,) -> impl IntoResponse {

    /// Updates an existing Policy, If the Policy is not found, an error is reported
    ///
    /// # Example
    ///
    /// Request Body:
    /// {
    ///   "@context": {
    ///     "@vocab": "https://w3id.org/edc/v0.0.1/ns/"
    ///   },
    ///   "@id": "definition-id",
    ///   "policy": {
    ///     "@context": "http://www.w3.org/ns/odrl.jsonld",
    ///     "@type": "Set",
    ///     "uid": "http://example.com/policy:1010",
    ///     "permission": [
    ///       {
    ///         "target": "http://example.com/asset:9898.movie",
    ///         "action": "display",
    ///         "constraint": [
    ///           {
    ///             "leftOperand": "spatial",
    ///             "operator": "eq",
    ///             "rightOperand": "https://www.wikidata.org/wiki/Q183",
    ///             "comment": "i.e Germany"
    ///           }
    ///         ]
    ///       }
    ///     ]
    ///   }
    /// }
    ///
    /// Responses:
    /// 204 - Policy definition was updated successfully.
    /// 400 - Request was malformed, e.g. id was null
    /// 404 - Policy definition could not be updated, because it does not exist

    info!("Update Policy Definition called");

    if input.at_id.is_none() {
        return (StatusCode::BAD_REQUEST, Json(error!("Request was malformed, id was null"))).into_response();
    }

    let id = input.at_id.clone().unwrap();

    debug!("Received Policy Definition update for id {:#?}", id.clone());
    debug!("Update information: {:#?}", input.clone());

    let mut state = state.lock().await;

    if state.contains_key(&id) {
        let policy_definition = input2output(input.clone(), id.clone(), None).await;
        state.insert(id.clone(), policy_definition);
        StatusCode::NO_CONTENT.into_response()
    } else {
        (StatusCode::NOT_FOUND, Json(error!("Policy Definition could not be updated, because an Asset with the given id does not exist."))).into_response()
    }

}

pub(crate) async fn delete_policy_definition(State(state): State<SharedState>, Path(id): Path<String>,) -> impl IntoResponse {

    /// Removes a policy definition with the given ID if possible. Deleting a policy definition is only possible
    /// if that policy definition is not yet referenced by a contract definition, in which case an error is returned.
    /// DANGER ZONE: Note that deleting policy definitions can have unexpected results, do this at your own risk!
    ///
    /// # Example
    ///
    /// DELETE /v2/policydefinitions/{id}
    ///
    /// Parameter:
    /// id: String (required)  - The ID of the Asset
    ///
    /// Responses:
    /// 204 - Policy definition was deleted successfully
    /// 400 - Request was malformed, e.g. id was null
    /// 404 - A policy definition with the given ID does not exist
    /// 409 - The policy definition cannot be deleted, because it is referenced by a contract definition

    info!("Delete Policy Definition called");
    debug!("Received Policy Definition deletion request for Policy Definition with id: {:#?}", id.clone());

    let mut state = state.lock().await;

    let query = QuerySpec {
        filter_expression: vec![
            Criterion {
                at_type: None,
                operand_left: json!("accessPolicyId"),
                operand_right: json!(id.clone()),
                operator: "=".to_string(),
            },
            Criterion {
                at_type: None,
                operand_left: json!("contractPolicyId"),
                operand_right: json!(id.clone()),
                operator: "=".to_string(),
            },
        ],
        limit: Some(1), // It is only relevant to know if there is at least one contract definition referencing the policy definition, thus we limit the result to 1
        offset: None,
        sort_field: None,
        sort_order: None,
        at_context: None,
        at_type: None,
    };

    let url = "http://localhost:3000/v2/contractdefinitions/request";
    let http_client = Client::new();
    let response = http_client.post(url).json(&query).send().await;

    match response {
        Ok(resp) if resp.status().is_success() => {
            let contract_definitions: Vec<ContractDefinitionOutput> = resp.json().await.unwrap_or_default();

            // Check if there are any contract definitions referencing the policy definition
            if !contract_definitions.is_empty() {
                return (StatusCode::CONFLICT, Json(error!("The Policy Definition cannot be deleted because it is referenced by a Contract Definition."))).into_response();
            }
        }
        _ => {
            return (StatusCode::BAD_GATEWAY, Json(error!("Failed to verify Contract Definition references for the Policy Definition."))).into_response();
        }
    }

    if state.remove(&id).is_some() {
        StatusCode::NO_CONTENT.into_response()
    } else {
        (StatusCode::NOT_FOUND, Json(error!("Policy Definition could not be deleted, because a Policy Definition with the given id does not exist."))).into_response()
    }

}