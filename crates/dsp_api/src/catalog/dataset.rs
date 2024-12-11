/*
 * Dataset [ACK]
 * Sent by:    Provider
 * Schema:     TTL Shape, JSON Schema
 *
 * The Catalog contains all Datasets which the requester shall see.
 */

use crate::contract_negotiation::Offer;
use serde_with::{formats::PreferMany, serde_as, OneOrMany};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Dataset {
    #[serde(flatten)]
    pub abstract_dataset: AbstractDataset,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AbstractDataset {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(rename = "odrl:hasPolicy", skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<Offer>>,
    #[serde(rename = "dcat:distribution", skip_serializing_if = "Option::is_none")]
    pub distributions: Option<Vec<Distribution>>,
}

#[serde_as]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "dcat:keyword", skip_serializing_if = "Option::is_none")]
    pub keywords: Option<Vec<String>>,
    #[serde(rename = "dcat:theme", skip_serializing_if = "Option::is_none")]
    pub themes: Option<Vec<Reference>>,
    #[serde(rename = "dct:conformsTo", skip_serializing_if = "Option::is_none")]
    pub conforms_to: Option<String>,
    #[serde(rename = "dct:creator", skip_serializing_if = "Option::is_none")]
    pub creator: Option<String>,
    #[serde(rename = "dct:description", skip_serializing_if = "Option::is_none")]
    pub descriptions: Option<Vec<MultiLanguage>>,
    #[serde(rename = "dct:identifier", skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    #[serde(rename = "dct:issued", skip_serializing_if = "Option::is_none")]
    pub issued: Option<String>,
    #[serde(rename = "dct:modified", skip_serializing_if = "Option::is_none")]
    pub modified: Option<String>,
    #[serde(rename = "dct:title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[serde_as]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Distribution {
    #[serde(rename = "dct:title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "dct:description", skip_serializing_if = "Vec::is_empty")]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    pub descriptions: Vec<MultiLanguage>,
    #[serde(rename = "dct:issued", skip_serializing_if = "Option::is_none")]
    pub issued: Option<String>,
    #[serde(rename = "dct:modified", skip_serializing_if = "Option::is_none")]
    pub modified: Option<String>,
    #[serde(rename = "odrl:hasPolicy")]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    pub policy: Vec<Offer>,
    #[serde(rename = "dcat:accessService")]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    pub access_services: Vec<DataService>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataService {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(
        rename = "dcat:endpointDescription",
        skip_serializing_if = "Option::is_none"
    )]
    pub endpoint_description: Option<String>,
    #[serde(rename = "dcat:endpointURL", skip_serializing_if = "Option::is_none")]
    pub endpoint_url: Option<String>,
    #[serde(rename = "dcat:servesDataset", skip_serializing_if = "Option::is_none")]
    pub serves_datasets: Option<Vec<Dataset>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Reference {
    #[serde(rename = "@id")]
    pub id: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MultiLanguage {
    #[serde(rename = "@value")]
    pub value: String,
    #[serde(rename = "@language")]
    pub language: String,
}

impl Dataset {
    pub fn new(abstract_dataset: AbstractDataset) -> Dataset {
        Dataset { abstract_dataset }
    }
}

impl AbstractDataset {
    pub fn new(
        resource: Resource,
        policies: Option<Vec<Offer>>,
        distributions: Option<Vec<Distribution>>,
    ) -> AbstractDataset {
        AbstractDataset {
            resource,
            policies,
            distributions,
        }
    }
}

impl Resource {
    pub fn new(
        id: Option<String>,
        keywords: Option<Vec<String>>,
        themes: Option<Vec<Reference>>,
        conforms_to: Option<String>,
        creator: Option<String>,
        descriptions: Option<Vec<MultiLanguage>>,
        identifier: Option<String>,
        issued: Option<String>,
        modified: Option<String>,
        title: Option<String>,
    ) -> Resource {
        Resource {
            id,
            keywords,
            themes,
            conforms_to,
            creator,
            descriptions,
            identifier,
            issued,
            modified,
            title,
        }
    }
}

impl Distribution {
    pub fn new(
        title: Option<String>,
        descriptions: Vec<MultiLanguage>,
        issued: Option<String>,
        modified: Option<String>,
        policy: Vec<Offer>,
        access_services: Vec<DataService>,
    ) -> Distribution {
        Distribution {
            title,
            descriptions,
            issued,
            modified,
            policy,
            access_services,
        }
    }
}

impl DataService {
    pub fn new(
        resource: Resource,
        endpoint_description: Option<String>,
        endpoint_url: Option<String>,
        serves_datasets: Option<Vec<Dataset>>,
    ) -> DataService {
        DataService {
            resource,
            endpoint_description,
            endpoint_url,
            serves_datasets,
        }
    }
}

impl Reference {
    pub fn new(id: String) -> Reference {
        Reference { id }
    }
}

impl MultiLanguage {
    pub fn new(value: String, language: String) -> MultiLanguage {
        MultiLanguage { value, language }
    }
}
