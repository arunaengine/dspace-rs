use crate::model::type_alias::IRI;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LeftOperand {

    Literal(String),
    IRI(IRI),
    Reference(IRI),

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Operator {

    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RightOperand {

    Literal(String),
    IRI(IRI),
    Reference(IRI),

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Constraint {

    pub uid: Option<IRI>,
    pub left_operand: LeftOperand,
    pub operator: Operator,
    pub right_operand: RightOperand,
    pub data_type: Option<IRI>,
    pub unit: Option<IRI>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogicalOperator {

    Or, // at least one of the Constraints MUST be satisfied
    Xone, // only one, and not more, of the Constraints MUST be satisfied
    And, // all of the Constraints MUST be satisfied
    AndSequence, // all of the Constraints - in sequence - MUST be satisfied
    // Add other logical operators as needed
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogicalConstraint {

    pub uid: Option<IRI>,
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