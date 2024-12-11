use odrl::model::action::Action;
use odrl::model::asset::Asset;
use odrl::model::constraint::Constraint;
use odrl::model::party::Party;
use odrl::model::type_alias::IRI;
use serde::{Deserialize, Serialize};

use crate::generics::StringOrX;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Rule {
    Permission(Permission),
    Prohibition(Prohibition),
    Duty(Duty),
    Obligation(Obligation),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Permission {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<IRI>,
    pub action: StringOrX<Action>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relation: Option<Asset>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub function: Vec<Party>,
    #[serde(rename = "failure", skip_serializing_if = "Vec::is_empty")]
    pub failures: Vec<Rule>,
    #[serde(rename = "constraint", skip_serializing_if = "Vec::is_empty")]
    pub constraints: Vec<Constraint>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<StringOrX<Asset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigner: Option<StringOrX<Party>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee: Option<StringOrX<Party>>,
    #[serde(rename = "duty", skip_serializing_if = "Vec::is_empty")]
    pub duties: Vec<Duty>,
}

impl Permission {
    pub fn new(
        uid: Option<IRI>,
        action: StringOrX<Action>,
        relation: Option<Asset>,
        function: Vec<Party>,
        failures: Vec<Rule>,
        constraints: Vec<Constraint>,
        target: Option<StringOrX<Asset>>,
        assigner: Option<StringOrX<Party>>,
        assignee: Option<StringOrX<Party>>,
        duties: Vec<Duty>,
    ) -> Self {
        Permission {
            uid,
            action,
            relation,
            function,
            failures,
            constraints,
            target,
            assigner,
            assignee,
            duties,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Prohibition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<IRI>,
    pub action: StringOrX<Action>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relation: Option<Asset>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub function: Vec<Party>,
    #[serde(rename = "failure", skip_serializing_if = "Vec::is_empty")]
    pub failures: Vec<Rule>,
    #[serde(rename = "constraint", skip_serializing_if = "Vec::is_empty")]
    pub constraints: Vec<Constraint>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<StringOrX<Asset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigner: Option<StringOrX<Party>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee: Option<StringOrX<Party>>,
    #[serde(rename = "remedy", skip_serializing_if = "Vec::is_empty")]
    pub remedies: Vec<Duty>,
}

impl Prohibition {
    pub fn new(
        uid: Option<IRI>,
        action: StringOrX<Action>,
        relation: Option<Asset>,
        function: Vec<Party>,
        failures: Vec<Rule>,
        constraints: Vec<Constraint>,
        target: Option<StringOrX<Asset>>,
        assigner: Option<StringOrX<Party>>,
        assignee: Option<StringOrX<Party>>,
        remedies: Vec<Duty>,
    ) -> Self {
        Prohibition {
            uid,
            action,
            relation,
            function,
            failures,
            constraints,
            target,
            assigner,
            assignee,
            remedies,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Duty {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<IRI>,
    pub action: StringOrX<Action>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relation: Option<Asset>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub function: Vec<Party>,
    #[serde(rename = "failure", skip_serializing_if = "Vec::is_empty")]
    pub failures: Vec<Rule>,
    #[serde(rename = "constraint", skip_serializing_if = "Vec::is_empty")]
    pub constraints: Vec<Constraint>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<StringOrX<Asset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigner: Option<StringOrX<Party>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee: Option<StringOrX<Party>>,
    #[serde(rename = "consequence", skip_serializing_if = "Vec::is_empty")]
    pub consequences: Vec<Duty>,
    #[serde(skip_serializing)]
    pub pre_condition: Option<Vec<Duty>>,
}

impl Duty {
    pub fn new(
        uid: Option<IRI>,
        action: StringOrX<Action>,
        relation: Option<Asset>,
        function: Vec<Party>,
        failures: Vec<Rule>,
        constraints: Vec<Constraint>,
        target: Option<StringOrX<Asset>>,
        assigner: Option<StringOrX<Party>>,
        assignee: Option<StringOrX<Party>>,
        consequences: Vec<Duty>,
        pre_condition: Option<Vec<Duty>>,
    ) -> Self {
        Duty {
            uid,
            action,
            relation,
            function,
            failures,
            constraints,
            target,
            assigner,
            assignee,
            consequences,
            pre_condition,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Obligation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<IRI>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<StringOrX<Asset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigner: Option<StringOrX<Party>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee: Option<StringOrX<Party>>,
    pub action: StringOrX<Action>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub consequence: Vec<Duty>,
}

impl Obligation {
    pub fn new(
        uid: Option<IRI>,
        target: Option<StringOrX<Asset>>,
        assigner: Option<StringOrX<Party>>,
        assignee: Option<StringOrX<Party>>,
        action: StringOrX<Action>,
        consequence: Vec<Duty>,
    ) -> Self {
        Obligation {
            uid,
            target,
            assigner,
            assignee,
            action,
            consequence,
        }
    }
}
