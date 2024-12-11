use std::collections::HashMap;
use utoipa::ToSchema;
use odrl::model::action::Action;
use odrl::model::asset::Asset;
use odrl::model::party::Party;
use odrl::model::rule::{Permission, Rule};
use odrl::model::conflict_term::ConflictTerm;
use odrl::model::type_alias::IRI;


// A Policy MAY include an obligation to fulfil a Duty. The obligation is fulfilled if all constraints are satisfied and if its action, with all refinements satisfied, has been exercised.


// Validate required fields depending on provided type

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct Policy {

    #[serde(rename = "@context", skip_serializing_if = "Option::is_none")]
    pub context: Option<HashMap<String, serde_json::Value>>,
    #[serde(rename = "uid", skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
    #[serde(rename = "@type")]
    pub policy_type: String,
    #[serde(rename = "assigner", skip_serializing_if = "Option::is_none")]
    pub assigner: Option<Party>,
    #[serde(rename = "assignee", skip_serializing_if = "Option::is_none")]
    pub assignee: Option<Party>,
    #[serde(rename = "target", skip_serializing_if = "Option::is_none")]
    pub target: Option<Box<Asset>>,
    #[serde(rename = "action", skip_serializing_if = "Option::is_none")]
    pub action: Option<Action>,
    #[serde(skip_serializing)]
    pub rules: Vec<Rule>,
    #[serde(rename = "profile", skip_serializing_if = "Vec::is_empty")]
    pub profiles: Vec<IRI>,
    #[serde(rename = "inheritFrom", skip_serializing_if = "Vec::is_empty")]
    pub inherit_from: Vec<IRI>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflict: Option<ConflictTerm>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub obligation: Vec<Rule>,

}

impl Policy {

    pub fn new(context: Option<HashMap<String, serde_json::Value>>, uid: Option<String>, policy_type: String, assigner: Option<Party>, assignee: Option<Party>, target: Option<Box<Asset>>, action: Option<Action>, rules: Vec<Rule>, profiles: Vec<IRI>, inherit_from: Vec<IRI>, conflict: Option<ConflictTerm>, obligation: Vec<Rule>) -> Self {
        Policy {
            context,
            uid,
            policy_type,
            assigner,
            assignee,
            target,
            action,
            rules,
            profiles,
            inherit_from,
            conflict,
            obligation,
        }
    }

}

impl Default for Policy {
    fn default() -> Self {
        Policy {
            context: Some(HashMap::from([("@vocab".to_string(), serde_json::Value::String("https://w3id.org/edc/v0.0.1/ns/".to_string()))])),
            uid: Some("http://example.com/policy:1010".to_string()),
            policy_type: "Set".to_string(),
            assigner: None,
            assignee: None,
            target: None,
            action: None,
            rules: Vec::from([Rule::Permission(Permission {
                uid: None,
                action: Action {
                    name: "use".to_string(),
                    refinements: None,
                    included_in: None,
                    implies: vec![],
                },
                relation: None,
                function: vec![],
                failures: vec![],
                constraints: vec![],
                target: Asset {
                    context: None,
                    uid: Some("http://example.com/asset:9898.movie".to_string()),
                    edc_type: None,
                    part_of: vec![],
                    relation: None,
                    has_policy: None,
                    properties: None,
                    private_properties: None,
                    data_address: None,
                    created_at: None,
                },
                assigner: None,
                assignee: None,
                duties: vec![],
            })]),
            profiles: Vec::new(),
            inherit_from: Vec::new(),
            conflict: None,
            obligation: Vec::new(),
        }
    }
}