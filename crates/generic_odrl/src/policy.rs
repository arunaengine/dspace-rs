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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission: Option<Vec<Permission>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prohibition: Option<Vec<Prohibition>>,
    #[serde(rename = "profile", skip_serializing_if = "Option::is_none")]
    pub profiles: Option<StringOrX<Vec<IRI>>>,
    #[serde(rename = "inheritFrom", skip_serializing_if = "Option::is_none")]
    pub inherit_from: Option<Vec<IRI>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflict: Option<ConflictTerm>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obligation: Option<Vec<Obligation>>,
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
        permission: Option<Vec<Permission>>,
        prohibition: Option<Vec<Prohibition>>,
        profiles: Option<StringOrX<Vec<IRI>>>,
        inherit_from: Option<Vec<IRI>>,
        conflict: Option<ConflictTerm>,
        obligation: Option<Vec<Obligation>>,
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
    fn deserialize_example1() {
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
    }
    #[test]
    fn deserialize_example2() {
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

    #[test]
    fn deserialize_example3() {
        let example = r#"
{
    "@context": "http://www.w3.org/ns/odrl.jsonld",
    "@type": "Agreement",
    "uid": "http://example.com/policy:1012",
    "profile": "http://example.com/odrl:profile:01",
    "permission": [{
        "target": "http://example.com/asset:9898.movie",
        "assigner": "http://example.com/party:org:abc",
        "assignee": "http://example.com/party:person:billie",
        "action": "play"
    }]
} 
    "#;
        serde_json::from_str::<super::GenericPolicy>(example).unwrap();
    }
}
