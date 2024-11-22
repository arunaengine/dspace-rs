use std::collections::HashMap;
use std::sync::{Arc};
use tokio::sync::Mutex;
use axum::extract::State;
use axum::Json;
use axum::response::IntoResponse;
use edc_api::{PolicyDefinitionInput, PolicyDefinitionOutput};

type SharedState = Arc<Mutex<HashMap<String, PolicyDefinitionOutput>>>;

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

    unimplemented!()
}

pub(crate) async fn request_policy_definitions() -> impl IntoResponse {
    unimplemented!()
}

pub(crate) async fn get_policy_definition() -> impl IntoResponse {
    unimplemented!()
}

pub(crate) async fn update_policy_definition() -> impl IntoResponse {
    unimplemented!()
}

pub(crate) async fn delete_policy_definition() -> impl IntoResponse {
    unimplemented!()
}