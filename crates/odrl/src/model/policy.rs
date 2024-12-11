use crate::model::action::Action;
use crate::model::asset::Asset;
use crate::model::conflict_term::ConflictTerm;
use crate::model::party::Party;
use crate::model::rule::*;
use crate::model::type_alias::IRI;
use utoipa::ToSchema;

// A Policy MAY include an obligation to fulfil a Duty. The obligation is fulfilled if all constraints are satisfied and if its action, with all refinements satisfied, has been exercised.

/// Default Policy of type Set
#[derive(Debug, Clone, PartialEq, Eq, Default, Deserialize, ToSchema)]
pub struct SetPolicy {
    pub uid: IRI,
    #[serde(skip_serializing)]
    pub rules: Vec<Rule>, // will be serialized manually since rules are serialized as arrays of permissions, prohibitions and obligations
    #[serde(rename = "profile", skip_serializing_if = "Vec::is_empty")]
    pub profiles: Vec<IRI>,
    #[serde(rename = "inheritFrom", skip_serializing_if = "Vec::is_empty")]
    pub inherit_from: Vec<IRI>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflict: Option<ConflictTerm>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub obligation: Vec<Rule>,
    #[serde(rename = "target", skip_serializing_if = "Option::is_none")]
    pub target: Option<Box<Asset>>,
    #[serde(rename = "action", skip_serializing_if = "Option::is_none")]
    pub action: Option<Action>,
}

impl SetPolicy {
    pub fn new(
        uid: IRI,
        rules: Vec<Rule>,
        profiles: Vec<IRI>,
        inherit_from: Vec<IRI>,
        conflict: Option<ConflictTerm>,
        obligation: Vec<Rule>,
        target: Option<Box<Asset>>,
        action: Option<Action>,
    ) -> Self {
        SetPolicy {
            uid,
            rules,
            profiles,
            inherit_from,
            conflict,
            obligation,
            target,
            action,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Deserialize, ToSchema)]
pub struct OfferPolicy {
    pub uid: IRI,
    #[serde(skip_serializing)]
    pub assigner: Party, // will be serialized as part of the rule
    #[serde(skip_serializing)]
    pub rules: Vec<Rule>, // will be serialized manually since rules are serialized as arrays of permissions, prohibitions and obligations
    #[serde(rename = "profile", skip_serializing_if = "Vec::is_empty")]
    pub profiles: Vec<IRI>,
    #[serde(rename = "inheritFrom", skip_serializing_if = "Vec::is_empty")]
    pub inherit_from: Vec<IRI>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflict: Option<ConflictTerm>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub obligation: Vec<Rule>,
    #[serde(rename = "target", skip_serializing_if = "Option::is_none")]
    pub target: Option<Box<Asset>>,
    #[serde(rename = "action", skip_serializing_if = "Option::is_none")]
    pub action: Option<Action>,
}

impl OfferPolicy {
    pub fn new(
        uid: IRI,
        assigner: Party,
        rules: Vec<Rule>,
        profiles: Vec<IRI>,
        inherit_from: Vec<IRI>,
        conflict: Option<ConflictTerm>,
        obligation: Vec<Rule>,
        target: Option<Box<Asset>>,
        action: Option<Action>,
    ) -> Self {
        OfferPolicy {
            uid,
            assigner,
            rules,
            profiles,
            inherit_from,
            conflict,
            obligation,
            target,
            action,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Deserialize, ToSchema)]
pub struct AgreementPolicy {
    pub uid: IRI,
    #[serde(skip_serializing)]
    pub assigner: Party, // will be serialized as part of the rule
    #[serde(skip_serializing)]
    pub assignee: Party, // will be serialized as part of the rule
    #[serde(skip_serializing)]
    pub rules: Vec<Rule>, // will be serialized manually since rules are serialized as arrays of permissions, prohibitions and obligations
    #[serde(rename = "profile", skip_serializing_if = "Vec::is_empty")]
    pub profiles: Vec<IRI>,
    #[serde(rename = "inheritFrom", skip_serializing_if = "Vec::is_empty")]
    pub inherit_from: Vec<IRI>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflict: Option<ConflictTerm>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub obligation: Vec<Rule>,
    #[serde(rename = "target", skip_serializing_if = "Option::is_none")]
    pub target: Option<Box<Asset>>,
    #[serde(rename = "action", skip_serializing_if = "Option::is_none")]
    pub action: Option<Action>,
}

impl AgreementPolicy {
    pub fn new(
        uid: IRI,
        assigner: Party,
        assignee: Party,
        rules: Vec<Rule>,
        profiles: Vec<IRI>,
        inherit_from: Vec<IRI>,
        conflict: Option<ConflictTerm>,
        obligation: Vec<Rule>,
        target: Option<Box<Asset>>,
        action: Option<Action>,
    ) -> Self {
        AgreementPolicy {
            uid,
            assigner,
            assignee,
            rules,
            profiles,
            inherit_from,
            conflict,
            obligation,
            target,
            action,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub enum Policy {
    SetPolicy(SetPolicy),
    OfferPolicy(OfferPolicy),
    AgreementPolicy(AgreementPolicy),
}

impl Default for Policy {
    fn default() -> Self {
        // Default to SetPolicy
        Policy::SetPolicy(SetPolicy::default())
    }
}
