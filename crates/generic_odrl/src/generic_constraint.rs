use serde_derive::{Deserialize, Serialize};
use odrl::model::type_alias::IRI;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LeftOperand {
    Literal(String),
    IRI(IRI),
    Reference(IRI),
}

impl Default for LeftOperand {
    fn default() -> Self {
        LeftOperand::Literal("".to_string())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Operator {
    #[serde(rename = "eq")]
    Equal,
    #[serde(rename = "neq")]
    NotEqual,
    #[serde(rename = "gt")]
    GreaterThan,
    #[serde(rename = "lt")]
    LessThan,
    #[serde(rename = "gteq")]
    GreaterThanOrEqual,
    #[serde(rename = "lteq")]
    LessThanOrEqual,
    #[serde(rename = "hasPart")]
    HasPart,
    #[serde(rename = "isA")]
    IsA,
    #[serde(rename = "isAllOf")]
    IsAllOf,
    #[serde(rename = "isAnyOf")]
    IsAnyOf,
    #[serde(rename = "isNoneOf")]
    IsNoneOf,
    #[serde(rename = "isPartOf")]
    IsPartOf,
}

impl Default for Operator {
    fn default() -> Self {
        Operator::Equal
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RightOperand {
    Literal(String),
    IRI(IRI),
    Reference(IRI),
}

impl Default for RightOperand {
    fn default() -> Self {
        RightOperand::Literal("".to_string())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Constraint {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<IRI>,
    #[serde(rename = "leftOperand", skip_serializing_if = "Option::is_none")]
    pub left_operand: Option<LeftOperand>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator: Option<Operator>,
    #[serde(rename = "rightOperand", skip_serializing_if = "Option::is_none")]
    pub right_operand: Option<RightOperand>,
    #[serde(rename = "dataType", skip_serializing_if = "Option::is_none")]
    pub data_type: Option<IRI>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<IRI>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

impl Constraint {
    pub fn new(
        uid: Option<IRI>,
        left_operand: Option<LeftOperand>,
        operator: Option<Operator>,
        right_operand: Option<RightOperand>,
        data_type: Option<IRI>,
        unit: Option<IRI>,
        status: Option<String>,
    ) -> Constraint {
        Constraint {
            uid,
            left_operand,
            operator,
            right_operand,
            data_type,
            unit,
            status,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LogicalOperator {
    #[serde(rename = "or")]
    Or, // at least one of the Constraints MUST be satisfied
    #[serde(rename = "xone")]
    Xone, // only one, and not more, of the Constraints MUST be satisfied
    #[serde(rename = "and")]
    And, // all of the Constraints MUST be satisfied
    #[serde(rename = "andSequence")]
    AndSequence, // all of the Constraints - in sequence - MUST be satisfied
    // Add other logical operators as needed
}

impl Default for LogicalOperator {
    fn default() -> Self {
        LogicalOperator::Or
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct LogicalConstraint {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<IRI>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operand: Option<(LogicalOperator, Vec<IRI>)>,
}

impl LogicalConstraint {
    pub fn new(
        uid: Option<IRI>,
        operand: Option<(LogicalOperator, Vec<IRI>)>,
    ) -> LogicalConstraint {
        LogicalConstraint { uid, operand }
    }
}
