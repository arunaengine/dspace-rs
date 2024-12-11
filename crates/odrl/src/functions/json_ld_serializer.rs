use sophia::api::prelude::*;
use sophia::inmem::graph::LightGraph;
use sophia::iri::IriRef;
use sophia::turtle::serializer::nt::NtSerializer;
use sophia_jsonld::*;

use crate::{model, name_spaces};
use model::action::*;
use model::conflict_term::*;
use model::constraint::*;
use model::policy::*;
use model::rule::*;

use serde_json::Value;

pub trait Serializable {
    fn add_to_graph(&self, graph: &mut LightGraph) -> Result<(), Box<dyn std::error::Error>>;
}

impl Serializable for Policy {
    fn add_to_graph(&self, graph: &mut LightGraph) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Policy::SetPolicy(set_policy) => set_policy.add_to_graph(graph),
            Policy::OfferPolicy(offer_policy) => offer_policy.add_to_graph(graph),
            Policy::AgreementPolicy(agreement_policy) => agreement_policy.add_to_graph(graph),
        }
        .expect("Error adding policy to graph since its type is not supported");

        Ok(())
    }
}

impl Serializable for SetPolicy {
    fn add_to_graph(&self, graph: &mut LightGraph) -> Result<(), Box<dyn std::error::Error>> {
        let set_policy_type = IriRef::new(format!("{}type", name_spaces::RDF_NS))?;
        let set_policy_uid = self.uid.clone();
        let set_policy_term = IriRef::new(set_policy_uid)?;

        // Insert triples for SetPolicy
        graph.insert(
            &set_policy_term,
            &set_policy_type,
            &IriRef::new(format!("{}Set", name_spaces::LD_NS))?,
        )?; // Type
        graph.insert(
            &set_policy_term,
            &IriRef::new(format!("{}uid", name_spaces::LD_NS))?,
            &set_policy_term,
        )?; // UID

        // Insert other triples for fields:
        //      rules: Vec<Rule>,
        //      profiles: Vec<IRI>,
        //      inherit_from: Vec<IRI>,
        //      conflict: Option<ConflictTerm>,
        //      obligation: Option<Vec<Obligation>>,
        for rule in &self.rules {
            rule.add_to_graph(graph)?;
            // Rules can be of type Permission, Prohibition, or Duty, therefore their insertion will be handled by the add_to_graph() method.
        }
        /* Profiles and inherit_from are represented as IRIs, therefore can be directly added to the graph as object values in triples. */
        for profile in &self.profiles {
            graph.insert(
                &set_policy_term,
                &IriRef::new(format!("{}{}", name_spaces::LD_NS, "profile"))?,
                &IriRef::new(profile.to_string())?,
            )?;
        }
        for inherit in &self.inherit_from {
            graph.insert(
                &set_policy_term,
                &IriRef::new(format!("{}{}", name_spaces::LD_NS, "inheritFrom"))?,
                &IriRef::new(inherit.to_string())?,
            )?;
        }
        /* Since ConcflictTerms and Obligations are more complex structures, they may require more than just the IRI to be represented in the graph.
        Therefore, their insertion will be handled by the add_to_graph() method. */
        if let Some(conflict) = &self.conflict {
            match conflict {
                ConflictTerm::Perm => graph.insert(
                    &set_policy_term,
                    &IriRef::new(format!("{}conflict", name_spaces::LD_NS))?,
                    &IriRef::new(format!("{}perm", name_spaces::LD_NS))?,
                )?,
                ConflictTerm::Prohibit => graph.insert(
                    &set_policy_term,
                    &IriRef::new(format!("{}conflict", name_spaces::LD_NS))?,
                    &IriRef::new(format!("{}prohibit", name_spaces::LD_NS))?,
                )?,
                ConflictTerm::Invalid => graph.insert(
                    &set_policy_term,
                    &IriRef::new(format!("{}conflict", name_spaces::LD_NS))?,
                    &IriRef::new(format!("{}invalid", name_spaces::LD_NS))?,
                )?,
            };
        }
        for obligation in &self.obligation {
            obligation.add_to_graph(graph)?;
        }
        Ok(())
    }
}

impl Serializable for OfferPolicy {
    fn add_to_graph(&self, graph: &mut LightGraph) -> Result<(), Box<dyn std::error::Error>> {
        let offer_policy_type = IriRef::new(format!("{}type", name_spaces::RDF_NS))?;
        let offer_policy_uid = self.uid.clone();
        let offer_policy_term = IriRef::new(offer_policy_uid)?;

        // Insert triples for OfferPolicy
        graph.insert(
            &offer_policy_term,
            &offer_policy_type,
            &IriRef::new(format!("{}Offer", name_spaces::LD_NS))?,
        )?; // Type
        graph.insert(
            &offer_policy_term,
            &IriRef::new(format!("{}uid", name_spaces::LD_NS))?,
            &offer_policy_term,
        )?; // UID

        // Insert other triples for fields:
        //      assigner: Party,
        //      rules: Vec<Rule>,
        //      profiles: Vec<IRI>,
        //      inherit_from: Vec<IRI>,
        //      conflict: Option<ConflictTerm>,
        //      obligation: Option<Vec<Obligation>>
        let assigner_term = IriRef::new(self.assigner.uid.clone().unwrap())?;
        graph.insert(
            &offer_policy_term,
            &IriRef::new(format!("{}assigner", name_spaces::LD_NS))?,
            &assigner_term,
        )?; // Assigner
        for rule in &self.rules {
            rule.add_to_graph(graph)?;
            // Rules can be of type Permission, Prohibition, or Duty, therefore their insertion will be handled by the add_to_graph() method.
        }
        /* Profiles and inherit_from are represented as IRIs, therefore can be directly added to the graph as object values in triples. */
        for profile in &self.profiles {
            graph.insert(
                &offer_policy_term,
                &IriRef::new(format!("{}{}", name_spaces::LD_NS, "profile"))?,
                &IriRef::new(profile.to_string())?,
            )?;
        }
        for inherit in &self.inherit_from {
            graph.insert(
                &offer_policy_term,
                &IriRef::new(format!("{}{}", name_spaces::LD_NS, "inheritFrom"))?,
                &IriRef::new(inherit.to_string())?,
            )?;
        }
        /* Since ConcflictTerms and Obligations are more complex structures, they may require more than just the IRI to be represented in the graph.
        Therefore, their insertion will be handled by the add_to_graph() method. */
        if let Some(conflict) = &self.conflict {
            match conflict {
                ConflictTerm::Perm => graph.insert(
                    &offer_policy_term,
                    &IriRef::new(format!("{}conflict", name_spaces::LD_NS))?,
                    &IriRef::new(format!("{}perm", name_spaces::LD_NS))?,
                )?,
                ConflictTerm::Prohibit => graph.insert(
                    &offer_policy_term,
                    &IriRef::new(format!("{}conflict", name_spaces::LD_NS))?,
                    &IriRef::new(format!("{}prohibit", name_spaces::LD_NS))?,
                )?,
                ConflictTerm::Invalid => graph.insert(
                    &offer_policy_term,
                    &IriRef::new(format!("{}conflict", name_spaces::LD_NS))?,
                    &IriRef::new(format!("{}invalid", name_spaces::LD_NS))?,
                )?,
            };
        }
        for obligation in &self.obligation {
            obligation.add_to_graph(graph)?;
        }
        Ok(())
    }
}

impl Serializable for AgreementPolicy {
    fn add_to_graph(&self, graph: &mut LightGraph) -> Result<(), Box<dyn std::error::Error>> {
        let agreement_policy_type = IriRef::new(format!("{}type", name_spaces::RDF_NS))?;
        let agreement_policy_uid = self.uid.clone();
        let agreement_policy_term = IriRef::new(agreement_policy_uid)?;

        // Insert triples for OfferPolicy
        graph.insert(
            &agreement_policy_term,
            &agreement_policy_type,
            &IriRef::new(format!("{}Agreement", name_spaces::LD_NS))?,
        )?; // Type
        graph.insert(
            &agreement_policy_term,
            &IriRef::new(format!("{}uid", name_spaces::LD_NS))?,
            &agreement_policy_term,
        )?; // UID

        // Insert other triples for fields:
        //      assigner: Party,
        //      assignee: Party,
        //      rules: Vec<Rule>,
        //      profiles: Vec<IRI>,
        //      inherit_from: Vec<IRI>,
        //      conflict: Option<ConflictTerm>,
        //      obligation: Option<Vec<Obligation>>
        let assigner_term = IriRef::new(self.assigner.uid.clone().unwrap())?;
        graph.insert(
            &agreement_policy_term,
            &IriRef::new(format!("{}assigner", name_spaces::LD_NS))?,
            &assigner_term,
        )?; // Assigner
        let assignee_term = IriRef::new(self.assignee.uid.clone().unwrap())?;
        graph.insert(
            &agreement_policy_term,
            &IriRef::new(format!("{}assignee", name_spaces::LD_NS))?,
            &assignee_term,
        )?; // Assignee

        for rule in &self.rules {
            rule.add_to_graph(graph)?;
            // Rules can be of type Permission, Prohibition, or Duty, therefore their insertion will be handled by the add_to_graph() method.
        }
        /* Profiles and inherit_from are represented as IRIs, therefore can be directly added to the graph as object values in triples. */
        for profile in &self.profiles {
            graph.insert(
                &agreement_policy_term,
                &IriRef::new(format!("{}{}", name_spaces::LD_NS, "profile"))?,
                &IriRef::new(profile.to_string())?,
            )?;
        }
        for inherit in &self.inherit_from {
            graph.insert(
                &agreement_policy_term,
                &IriRef::new(format!("{}{}", name_spaces::LD_NS, "inheritFrom"))?,
                &IriRef::new(inherit.to_string())?,
            )?;
        }
        /* Since ConcflictTerms and Obligations are more complex structures, they may require more than just the IRI to be represented in the graph.
        Therefore, their insertion will be handled by the add_to_graph() method. */
        if let Some(conflict) = &self.conflict {
            match conflict {
                ConflictTerm::Perm => graph.insert(
                    &agreement_policy_term,
                    &IriRef::new(format!("{}conflict", name_spaces::LD_NS))?,
                    &IriRef::new(format!("{}perm", name_spaces::LD_NS))?,
                )?,
                ConflictTerm::Prohibit => graph.insert(
                    &agreement_policy_term,
                    &IriRef::new(format!("{}conflict", name_spaces::LD_NS))?,
                    &IriRef::new(format!("{}prohibit", name_spaces::LD_NS))?,
                )?,
                ConflictTerm::Invalid => graph.insert(
                    &agreement_policy_term,
                    &IriRef::new(format!("{}conflict", name_spaces::LD_NS))?,
                    &IriRef::new(format!("{}invalid", name_spaces::LD_NS))?,
                )?,
            };
        }
        for obligation in &self.obligation {
            obligation.add_to_graph(graph)?;
        }
        Ok(())
    }
}

impl Serializable for Rule {
    fn add_to_graph(&self, graph: &mut LightGraph) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Rule::Permission(permission) => permission.add_to_graph(graph),
            Rule::Prohibition(prohibition) => prohibition.add_to_graph(graph),
            Rule::Duty(duty) => duty.add_to_graph(graph),
            Rule::Obligation(obligation) => obligation.add_to_graph(graph),
        }
        .expect("Error adding rule to graph since its type is not supported");
        Ok(())
    }
}

impl Serializable for Permission {
    fn add_to_graph(&self, graph: &mut LightGraph) -> Result<(), Box<dyn std::error::Error>> {
        let permission_term;
        if let Some(uid) = &self.uid {
            permission_term = IriRef::new(uid.clone())?;
        } else {
            permission_term = IriRef::new(format!("{}Permission", name_spaces::LD_NS))?;
        }

        let target_uid = self.target.uid.clone().unwrap();
        let target_term = IriRef::new(target_uid)?;
        let action_name = self.action.name.clone();

        // Insert triples for permission
        graph.insert(
            &permission_term,
            IriRef::new(format!("{}type", name_spaces::RDF_NS))?,
            &IriRef::new(format!("{}permission", name_spaces::LD_NS))?,
        )?; // Type
        graph.insert(
            &permission_term,
            &IriRef::new(format!("{}target", name_spaces::LD_NS))?,
            &target_term,
        )?; // Target
        graph.insert(
            &permission_term,
            &IriRef::new(format!("{}action", name_spaces::LD_NS))?,
            &IriRef::new(format!("{}{}", name_spaces::ODRL_NS, action_name))?,
        )?; // Action

        // Action Refinement
        if let Some(refinements) = &self.action.refinements {
            let refinement_term = IriRef::new(format!("{}refinement", name_spaces::LD_NS))?;
            graph.insert(
                &IriRef::new(format!("{}action", name_spaces::LD_NS))?,
                &IriRef::new(format!("{}hasRefinement", name_spaces::LD_NS))?,
                &refinement_term,
            )?;

            match refinements {
                Refinements::Constraints(constraints) => {
                    for constraint in constraints {
                        // LeftOperand
                        match &constraint.left_operand {
                            LeftOperand::IRI(iri) => {
                                let left_operand_term = IriRef::new(iri.clone())?;
                                graph.insert(
                                    &refinement_term,
                                    &IriRef::new(format!("{}leftOperand", name_spaces::LD_NS))?,
                                    &left_operand_term,
                                )?;
                            }
                            LeftOperand::Literal(literal) => {
                                // For literal left operands, you may need to handle data type and other attributes
                                let datatype_term = match &constraint.data_type {
                                    Some(datatype) => IriRef::new(datatype.clone())?,
                                    None => IriRef::new("xsd:string".to_string())?, // Assuming default data type is string
                                };
                                // Insert triple for data type
                                graph.insert(
                                    &IriRef::new(format!("{}leftOperand", name_spaces::LD_NS))?,
                                    &IriRef::new(format!("{}type", name_spaces::RDF_NS))?,
                                    &datatype_term,
                                )?;

                                // Insert triple for literal value
                                graph.insert(
                                    &IriRef::new(format!("{}leftOperand", name_spaces::LD_NS))?,
                                    &IriRef::new(format!("{}value", name_spaces::RDF_NS))?,
                                    &IriRef::new(literal.clone())?,
                                )?;
                            }
                            LeftOperand::Reference(iri) => {
                                let left_operand_term = IriRef::new(iri.clone())?;
                                graph.insert(
                                    &refinement_term,
                                    &IriRef::new(format!("{}leftOperand", name_spaces::LD_NS))?,
                                    &left_operand_term,
                                )?;
                            }
                        }

                        // Operator
                        let operator_term = match constraint.operator {
                            Operator::Equal => IriRef::new(format!("{}eq", name_spaces::ODRL_NS))?,
                            Operator::NotEqual => {
                                IriRef::new(format!("{}neq", name_spaces::ODRL_NS))?
                            }
                            Operator::GreaterThan => {
                                IriRef::new(format!("{}gt", name_spaces::ODRL_NS))?
                            }
                            Operator::LessThan => {
                                IriRef::new(format!("{}lt", name_spaces::ODRL_NS))?
                            }
                            Operator::GreaterThanOrEqual => {
                                IriRef::new(format!("{}gteq", name_spaces::ODRL_NS))?
                            }
                            Operator::LessThanOrEqual => {
                                IriRef::new(format!("{}lteq", name_spaces::ODRL_NS))?
                            }
                            Operator::HasPart => {
                                IriRef::new(format!("{}hasPart", name_spaces::ODRL_NS))?
                            }
                            Operator::IsA => IriRef::new(format!("{}isA", name_spaces::ODRL_NS))?,
                            Operator::IsAllOf => {
                                IriRef::new(format!("{}isAllOf", name_spaces::ODRL_NS))?
                            }
                            Operator::IsAnyOf => {
                                IriRef::new(format!("{}isAnyOf", name_spaces::ODRL_NS))?
                            }
                            Operator::IsNoneOf => {
                                IriRef::new(format!("{}isNoneOf", name_spaces::ODRL_NS))?
                            }
                            Operator::IsPartOf => {
                                IriRef::new(format!("{}isPartOf", name_spaces::ODRL_NS))?
                            }
                        };
                        graph.insert(
                            &refinement_term,
                            &IriRef::new(format!("{}operator", name_spaces::LD_NS))?,
                            &operator_term,
                        )?;

                        // RightOperand
                        match &constraint.right_operand {
                            RightOperand::IRI(iri) => {
                                let right_operand_term = IriRef::new(iri.clone())?;
                                graph.insert(
                                    &refinement_term,
                                    &IriRef::new(format!("{}rightOperand", name_spaces::LD_NS))?,
                                    &right_operand_term,
                                )?;
                            }
                            RightOperand::Literal(literal) => {
                                // For literal left operands, you may need to handle data type and other attributes
                                let datatype_term = match &constraint.data_type {
                                    Some(datatype) => IriRef::new(datatype.clone())?,
                                    None => IriRef::new("xsd:string".to_string())?, // Assuming default data type is string
                                };
                                // Insert triple for data type
                                graph.insert(
                                    &IriRef::new(format!("{}rightOperand", name_spaces::LD_NS))?,
                                    &IriRef::new(format!("{}type", name_spaces::RDF_NS))?,
                                    &datatype_term,
                                )?;

                                // Insert triple for literal value
                                graph.insert(
                                    &IriRef::new(format!("{}rightOperand", name_spaces::LD_NS))?,
                                    &IriRef::new(format!("{}value", name_spaces::RDF_NS))?,
                                    &IriRef::new(literal.clone())?,
                                )?;
                            }
                            RightOperand::Reference(iri) => {
                                let right_operand_term = IriRef::new(iri.clone())?;
                                graph.insert(
                                    &refinement_term,
                                    &IriRef::new(format!("{}rightOperand", name_spaces::LD_NS))?,
                                    &right_operand_term,
                                )?;
                            }
                        }

                        // Unit
                        if let Some(unit) = &constraint.unit {
                            let unit_term = IriRef::new(unit.clone())?;
                            graph.insert(
                                &refinement_term,
                                &IriRef::new(format!("{}unit", name_spaces::LD_NS))?,
                                &unit_term,
                            )?;
                        }
                    }
                }
                Refinements::LogicalConstraints(_logical_constraints) => {
                    for logical_constraint in _logical_constraints {
                        // uid
                        if let Some(uid) = &logical_constraint.uid {
                            let logical_constraint_term = IriRef::new(uid.clone())?;
                            graph.insert(
                                &refinement_term,
                                &IriRef::new(format!("{}LogicalConstraint", name_spaces::LD_NS))?,
                                &logical_constraint_term,
                            )?;
                        }
                        // operator and constraints
                        if let Some(operand) = &logical_constraint.operand {
                            let (operator, constraints) = operand;
                            let operator_term = match operator {
                                LogicalOperator::And => {
                                    IriRef::new(format!("{}and", name_spaces::ODRL_NS))?
                                }
                                LogicalOperator::Or => {
                                    IriRef::new(format!("{}or", name_spaces::ODRL_NS))?
                                }
                                LogicalOperator::Xone => {
                                    IriRef::new(format!("{}xone", name_spaces::ODRL_NS))?
                                }
                                LogicalOperator::AndSequence => {
                                    IriRef::new(format!("{}andSequence", name_spaces::ODRL_NS))?
                                }
                            };
                            for constraint in constraints {
                                let constraint_term = IriRef::new(constraint.clone())?;
                                graph.insert(&refinement_term, &operator_term, constraint_term)?;
                            }
                        }
                    }
                }
            }
        }

        if let Some(assigner) = &self.assigner {
            let assigner_term = IriRef::new(assigner.uid.clone().unwrap())?;
            graph.insert(
                &permission_term,
                &IriRef::new(format!("{}assigner", name_spaces::LD_NS))?,
                &assigner_term,
            )?; // Assigner
        }

        if let Some(assignee) = &self.assignee {
            let assignee_term = IriRef::new(assignee.uid.clone().unwrap())?;
            graph.insert(
                &permission_term,
                &IriRef::new(format!("{}assignee", name_spaces::LD_NS))?,
                &assignee_term,
            )?; // Assignee
        }

        // Insert triples for constraints
        if self.constraints.len() != 0 {
            let constraint_term = IriRef::new(format!("{}Constraint", name_spaces::LD_NS))?;
            graph.insert(
                &permission_term,
                &IriRef::new(format!("{}hasConstraint", name_spaces::LD_NS))?,
                &constraint_term,
            )?;

            for constraint in &self.constraints {
                // LeftOperand
                match &constraint.left_operand {
                    LeftOperand::IRI(iri) => {
                        let left_operand_term = IriRef::new(iri.clone())?;
                        graph.insert(
                            &constraint_term,
                            &IriRef::new(format!("{}leftOperand", name_spaces::LD_NS))?,
                            &left_operand_term,
                        )?;
                    }
                    LeftOperand::Literal(literal) => {
                        // For literal left operands, you may need to handle data type and other attributes
                        let datatype_term = match &constraint.data_type {
                            Some(datatype) => IriRef::new(datatype.clone())?,
                            None => IriRef::new("xsd:string".to_string())?, // Assuming default data type is string
                        };
                        // Insert triple for data type
                        graph.insert(
                            &IriRef::new(format!("{}leftOperand", name_spaces::LD_NS))?,
                            &IriRef::new(format!("{}type", name_spaces::RDF_NS))?,
                            &datatype_term,
                        )?;

                        // Insert triple for literal value
                        graph.insert(
                            &IriRef::new(format!("{}leftOperand", name_spaces::LD_NS))?,
                            &IriRef::new(format!("{}value", name_spaces::RDF_NS))?,
                            &IriRef::new(literal.clone())?,
                        )?;
                    }
                    LeftOperand::Reference(iri) => {
                        let left_operand_term = IriRef::new(iri.clone())?;
                        graph.insert(
                            &constraint_term,
                            &IriRef::new(format!("{}leftOperand", name_spaces::LD_NS))?,
                            &left_operand_term,
                        )?;
                    }
                }

                // Operator
                let operator_term = match constraint.operator {
                    Operator::Equal => IriRef::new(format!("{}eq", name_spaces::ODRL_NS))?,
                    Operator::NotEqual => IriRef::new(format!("{}neq", name_spaces::ODRL_NS))?,
                    Operator::GreaterThan => IriRef::new(format!("{}gt", name_spaces::ODRL_NS))?,
                    Operator::LessThan => IriRef::new(format!("{}lt", name_spaces::ODRL_NS))?,
                    Operator::GreaterThanOrEqual => {
                        IriRef::new(format!("{}gteq", name_spaces::ODRL_NS))?
                    }
                    Operator::LessThanOrEqual => {
                        IriRef::new(format!("{}lteq", name_spaces::ODRL_NS))?
                    }
                    Operator::HasPart => IriRef::new(format!("{}hasPart", name_spaces::ODRL_NS))?,
                    Operator::IsA => IriRef::new(format!("{}isA", name_spaces::ODRL_NS))?,
                    Operator::IsAllOf => IriRef::new(format!("{}isAllOf", name_spaces::ODRL_NS))?,
                    Operator::IsAnyOf => IriRef::new(format!("{}isAnyOf", name_spaces::ODRL_NS))?,
                    Operator::IsNoneOf => IriRef::new(format!("{}isNoneOf", name_spaces::ODRL_NS))?,
                    Operator::IsPartOf => IriRef::new(format!("{}isPartOf", name_spaces::ODRL_NS))?,
                };
                graph.insert(
                    &constraint_term,
                    &IriRef::new(format!("{}operator", name_spaces::LD_NS))?,
                    &operator_term,
                )?;

                // RightOperand
                match &constraint.right_operand {
                    RightOperand::IRI(iri) => {
                        let right_operand_term = IriRef::new(iri.clone())?;
                        graph.insert(
                            &constraint_term,
                            &IriRef::new(format!("{}rightOperand", name_spaces::LD_NS))?,
                            &right_operand_term,
                        )?;
                    }
                    RightOperand::Literal(literal) => {
                        // For literal left operands, you may need to handle data type and other attributes
                        let datatype_term = match &constraint.data_type {
                            Some(datatype) => IriRef::new(datatype.clone())?,
                            None => IriRef::new("xsd:string".to_string())?, // Assuming default data type is string
                        };
                        // Insert triple for data type
                        graph.insert(
                            &IriRef::new(format!("{}rightOperand", name_spaces::LD_NS))?,
                            &IriRef::new(format!("{}type", name_spaces::RDF_NS))?,
                            &datatype_term,
                        )?;

                        // Insert triple for literal value
                        graph.insert(
                            &IriRef::new(format!("{}rightOperand", name_spaces::LD_NS))?,
                            &IriRef::new(format!("{}value", name_spaces::RDF_NS))?,
                            &IriRef::new(literal.clone())?,
                        )?;
                    }
                    RightOperand::Reference(iri) => {
                        let right_operand_term = IriRef::new(iri.clone())?;
                        graph.insert(
                            &constraint_term,
                            &IriRef::new(format!("{}rightOperand", name_spaces::LD_NS))?,
                            &right_operand_term,
                        )?;
                    }
                }

                // Unit
                if let Some(unit) = &constraint.unit {
                    let unit_term = IriRef::new(unit.clone())?;
                    graph.insert(
                        &constraint_term,
                        &IriRef::new(format!("{}unit", name_spaces::LD_NS))?,
                        &unit_term,
                    )?;
                }
            }
        }

        Ok(())
    }
}

impl Serializable for Prohibition {
    fn add_to_graph(&self, graph: &mut LightGraph) -> Result<(), Box<dyn std::error::Error>> {
        let prohibiton_term;
        if let Some(uid) = &self.uid {
            prohibiton_term = IriRef::new(uid.clone())?;
        } else {
            prohibiton_term = IriRef::new(format!("{}Prohibition", name_spaces::LD_NS))?;
        }

        let target_uid = self.target.uid.clone().unwrap();
        let target_term = IriRef::new(target_uid)?;
        let action_name = self.action.name.clone();

        // Insert triples for permission
        graph.insert(
            &prohibiton_term,
            IriRef::new(format!("{}type", name_spaces::RDF_NS))?,
            &IriRef::new(format!("{}prohibition", name_spaces::LD_NS))?,
        )?; // Type
        graph.insert(
            &prohibiton_term,
            &IriRef::new(format!("{}target", name_spaces::LD_NS))?,
            &target_term,
        )?; // Target
        graph.insert(
            &prohibiton_term,
            &IriRef::new(format!("{}action", name_spaces::LD_NS))?,
            &IriRef::new(format!("{}{}", name_spaces::ODRL_NS, action_name))?,
        )?; // Action

        // Action Refinement
        if let Some(refinements) = &self.action.refinements {
            let refinement_term = IriRef::new(format!("{}refinement", name_spaces::LD_NS))?;
            graph.insert(
                &IriRef::new(format!("{}action", name_spaces::LD_NS))?,
                &IriRef::new(format!("{}hasRefinement", name_spaces::LD_NS))?,
                &refinement_term,
            )?;

            match refinements {
                Refinements::Constraints(constraints) => {
                    for constraint in constraints {
                        // LeftOperand
                        match &constraint.left_operand {
                            LeftOperand::IRI(iri) => {
                                let left_operand_term = IriRef::new(iri.clone())?;
                                graph.insert(
                                    &refinement_term,
                                    &IriRef::new(format!("{}leftOperand", name_spaces::LD_NS))?,
                                    &left_operand_term,
                                )?;
                            }
                            LeftOperand::Literal(literal) => {
                                // For literal left operands, you may need to handle data type and other attributes
                                let datatype_term = match &constraint.data_type {
                                    Some(datatype) => IriRef::new(datatype.clone())?,
                                    None => IriRef::new("xsd:string".to_string())?, // Assuming default data type is string
                                };
                                // Insert triple for data type
                                graph.insert(
                                    &IriRef::new(format!("{}leftOperand", name_spaces::LD_NS))?,
                                    &IriRef::new(format!("{}type", name_spaces::RDF_NS))?,
                                    &datatype_term,
                                )?;

                                // Insert triple for literal value
                                graph.insert(
                                    &IriRef::new(format!("{}leftOperand", name_spaces::LD_NS))?,
                                    &IriRef::new(format!("{}value", name_spaces::RDF_NS))?,
                                    &IriRef::new(literal.clone())?,
                                )?;
                            }
                            LeftOperand::Reference(iri) => {
                                let left_operand_term = IriRef::new(iri.clone())?;
                                graph.insert(
                                    &refinement_term,
                                    &IriRef::new(format!("{}leftOperand", name_spaces::LD_NS))?,
                                    &left_operand_term,
                                )?;
                            }
                        }

                        // Operator
                        let operator_term = match constraint.operator {
                            Operator::Equal => IriRef::new(format!("{}eq", name_spaces::ODRL_NS))?,
                            Operator::NotEqual => {
                                IriRef::new(format!("{}neq", name_spaces::ODRL_NS))?
                            }
                            Operator::GreaterThan => {
                                IriRef::new(format!("{}gt", name_spaces::ODRL_NS))?
                            }
                            Operator::LessThan => {
                                IriRef::new(format!("{}lt", name_spaces::ODRL_NS))?
                            }
                            Operator::GreaterThanOrEqual => {
                                IriRef::new(format!("{}gteq", name_spaces::ODRL_NS))?
                            }
                            Operator::LessThanOrEqual => {
                                IriRef::new(format!("{}lteq", name_spaces::ODRL_NS))?
                            }
                            Operator::HasPart => {
                                IriRef::new(format!("{}hasPart", name_spaces::ODRL_NS))?
                            }
                            Operator::IsA => IriRef::new(format!("{}isA", name_spaces::ODRL_NS))?,
                            Operator::IsAllOf => {
                                IriRef::new(format!("{}isAllOf", name_spaces::ODRL_NS))?
                            }
                            Operator::IsAnyOf => {
                                IriRef::new(format!("{}isAnyOf", name_spaces::ODRL_NS))?
                            }
                            Operator::IsNoneOf => {
                                IriRef::new(format!("{}isNoneOf", name_spaces::ODRL_NS))?
                            }
                            Operator::IsPartOf => {
                                IriRef::new(format!("{}isPartOf", name_spaces::ODRL_NS))?
                            }
                        };
                        graph.insert(
                            &refinement_term,
                            &IriRef::new(format!("{}operator", name_spaces::LD_NS))?,
                            &operator_term,
                        )?;

                        // RightOperand
                        match &constraint.right_operand {
                            RightOperand::IRI(iri) => {
                                let right_operand_term = IriRef::new(iri.clone())?;
                                graph.insert(
                                    &refinement_term,
                                    &IriRef::new(format!("{}rightOperand", name_spaces::LD_NS))?,
                                    &right_operand_term,
                                )?;
                            }
                            RightOperand::Literal(literal) => {
                                // For literal left operands, you may need to handle data type and other attributes
                                let datatype_term = match &constraint.data_type {
                                    Some(datatype) => IriRef::new(datatype.clone())?,
                                    None => IriRef::new("xsd:string".to_string())?, // Assuming default data type is string
                                };
                                // Insert triple for data type
                                graph.insert(
                                    &IriRef::new(format!("{}rightOperand", name_spaces::LD_NS))?,
                                    &IriRef::new(format!("{}type", name_spaces::RDF_NS))?,
                                    &datatype_term,
                                )?;

                                // Insert triple for literal value
                                graph.insert(
                                    &IriRef::new(format!("{}rightOperand", name_spaces::LD_NS))?,
                                    &IriRef::new(format!("{}value", name_spaces::RDF_NS))?,
                                    &IriRef::new(literal.clone())?,
                                )?;
                            }
                            RightOperand::Reference(iri) => {
                                let right_operand_term = IriRef::new(iri.clone())?;
                                graph.insert(
                                    &refinement_term,
                                    &IriRef::new(format!("{}rightOperand", name_spaces::LD_NS))?,
                                    &right_operand_term,
                                )?;
                            }
                        }

                        // Unit
                        if let Some(unit) = &constraint.unit {
                            let unit_term = IriRef::new(unit.clone())?;
                            graph.insert(
                                &refinement_term,
                                &IriRef::new(format!("{}unit", name_spaces::LD_NS))?,
                                &unit_term,
                            )?;
                        }
                    }
                }
                Refinements::LogicalConstraints(_logical_constraints) => {
                    for logical_constraint in _logical_constraints {
                        // uid
                        if let Some(uid) = &logical_constraint.uid {
                            let logical_constraint_term = IriRef::new(uid.clone())?;
                            graph.insert(
                                &refinement_term,
                                &IriRef::new(format!("{}LogicalConstraint", name_spaces::LD_NS))?,
                                &logical_constraint_term,
                            )?;
                        }
                        // operator and constraints
                        if let Some(operand) = &logical_constraint.operand {
                            let (operator, constraints) = operand;
                            let operator_term = match operator {
                                LogicalOperator::And => {
                                    IriRef::new(format!("{}and", name_spaces::ODRL_NS))?
                                }
                                LogicalOperator::Or => {
                                    IriRef::new(format!("{}or", name_spaces::ODRL_NS))?
                                }
                                LogicalOperator::Xone => {
                                    IriRef::new(format!("{}xone", name_spaces::ODRL_NS))?
                                }
                                LogicalOperator::AndSequence => {
                                    IriRef::new(format!("{}andSequence", name_spaces::ODRL_NS))?
                                }
                            };
                            for constraint in constraints {
                                let constraint_term = IriRef::new(constraint.clone())?;
                                graph.insert(&refinement_term, &operator_term, constraint_term)?;
                            }
                        }
                    }
                }
            }
        }

        if let Some(assigner) = &self.assigner {
            let assigner_term = IriRef::new(assigner.uid.clone().unwrap())?;
            graph.insert(
                &prohibiton_term,
                &IriRef::new(format!("{}assigner", name_spaces::LD_NS))?,
                &assigner_term,
            )?; // Assigner
        }

        if let Some(assignee) = &self.assignee {
            let assignee_term = IriRef::new(assignee.uid.clone().unwrap())?;
            graph.insert(
                &prohibiton_term,
                &IriRef::new(format!("{}assignee", name_spaces::LD_NS))?,
                &assignee_term,
            )?; // Assignee
        }

        // Insert triples for constraints
        if self.constraints.len() != 0 {
            let constraint_term = IriRef::new(format!("{}Constraint", name_spaces::LD_NS))?;
            graph.insert(
                &prohibiton_term,
                &IriRef::new(format!("{}hasConstraint", name_spaces::LD_NS))?,
                &constraint_term,
            )?;
            for constraint in &self.constraints {
                // LeftOperand
                match &constraint.left_operand {
                    LeftOperand::IRI(iri) => {
                        let left_operand_term = IriRef::new(iri.clone())?;
                        graph.insert(
                            &constraint_term,
                            &IriRef::new(format!("{}leftOperand", name_spaces::LD_NS))?,
                            &left_operand_term,
                        )?;
                    }
                    LeftOperand::Literal(literal) => {
                        // For literal left operands, you may need to handle data type and other attributes
                        let datatype_term = match &constraint.data_type {
                            Some(datatype) => IriRef::new(datatype.clone())?,
                            None => IriRef::new("xsd:string".to_string())?, // Assuming default data type is string
                        };
                        // Insert triple for data type
                        graph.insert(
                            &IriRef::new(format!("{}leftOperand", name_spaces::LD_NS))?,
                            &IriRef::new(format!("{}type", name_spaces::RDF_NS))?,
                            &datatype_term,
                        )?;

                        // Insert triple for literal value
                        graph.insert(
                            &IriRef::new(format!("{}leftOperand", name_spaces::LD_NS))?,
                            &IriRef::new(format!("{}value", name_spaces::RDF_NS))?,
                            &IriRef::new(literal.clone())?,
                        )?;
                    }
                    LeftOperand::Reference(iri) => {
                        let left_operand_term = IriRef::new(iri.clone())?;
                        graph.insert(
                            &constraint_term,
                            &IriRef::new(format!("{}leftOperand", name_spaces::LD_NS))?,
                            &left_operand_term,
                        )?;
                    }
                }

                // Operator
                let operator_term = match constraint.operator {
                    Operator::Equal => IriRef::new(format!("{}eq", name_spaces::ODRL_NS))?,
                    Operator::NotEqual => IriRef::new(format!("{}neq", name_spaces::ODRL_NS))?,
                    Operator::GreaterThan => IriRef::new(format!("{}gt", name_spaces::ODRL_NS))?,
                    Operator::LessThan => IriRef::new(format!("{}lt", name_spaces::ODRL_NS))?,
                    Operator::GreaterThanOrEqual => {
                        IriRef::new(format!("{}gteq", name_spaces::ODRL_NS))?
                    }
                    Operator::LessThanOrEqual => {
                        IriRef::new(format!("{}lteq", name_spaces::ODRL_NS))?
                    }
                    Operator::HasPart => IriRef::new(format!("{}hasPart", name_spaces::ODRL_NS))?,
                    Operator::IsA => IriRef::new(format!("{}isA", name_spaces::ODRL_NS))?,
                    Operator::IsAllOf => IriRef::new(format!("{}isAllOf", name_spaces::ODRL_NS))?,
                    Operator::IsAnyOf => IriRef::new(format!("{}isAnyOf", name_spaces::ODRL_NS))?,
                    Operator::IsNoneOf => IriRef::new(format!("{}isNoneOf", name_spaces::ODRL_NS))?,
                    Operator::IsPartOf => IriRef::new(format!("{}isPartOf", name_spaces::ODRL_NS))?,
                };
                graph.insert(
                    &constraint_term,
                    &IriRef::new(format!("{}operator", name_spaces::LD_NS))?,
                    &operator_term,
                )?;

                // RightOperand
                match &constraint.right_operand {
                    RightOperand::IRI(iri) => {
                        let right_operand_term = IriRef::new(iri.clone())?;
                        graph.insert(
                            &constraint_term,
                            &IriRef::new(format!("{}rightOperand", name_spaces::LD_NS))?,
                            &right_operand_term,
                        )?;
                    }
                    RightOperand::Literal(literal) => {
                        // For literal left operands, you may need to handle data type and other attributes
                        let datatype_term = match &constraint.data_type {
                            Some(datatype) => IriRef::new(datatype.clone())?,
                            None => IriRef::new("xsd:string".to_string())?, // Assuming default data type is string
                        };
                        // Insert triple for data type
                        graph.insert(
                            &IriRef::new(format!("{}rightOperand", name_spaces::LD_NS))?,
                            &IriRef::new(format!("{}type", name_spaces::RDF_NS))?,
                            &datatype_term,
                        )?;

                        // Insert triple for literal value
                        graph.insert(
                            &IriRef::new(format!("{}rightOperand", name_spaces::LD_NS))?,
                            &IriRef::new(format!("{}value", name_spaces::RDF_NS))?,
                            &IriRef::new(literal.clone())?,
                        )?;
                    }
                    RightOperand::Reference(iri) => {
                        let right_operand_term = IriRef::new(iri.clone())?;
                        graph.insert(
                            &constraint_term,
                            &IriRef::new(format!("{}rightOperand", name_spaces::LD_NS))?,
                            &right_operand_term,
                        )?;
                    }
                }
                // Unit
                if let Some(unit) = &constraint.unit {
                    let unit_term = IriRef::new(unit.clone())?;
                    graph.insert(
                        &constraint_term,
                        &IriRef::new(format!("{}unit", name_spaces::LD_NS))?,
                        &unit_term,
                    )?;
                }
            }
        }
        Ok(())
    }
}

impl Serializable for Duty {
    fn add_to_graph(&self, graph: &mut LightGraph) -> Result<(), Box<dyn std::error::Error>> {
        let duty_term;
        if let Some(uid) = &self.uid {
            duty_term = IriRef::new(uid.clone())?;
        } else {
            duty_term = IriRef::new(format!("{}Duty", name_spaces::LD_NS))?;
        }

        let action_name = self.action.name.clone();

        // Insert triples for permission
        graph.insert(
            &duty_term,
            IriRef::new(format!("{}type", name_spaces::RDF_NS))?,
            &IriRef::new(format!("{}duty", name_spaces::LD_NS))?,
        )?; // Type
        if let Some(target) = &self.target {
            let target_uid = target.uid.clone().unwrap();
            let target_term = IriRef::new(target_uid)?;
            graph.insert(
                &duty_term,
                &IriRef::new(format!("{}target", name_spaces::LD_NS))?,
                &target_term,
            )?; // Target
        }
        graph.insert(
            &duty_term,
            &IriRef::new(format!("{}action", name_spaces::LD_NS))?,
            &IriRef::new(format!("{}{}", name_spaces::ODRL_NS, action_name))?,
        )?; // Action

        // Action Refinement
        if let Some(refinements) = &self.action.refinements {
            let refinement_term = IriRef::new(format!("{}refinement", name_spaces::LD_NS))?;
            graph.insert(
                &IriRef::new(format!("{}action", name_spaces::LD_NS))?,
                &IriRef::new(format!("{}hasRefinement", name_spaces::LD_NS))?,
                &refinement_term,
            )?;

            match refinements {
                Refinements::Constraints(constraints) => {
                    for constraint in constraints {
                        // LeftOperand
                        match &constraint.left_operand {
                            LeftOperand::IRI(iri) => {
                                let left_operand_term = IriRef::new(iri.clone())?;
                                graph.insert(
                                    &refinement_term,
                                    &IriRef::new(format!("{}leftOperand", name_spaces::LD_NS))?,
                                    &left_operand_term,
                                )?;
                            }
                            LeftOperand::Literal(literal) => {
                                // For literal left operands, you may need to handle data type and other attributes
                                let datatype_term = match &constraint.data_type {
                                    Some(datatype) => IriRef::new(datatype.clone())?,
                                    None => IriRef::new("xsd:string".to_string())?, // Assuming default data type is string
                                };
                                // Insert triple for data type
                                graph.insert(
                                    &IriRef::new(format!("{}leftOperand", name_spaces::LD_NS))?,
                                    &IriRef::new(format!("{}type", name_spaces::RDF_NS))?,
                                    &datatype_term,
                                )?;

                                // Insert triple for literal value
                                graph.insert(
                                    &IriRef::new(format!("{}leftOperand", name_spaces::LD_NS))?,
                                    &IriRef::new(format!("{}value", name_spaces::RDF_NS))?,
                                    &IriRef::new(literal.clone())?,
                                )?;
                            }
                            LeftOperand::Reference(iri) => {
                                let left_operand_term = IriRef::new(iri.clone())?;
                                graph.insert(
                                    &refinement_term,
                                    &IriRef::new(format!("{}leftOperand", name_spaces::LD_NS))?,
                                    &left_operand_term,
                                )?;
                            }
                        }

                        // Operator
                        let operator_term = match constraint.operator {
                            Operator::Equal => IriRef::new(format!("{}eq", name_spaces::ODRL_NS))?,
                            Operator::NotEqual => {
                                IriRef::new(format!("{}neq", name_spaces::ODRL_NS))?
                            }
                            Operator::GreaterThan => {
                                IriRef::new(format!("{}gt", name_spaces::ODRL_NS))?
                            }
                            Operator::LessThan => {
                                IriRef::new(format!("{}lt", name_spaces::ODRL_NS))?
                            }
                            Operator::GreaterThanOrEqual => {
                                IriRef::new(format!("{}gteq", name_spaces::ODRL_NS))?
                            }
                            Operator::LessThanOrEqual => {
                                IriRef::new(format!("{}lteq", name_spaces::ODRL_NS))?
                            }
                            Operator::HasPart => {
                                IriRef::new(format!("{}hasPart", name_spaces::ODRL_NS))?
                            }
                            Operator::IsA => IriRef::new(format!("{}isA", name_spaces::ODRL_NS))?,
                            Operator::IsAllOf => {
                                IriRef::new(format!("{}isAllOf", name_spaces::ODRL_NS))?
                            }
                            Operator::IsAnyOf => {
                                IriRef::new(format!("{}isAnyOf", name_spaces::ODRL_NS))?
                            }
                            Operator::IsNoneOf => {
                                IriRef::new(format!("{}isNoneOf", name_spaces::ODRL_NS))?
                            }
                            Operator::IsPartOf => {
                                IriRef::new(format!("{}isPartOf", name_spaces::ODRL_NS))?
                            }
                        };
                        graph.insert(
                            &refinement_term,
                            &IriRef::new(format!("{}operator", name_spaces::LD_NS))?,
                            &operator_term,
                        )?;

                        // RightOperand
                        match &constraint.right_operand {
                            RightOperand::IRI(iri) => {
                                let right_operand_term = IriRef::new(iri.clone())?;
                                graph.insert(
                                    &refinement_term,
                                    &IriRef::new(format!("{}rightOperand", name_spaces::LD_NS))?,
                                    &right_operand_term,
                                )?;
                            }
                            RightOperand::Literal(literal) => {
                                // For literal left operands, you may need to handle data type and other attributes
                                let datatype_term = match &constraint.data_type {
                                    Some(datatype) => IriRef::new(datatype.clone())?,
                                    None => IriRef::new("xsd:string".to_string())?, // Assuming default data type is string
                                };
                                // Insert triple for data type
                                graph.insert(
                                    &IriRef::new(format!("{}rightOperand", name_spaces::LD_NS))?,
                                    &IriRef::new(format!("{}type", name_spaces::RDF_NS))?,
                                    &datatype_term,
                                )?;

                                // Insert triple for literal value
                                graph.insert(
                                    &IriRef::new(format!("{}rightOperand", name_spaces::LD_NS))?,
                                    &IriRef::new(format!("{}value", name_spaces::RDF_NS))?,
                                    &IriRef::new(literal.clone())?,
                                )?;
                            }
                            RightOperand::Reference(iri) => {
                                let right_operand_term = IriRef::new(iri.clone())?;
                                graph.insert(
                                    &refinement_term,
                                    &IriRef::new(format!("{}rightOperand", name_spaces::LD_NS))?,
                                    &right_operand_term,
                                )?;
                            }
                        }

                        // Unit
                        if let Some(unit) = &constraint.unit {
                            let unit_term = IriRef::new(unit.clone())?;
                            graph.insert(
                                &refinement_term,
                                &IriRef::new(format!("{}unit", name_spaces::LD_NS))?,
                                &unit_term,
                            )?;
                        }
                    }
                }
                Refinements::LogicalConstraints(_logical_constraints) => {
                    for logical_constraint in _logical_constraints {
                        // uid
                        if let Some(uid) = &logical_constraint.uid {
                            let logical_constraint_term = IriRef::new(uid.clone())?;
                            graph.insert(
                                &refinement_term,
                                &IriRef::new(format!("{}LogicalConstraint", name_spaces::LD_NS))?,
                                &logical_constraint_term,
                            )?;
                        }
                        // operator and constraints
                        if let Some(operand) = &logical_constraint.operand {
                            let (operator, constraints) = operand;
                            let operator_term = match operator {
                                LogicalOperator::And => {
                                    IriRef::new(format!("{}and", name_spaces::ODRL_NS))?
                                }
                                LogicalOperator::Or => {
                                    IriRef::new(format!("{}or", name_spaces::ODRL_NS))?
                                }
                                LogicalOperator::Xone => {
                                    IriRef::new(format!("{}xone", name_spaces::ODRL_NS))?
                                }
                                LogicalOperator::AndSequence => {
                                    IriRef::new(format!("{}andSequence", name_spaces::ODRL_NS))?
                                }
                            };
                            for constraint in constraints {
                                let constraint_term = IriRef::new(constraint.clone())?;
                                graph.insert(&refinement_term, &operator_term, constraint_term)?;
                            }
                        }
                    }
                }
            }
        }

        if let Some(assigner) = &self.assigner {
            let assigner_term = IriRef::new(assigner.uid.clone().unwrap())?;
            graph.insert(
                &duty_term,
                &IriRef::new(format!("{}assigner", name_spaces::LD_NS))?,
                &assigner_term,
            )?; // Assigner
        }

        if let Some(assignee) = &self.assignee {
            let assignee_term = IriRef::new(assignee.uid.clone().unwrap())?;
            graph.insert(
                &duty_term,
                &IriRef::new(format!("{}assignee", name_spaces::LD_NS))?,
                &assignee_term,
            )?; // Assignee
        }

        // Insert triples for constraints+
        if self.constraints.len() != 0 {
            let constraint_term = IriRef::new(format!("{}Constraint", name_spaces::LD_NS))?;
            graph.insert(
                &duty_term,
                &IriRef::new(format!("{}hasConstraint", name_spaces::LD_NS))?,
                &constraint_term,
            )?;
            for constraint in &self.constraints {
                // LeftOperand
                match &constraint.left_operand {
                    LeftOperand::IRI(iri) => {
                        let left_operand_term = IriRef::new(iri.clone())?;
                        graph.insert(
                            &constraint_term,
                            &IriRef::new(format!("{}leftOperand", name_spaces::LD_NS))?,
                            &left_operand_term,
                        )?;
                    }
                    LeftOperand::Literal(literal) => {
                        // For literal left operands, you may need to handle data type and other attributes
                        let datatype_term = match &constraint.data_type {
                            Some(datatype) => IriRef::new(datatype.clone())?,
                            None => IriRef::new("xsd:string".to_string())?, // Assuming default data type is string
                        };
                        // Insert triple for data type
                        graph.insert(
                            IriRef::new(format!("{}leftOperand", name_spaces::LD_NS))?,
                            &IriRef::new(format!("{}type", name_spaces::RDF_NS))?,
                            &datatype_term,
                        )?;

                        // Insert triple for literal value
                        graph.insert(
                            &IriRef::new(format!("{}leftOperand", name_spaces::LD_NS))?,
                            &IriRef::new(format!("{}value", name_spaces::RDF_NS))?,
                            &IriRef::new(literal.clone())?,
                        )?;
                    }
                    LeftOperand::Reference(iri) => {
                        let left_operand_term = IriRef::new(iri.clone())?;
                        graph.insert(
                            &constraint_term,
                            &IriRef::new(format!("{}leftOperand", name_spaces::LD_NS))?,
                            &left_operand_term,
                        )?;
                    }
                }

                // Operator
                let operator_term = match constraint.operator {
                    Operator::Equal => IriRef::new(format!("{}eq", name_spaces::ODRL_NS))?,
                    Operator::NotEqual => IriRef::new(format!("{}neq", name_spaces::ODRL_NS))?,
                    Operator::GreaterThan => IriRef::new(format!("{}gt", name_spaces::ODRL_NS))?,
                    Operator::LessThan => IriRef::new(format!("{}lt", name_spaces::ODRL_NS))?,
                    Operator::GreaterThanOrEqual => {
                        IriRef::new(format!("{}gteq", name_spaces::ODRL_NS))?
                    }
                    Operator::LessThanOrEqual => {
                        IriRef::new(format!("{}lteq", name_spaces::ODRL_NS))?
                    }
                    Operator::HasPart => IriRef::new(format!("{}hasPart", name_spaces::ODRL_NS))?,
                    Operator::IsA => IriRef::new(format!("{}isA", name_spaces::ODRL_NS))?,
                    Operator::IsAllOf => IriRef::new(format!("{}isAllOf", name_spaces::ODRL_NS))?,
                    Operator::IsAnyOf => IriRef::new(format!("{}isAnyOf", name_spaces::ODRL_NS))?,
                    Operator::IsNoneOf => IriRef::new(format!("{}isNoneOf", name_spaces::ODRL_NS))?,
                    Operator::IsPartOf => IriRef::new(format!("{}isPartOf", name_spaces::ODRL_NS))?,
                };
                graph.insert(
                    &constraint_term,
                    &IriRef::new(format!("{}operator", name_spaces::LD_NS))?,
                    &operator_term,
                )?;

                // RightOperand
                match &constraint.right_operand {
                    RightOperand::IRI(iri) => {
                        let right_operand_term = IriRef::new(iri.clone())?;
                        graph.insert(
                            &constraint_term,
                            &IriRef::new(format!("{}rightOperand", name_spaces::LD_NS))?,
                            &right_operand_term,
                        )?;
                    }
                    RightOperand::Literal(literal) => {
                        // For literal left operands, you may need to handle data type and other attributes
                        let datatype_term = match &constraint.data_type {
                            Some(datatype) => IriRef::new(datatype.clone())?,
                            None => IriRef::new("xsd:string".to_string())?, // Assuming default data type is string
                        };
                        // Insert triple for data type
                        graph.insert(
                            &IriRef::new(format!("{}rightOperand", name_spaces::LD_NS))?,
                            &IriRef::new(format!("{}type", name_spaces::RDF_NS))?,
                            &datatype_term,
                        )?;

                        // Insert triple for literal value
                        graph.insert(
                            &IriRef::new(format!("{}rightOperand", name_spaces::LD_NS))?,
                            &IriRef::new(format!("{}value", name_spaces::RDF_NS))?,
                            &IriRef::new(literal.clone())?,
                        )?;
                    }
                    RightOperand::Reference(iri) => {
                        let right_operand_term = IriRef::new(iri.clone())?;
                        graph.insert(
                            &constraint_term,
                            &IriRef::new(format!("{}rightOperand", name_spaces::LD_NS))?,
                            &right_operand_term,
                        )?;
                    }
                }

                // Unit
                if let Some(unit) = &constraint.unit {
                    let unit_term = IriRef::new(unit.clone())?;
                    graph.insert(
                        &constraint_term,
                        &IriRef::new(format!("{}unit", name_spaces::LD_NS))?,
                        &unit_term,
                    )?;
                }
            }
        }
        Ok(())
    }
}

impl Serializable for Obligation {
    fn add_to_graph(&self, graph: &mut LightGraph) -> Result<(), Box<dyn std::error::Error>> {
        let obligation_type = IriRef::new(format!("{}type", name_spaces::RDF_NS))?; // Type
        let obligation_term = IriRef::new(format!("{}obligation", name_spaces::LD_NS))?;

        // Insert triples for Obligation
        graph.insert(&obligation_term, &obligation_type, &obligation_term)?; // Type
        graph.insert(
            &obligation_term,
            &IriRef::new(format!("{}assigner", name_spaces::LD_NS))?,
            &IriRef::new(self.assigner.uid.clone().unwrap())?,
        )?; // Assigner
        graph.insert(
            &obligation_term,
            &IriRef::new(format!("{}assignee", name_spaces::LD_NS))?,
            &IriRef::new(self.assignee.uid.clone().unwrap())?,
        )?; // Assignee
        graph.insert(
            &obligation_term,
            &IriRef::new(format!("{}action", name_spaces::LD_NS))?,
            &IriRef::new(format!("{}{}", name_spaces::ODRL_NS, self.action.name))?,
        )?; // Action

        if self.consequence.len() != 0 {
            let consequence_term = IriRef::new(format!("{}consequence", name_spaces::LD_NS))?;
            graph.insert(
                &obligation_term,
                &IriRef::new(format!("{}hasConsequence", name_spaces::LD_NS))?,
                &consequence_term,
            )?;
            for duty in &self.consequence {
                duty.add_to_graph(graph)?;
            }
        }

        Ok(())
    }
}

fn build_graph<T: Serializable>(
    odrl_object: &T,
    contexts: Vec<&str>,
) -> Result<LightGraph, Box<dyn std::error::Error>> {
    let mut graph = LightGraph::new();
    for context in contexts {
        graph.insert(
            &IriRef::new(name_spaces::LD_NS)?,
            &IriRef::new(format!("{}context", name_spaces::LD_NS))?,
            &IriRef::new(context)?,
        )?; // Context
    }

    odrl_object.add_to_graph(&mut graph)?;

    Ok(graph)
}

fn json_ld_from_graph(graph: LightGraph) -> Value {
    let mut nt_stringifier = NtSerializer::new_stringifier();
    let graph_as_str = nt_stringifier.serialize_graph(&graph).unwrap().as_str();
    println!("\n\nThe resulting graph:\n{}", graph_as_str);

    let data = graph.into_dataset();
    let mut jsonifier = JsonLdStringifier::new_stringifier();
    let json_ld_string = jsonifier.serialize_dataset(&data).unwrap().to_string();

    let pretty_print_obj: Value = serde_json::from_str(&json_ld_string).unwrap();
    println!(
        "The resulting JSON-LD from graph (parsed):\n{}\n",
        serde_json::to_string_pretty(&pretty_print_obj).unwrap()
    );

    pretty_print_obj
}

pub fn serialize<T: Serializable>(odrl_object: T, contexts: Vec<&str>) -> Value {
    // Parse the given object to a rdf graph and then to a JSON-LD
    let result = build_graph(&odrl_object, contexts).unwrap();
    json_ld_from_graph(result)
}
