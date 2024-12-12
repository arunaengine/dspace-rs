use serde_derive::{Deserialize, Serialize};
use odrl::model::constraint::Constraint;
use odrl::model::type_alias::IRI;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Function {
    #[serde(rename = "assigner")]
    Assigner,
    #[serde(rename = "assignee")]
    Assignee,
}

impl Default for Function {
    fn default() -> Function {
        Function::Assigner
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PartyType {
    #[serde(rename = "Party")]
    Party(Vec<String>),
    #[serde(rename = "PartyCollection")]
    PartyCollection(Vec<String>),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenericPartyType {
    Party(Party),
    PartyCollection(PartyCollection),
}

impl Default for PartyType {
    fn default() -> PartyType {
        PartyType::Party(vec!["Party".to_string(), "vcard:Individual".to_string()])
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Party {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<IRI>,
    #[serde(rename = "partOf", skip_serializing_if = "Option::is_none")]
    pub part_of: Option<Vec<PartyCollection>>,
    #[serde(skip_serializing, skip_serializing_if = "Option::is_none")]
    pub function: Option<Function>,
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub party_type: Option<PartyType>,
}

impl Party {
    pub fn new(
        uid: Option<IRI>,
        part_of: Option<Vec<PartyCollection>>,
        function: Option<Function>,
        party_type: Option<PartyType>,
    ) -> Party {
        Party {
            uid,
            part_of,
            function,
            party_type,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct PartyCollection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<IRI>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refinement: Option<Vec<Constraint>>,
}

impl PartyCollection {
    pub fn new(source: Option<IRI>, refinement: Option<Vec<Constraint>>) -> PartyCollection {
        PartyCollection { source, refinement }
    }
}
