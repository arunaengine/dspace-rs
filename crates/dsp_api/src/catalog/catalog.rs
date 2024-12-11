/*
 * Catalog [ACK]
 * Sent by:    Provider
 * Schema:     TTL Shape, JSON Schema
 *
 * The Catalog contains all Datasets which the requester shall see.
 */

use crate::catalog::{AbstractDataset, DataService, Dataset};
use serde_with::{formats::PreferMany, serde_as, OneOrMany};

#[serde_as]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Catalog {
    #[serde(flatten)]
    pub abstract_dataset: AbstractDataset,
    #[serde(rename = "@context")]
    pub context: std::collections::HashMap<String, serde_json::Value>,
    #[serde(rename = "@type")]
    pub dsp_type: String,
    #[serde(rename = "dcat:dataset", skip_serializing_if = "Option::is_none")]
    pub datasets: Option<Vec<Dataset>>,
    #[serde(rename = "dcat:service")]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    pub service: Vec<DataService>,
    #[serde(
        rename = "dspace:participantId",
        skip_serializing_if = "Option::is_none"
    )]
    pub participant_id: Option<String>,
    #[serde(rename = "foaf:homepage", skip_serializing_if = "Option::is_none")]
    pub homepage: Option<String>,
}

impl Catalog {
    pub fn new(
        abstract_dataset: AbstractDataset,
        context: std::collections::HashMap<String, serde_json::Value>,
        dsp_type: String,
        datasets: Option<Vec<Dataset>>,
        service: Vec<DataService>,
        participant_id: Option<String>,
        homepage: Option<String>,
    ) -> Catalog {
        Catalog {
            abstract_dataset,
            context,
            dsp_type,
            datasets,
            service,
            participant_id,
            homepage,
        }
    }
}
