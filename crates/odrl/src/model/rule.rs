use utoipa::ToSchema;
use crate::model::action::Action;
use crate::model::asset::Asset;
use crate::model::party::Party;
use crate::model::constraint::Constraint;
use crate::model::type_alias::IRI;


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub enum Rule {

    Permission(Permission),
    Prohibition(Prohibition),
    Duty(Duty),
    Obligation(Obligation),

}

impl Default for Rule {
    fn default() -> Self {
        Rule::Permission(Permission::default())
    }
}


#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize, ToSchema)]
pub struct Permission {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<IRI>,
    pub action: Action,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relation: Option<Asset>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub function: Vec<Party>,
    #[serde(rename = "failure", skip_serializing_if = "Vec::is_empty")]
    pub failures: Vec<Rule>,
    #[serde(rename = "constraint", skip_serializing_if = "Vec::is_empty")]
    pub constraints: Vec<Constraint>,

    pub target: Asset,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigner: Option<Party>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee: Option<Party>,
    #[serde(rename = "duty", skip_serializing_if = "Vec::is_empty")]
    pub duties: Vec<Duty>,

}

impl Permission {

    pub fn new(uid: Option<IRI>, action: Action, relation: Option<Asset>, function: Vec<Party>, failures: Vec<Rule>, constraints: Vec<Constraint>, target: Asset, assigner: Option<Party>, assignee: Option<Party>, duties: Vec<Duty>) -> Self {
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
            duties
        }
    }

}


#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize, ToSchema)]
pub struct Prohibition {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<IRI>,
    pub action: Action,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relation: Option<Asset>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub function: Vec<Party>,
    #[serde(rename = "failure", skip_serializing_if = "Vec::is_empty")]
    pub failures: Vec<Rule>,
    #[serde(rename = "constraint", skip_serializing_if = "Vec::is_empty")]
    pub constraints: Vec<Constraint>,

    pub target: Asset,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigner: Option<Party>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee: Option<Party>,
    #[serde(rename = "remedy", skip_serializing_if = "Vec::is_empty")]
    pub remedies: Vec<Duty>,

}

impl Prohibition {

    pub fn new(uid: Option<IRI>, action: Action, relation: Option<Asset>, function: Vec<Party>, failures: Vec<Rule>, constraints: Vec<Constraint>, target: Asset, assigner: Option<Party>, assignee: Option<Party>, remedies: Vec<Duty>) -> Self {
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
            remedies
        }
    }
}


#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize, ToSchema)]
pub struct Duty {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<IRI>,
    pub action: Action,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relation: Option<Asset>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub function: Vec<Party>,
    #[serde(rename = "failure", skip_serializing_if = "Vec::is_empty")]
    pub failures: Vec<Rule>,
    #[serde(rename = "constraint", skip_serializing_if = "Vec::is_empty")]
    pub constraints: Vec<Constraint>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<Asset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigner: Option<Party>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee: Option<Party>,
    #[serde(rename = "consequence", skip_serializing_if = "Vec::is_empty")]
    pub consequences: Vec<Duty>,
    #[serde(skip_serializing)]
    pub pre_condition: Option<Vec<Duty>>,


}

impl Duty {

    pub fn new(uid: Option<IRI>, action: Action, relation: Option<Asset>, function: Vec<Party>, failures: Vec<Rule>, constraints: Vec<Constraint>, target: Option<Asset>, assigner: Option<Party>, assignee: Option<Party>, consequences: Vec<Duty>, pre_condition: Option<Vec<Duty>>) -> Self {
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
            pre_condition
        }
    }

}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize, ToSchema)]
pub struct Obligation {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<IRI>,
    pub target: Asset,
    pub assigner: Party,
    pub assignee: Party,
    pub action: Action,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub consequence: Vec<Duty>,

}

impl Obligation {

    pub fn new(uid: Option<IRI>, target: Asset, assigner: Party, assignee: Party, action: Action, consequence: Vec<Duty>) -> Self {
        Obligation {
            uid,
            target,
            assigner,
            assignee,
            action,
            consequence
        }
    }

}