use crate::model::constraint::Constraint;
use crate::model::type_alias::IRI;
use utoipa::ToSchema;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub enum PartyType {
    #[serde(rename = "Party")]
    Party(Vec<String>),
    #[serde(rename = "PartyCollection")]
    PartyCollection(Vec<String>),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
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

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize, ToSchema)]
pub struct Party {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<IRI>,
    #[serde(rename = "partOf", skip_serializing_if = "Vec::is_empty")]
    pub part_of: Vec<PartyCollection>,
    #[serde(skip_serializing)]
    pub function: Function,
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub party_type: Option<PartyType>,
}

impl Party {
    pub fn new(
        uid: Option<IRI>,
        part_of: Vec<PartyCollection>,
        function: Function,
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

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize, ToSchema)]
pub struct PartyCollection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<IRI>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub refinement: Vec<Constraint>,
}

impl PartyCollection {
    pub fn new(source: Option<IRI>, refinement: Vec<Constraint>) -> PartyCollection {
        PartyCollection { source, refinement }
    }
}
