use serde_derive::{Deserialize, Serialize};
use odrl::model::constraint::Constraint;
use odrl::model::constraint::LogicalConstraint;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Refinements {
    Constraints(Vec<Constraint>),
    LogicalConstraints(Vec<LogicalConstraint>),
}

impl Default for Refinements {
    fn default() -> Refinements {
        Refinements::Constraints(vec![])
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Action {
    #[serde(rename = "action")]
    pub name: String,
    #[serde(rename = "refinement", skip_serializing_if = "Option::is_none")]
    pub refinements: Option<Refinements>,
    #[serde(rename = "includedIn", skip_serializing_if = "Option::is_none")]
    pub included_in: Option<Box<Action>>, // Use Box to allow recursive type definition
    #[serde(skip_serializing_if = "Option::is_none")]
    pub implies: Option<Vec<Box<Action>>>,
}

impl Action {
    pub fn new(
        name: &str,
        refinements: Option<Refinements>,
        included_in: Option<Action>,
        implies: Option<Vec<Box<Action>>>,
    ) -> Action {
        Action {
            name: name.to_string(),
            refinements,
            included_in: match included_in {
                Some(action) => Some(Box::new(action)),
                None => None,
            },
            implies,
        }
    }

}
