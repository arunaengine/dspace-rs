use utoipa::ToSchema;
use crate::model::constraint::Constraint;
use crate::model::constraint::LogicalConstraint;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub enum Refinements {
    Constraints(Vec<Constraint>),
    LogicalConstraints(Vec<LogicalConstraint>),
}

impl Default for Refinements {
    fn default() -> Refinements {
        Refinements::Constraints(vec![])
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize, ToSchema)]
pub struct Action {

    #[serde(rename = "action")]
    pub name: String,
    #[serde(rename = "refinement", skip_serializing_if = "Option::is_none")]
    pub refinements: Option<Refinements>,
    #[serde(rename = "includedIn", skip_serializing_if = "Option::is_none")]
    pub included_in: Option<Box<Action>>, // Use Box to allow recursive type definition
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub implies: Vec<Box<Action>>,

}

impl Action {

    pub fn new(name: &str, refinements: Option<Refinements>, included_in: Option<Action>, implies: Vec<Action>) -> Action {
        Action {
            name: name.to_string(),
            refinements,
            included_in: match included_in {
                Some(action) => Some(Box::new(action)),
                None => None,
            },
            implies: implies.into_iter().map(Box::new).collect(),
        }
    }

    // Function to create the two top level actions "use" and "transfer"
    pub fn init_top_level() -> (Action, Action) {
        let use_action = Action {
            name: "use".to_string(),
            refinements: None,
            included_in: None,
            implies: vec![],
        };
        let transfer_action = Action {
            name: "transfer".to_string(),
            refinements: None,
            included_in: None,
            implies: vec![],
        };

        (use_action, transfer_action)
    }

}
