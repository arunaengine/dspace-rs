use axum::extract::{Path, Query, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::Json;
use dsp_api::catalog::{Catalog, Dataset};
use edc_api::query_spec::SortOrder;
use edc_api::QuerySpec;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{debug, error, info};

type SharedState = Arc<Mutex<Catalog>>;

pub async fn get_catalog(
    State(state): State<SharedState>,
    Json(query): Json<Value>,
) -> impl IntoResponse {
    info!("Get catalog called");

    let state = state.lock().await;
    let catalog = state.clone();

    debug!("Received Catalog request with query {:#?}", query);

    // access the "dspace:filter" in query
    let query_spec = match query.get("dspace:filter") {
        Some(query_spec) => {
            let query_spec: QuerySpec = serde_json::from_value(query_spec.clone()).unwrap();
            Some(query_spec)
        }
        None => None,
    };

    if query_spec.is_some() {
        let query = query_spec.clone().unwrap();
        // collect all Dataset of the catalog and save them to output of type Vec<Dataset>
        let mut output: Vec<Dataset> = Vec::from(catalog.datasets.unwrap());

        let offset = query.offset.unwrap_or(0);
        let limit = query.limit.unwrap_or(i32::MAX);
        let sort_order = query.sort_order.unwrap_or(SortOrder::Asc);
        let sort_field = query.sort_field;
        let filter_expression = query.filter_expression;

        // Sort state hashmap by value for the given key (sort_field) and order (sort_order) if provided in the query and save the result in output
        // TODO: Only sorting by resource id is implemented. Implement sorting by other fields as well.
        if sort_field.is_some() {
            let sort_field = sort_field.unwrap();

            output.sort_by(|a: &Dataset, b: &Dataset| {
                let a_dataset = &a;
                let b_dataset = &b;

                let ordering = match sort_field.as_str() {
                    "resource" => a_dataset
                        .abstract_dataset
                        .resource
                        .id
                        .cmp(&b_dataset.abstract_dataset.resource.id),
                    _ => std::cmp::Ordering::Equal,
                };

                if sort_order == SortOrder::Asc {
                    ordering
                } else {
                    ordering.reverse()
                }
            });
        } else {
            output.sort_by(|a: &Dataset, b: &Dataset| {
                let a_dataset = &a;
                let b_dataset = &b;

                let ordering = a_dataset
                    .abstract_dataset
                    .resource
                    .id
                    .cmp(&b_dataset.abstract_dataset.resource.id);

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

        return (StatusCode::OK, Json(output)).into_response();
    }

    (StatusCode::OK, Json(catalog.clone())).into_response()
}

fn evaluate_condition(
    dataset: &Dataset,
    operand_left: &Value,
    operator: &str,
    operand_right: &Value,
) -> bool {
    let field_name = operand_left.as_str().unwrap_or("");

    // TODO: Implement evaluation of nested fields
    // TODO: Evaluate whether other field_names should be supported
    match field_name {
        "@id" => compare_values(
            dataset.abstract_dataset.resource.id.as_deref(),
            operator,
            operand_right.as_str(),
        ),
        "dcat:keyword" => {
            let keywords = dataset.abstract_dataset.resource.keywords.as_ref().unwrap();
            let mut bool = false;
            for keyword in keywords {
                if compare_values(Some(keyword.as_str()), operator, operand_right.as_str()) {
                    bool = true;
                }
            }
            bool
        }
        "dcat:theme" => {
            let themes = dataset.abstract_dataset.resource.themes.as_ref().unwrap();
            let mut bool = false;
            for theme in themes {
                if compare_values(Some(theme.id.as_str()), operator, operand_right.as_str()) {
                    bool = true;
                }
            }
            bool
        }
        "dct:conformsTo" => compare_values(
            dataset.abstract_dataset.resource.conforms_to.as_deref(),
            operator,
            operand_right.as_str(),
        ),
        "dct:creator" => compare_values(
            dataset.abstract_dataset.resource.creator.as_deref(),
            operator,
            operand_right.as_str(),
        ),
        "dct:description" => {
            let descriptions = dataset
                .abstract_dataset
                .resource
                .descriptions
                .as_ref()
                .unwrap();
            let mut bool = false;
            for description in descriptions {
                if compare_values(
                    Some(description.value.as_str()),
                    operator,
                    operand_right.as_str(),
                ) {
                    bool = true;
                }
            }
            bool
        }
        "dct:identifier" => compare_values(
            dataset.abstract_dataset.resource.identifier.as_deref(),
            operator,
            operand_right.as_str(),
        ),
        "dct:issued" => compare_values(
            dataset.abstract_dataset.resource.issued.as_deref(),
            operator,
            operand_right.as_str(),
        ),
        "dct:modified" => compare_values(
            dataset.abstract_dataset.resource.modified.as_deref(),
            operator,
            operand_right.as_str(),
        ),
        "dct:title" => compare_values(
            dataset.abstract_dataset.resource.title.as_deref(),
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

pub async fn get_dataset(
    State(state): State<SharedState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    info!("Get Dataset called");
    debug!("Received Dataset request for id {:#?}", id.clone());

    let state = state.lock().await;

    let datasets = state.clone().datasets.unwrap();
    for dataset in datasets {
        let dataset_id = dataset
            .clone()
            .abstract_dataset
            .resource
            .id
            .unwrap_or(String::new());
        if dataset_id == id {
            return (StatusCode::OK, Json(dataset.clone())).into_response();
        }
    }

    let err_msg = format!(
        "A Dataset with the given id {:#?} does not exist.",
        id.clone()
    );

    error!("{}", err_msg);

    (StatusCode::NOT_FOUND, Json(err_msg)).into_response()
}
