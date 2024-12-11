use crate::{
    generic_rule::{Obligation, Permission, Prohibition, Rule},
    generics::StringOrX,
};
use odrl::model::action::Action;
use odrl::model::asset::Asset;
use odrl::model::conflict_term::ConflictTerm;
use odrl::model::party::Party;
use odrl::model::type_alias::IRI;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// A Policy MAY include an obligation to fulfil a Duty. The obligation is fulfilled if all constraints are satisfied and if its action, with all refinements satisfied, has been exercised.

// Validate required fields depending on provided type

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GenericPolicy {
    #[serde(rename = "@context", skip_serializing_if = "Option::is_none")]
    pub context: Option<StringOrX<HashMap<String, serde_json::Value>>>,
    #[serde(rename = "uid", skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
    #[serde(rename = "@type")]
    pub policy_type: String,
    #[serde(rename = "assigner", skip_serializing_if = "Option::is_none")]
    pub assigner: Option<StringOrX<Party>>,
    #[serde(rename = "assignee", skip_serializing_if = "Option::is_none")]
    pub assignee: Option<StringOrX<Party>>,
    #[serde(rename = "target", skip_serializing_if = "Option::is_none")]
    pub target: Option<StringOrX<Box<Asset>>>,
    #[serde(rename = "action", skip_serializing_if = "Option::is_none")]
    pub action: Option<StringOrX<Action>>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub permission: Vec<Permission>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub prohibition: Vec<Prohibition>,
    #[serde(rename = "profile", skip_serializing_if = "Vec::is_empty")]
    pub profiles: Vec<IRI>,
    #[serde(rename = "inheritFrom", skip_serializing_if = "Vec::is_empty")]
    pub inherit_from: Vec<IRI>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflict: Option<ConflictTerm>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub obligation: Vec<Obligation>,
}

impl GenericPolicy {
    pub fn new(
        context: Option<StringOrX<HashMap<String, serde_json::Value>>>,
        uid: Option<String>,
        policy_type: String,
        assigner: Option<StringOrX<Party>>,
        assignee: Option<StringOrX<Party>>,
        target: Option<StringOrX<Box<Asset>>>,
        action: Option<StringOrX<Action>>,
        permission: Vec<Permission>,
        prohibition: Vec<Prohibition>,
        profiles: Vec<IRI>,
        inherit_from: Vec<IRI>,
        conflict: Option<ConflictTerm>,
        obligation: Vec<Obligation>,
    ) -> Self {
        GenericPolicy {
            context,
            uid,
            policy_type,
            assigner,
            assignee,
            target,
            action,
            permission,
            prohibition,
            profiles,
            inherit_from,
            conflict,
            obligation,
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    #[test]
    fn deserialize_examples() {
        let example1 = r#"
            {
    "@context": "http://www.w3.org/ns/odrl.jsonld",
    "@type": "Set",
    "uid": "http://example.com/policy:1010",
    "permission": [{
        "target": "http://example.com/asset:9898.movie",
        "action": "use"
    }]
}       
        "#;

        serde_json::from_str::<super::GenericPolicy>(example1).unwrap();

        let example2 = r#"
    {
    "@context": "http://www.w3.org/ns/odrl.jsonld",
    "@type": "Offer",
    "uid": "http://example.com/policy:1011",
    "profile": "http://example.com/odrl:profile:01",
    "permission": [{
        "target": "http://example.com/asset:9898.movie",
        "assigner": "http://example.com/party:org:abc",
        "action": "play"
    }]
}   
    "#;
        serde_json::from_str::<super::GenericPolicy>(example2).unwrap();
    }
}
