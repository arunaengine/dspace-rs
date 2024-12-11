use utoipa::ToSchema;
use crate::model::type_alias::IRI;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
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

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize, ToSchema)]
pub struct Constraint {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<IRI>,
    #[serde(rename = "leftOperand")]
    pub left_operand: LeftOperand,
    pub operator: Operator,
    #[serde(rename = "rightOperand")]
    pub right_operand: RightOperand,
    #[serde(rename = "dataType", skip_serializing_if = "Option::is_none")]
    pub data_type: Option<IRI>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<IRI>,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub status: String,

}

impl Constraint {

    pub fn new(
        uid: Option<IRI>,
        left_operand: LeftOperand,
        operator: Operator,
        right_operand: RightOperand,
        data_type: Option<IRI>,
        unit: Option<IRI>,
        status: String,
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
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

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize, ToSchema)]
pub struct LogicalConstraint {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<IRI>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operand: Option<(LogicalOperator, Vec<IRI>)>,

}

impl LogicalConstraint {


    pub fn new(uid: Option<IRI>, operand: Option<(LogicalOperator, Vec<IRI>)>) -> LogicalConstraint {
        LogicalConstraint {
            uid,
            operand
        }
    }

}