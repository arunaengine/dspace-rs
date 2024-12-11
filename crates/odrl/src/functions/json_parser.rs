use serde_json::{Result, Value};
use crate::model::action::Action;
use crate::model::action::Refinements;
use crate::model::asset::{Asset};
use crate::model::conflict_term::ConflictTerm;
use crate::model::constraint::{Constraint, LeftOperand, Operator, RightOperand, LogicalConstraint, LogicalOperator};
use crate::model::party::{Function, Party};
use crate::model::policy::{AgreementPolicy, OfferPolicy, Policy, SetPolicy};
use crate::model::rule::{Duty, Obligation, Permission, Prohibition, Rule};
use crate::model::type_alias::IRI;


#[derive(Debug, Deserialize)]
pub struct ParsedData {
    pub parsed_policies: Vec<Policy>,
    pub parsed_constraints: Vec<Constraint>,
}

impl ParsedData {
    pub fn new(parsed_policies: Vec<Policy>, parsed_constraints: Vec<Constraint>) -> ParsedData {
        ParsedData {
            parsed_policies,
            parsed_constraints,
        }
    }
}


fn parse_policy(policy: &Value) -> Result<Policy> {

    /* fields of policies:
        uid: IRI,
        rules: Vec<Rule>,
        profiles: Vec<IRI>,
        inherit_from: Vec<IRI>,
        conflict: Option<ConflictTerm>,
        obligation: Vec<Rule>,

        assigner: Party,        | part of OfferPolicy and AgreementPolicy
        assignee: Party,        | part of AgreementPolicy
     */

    let policy_uid = policy.get("uid").ok_or("No uid field").unwrap().to_string().replace("\"", "");

    let mut rules_vec: Vec<Rule> = vec![];

    /* Parse rules */
    if let Some(permissions) = policy.get("permission").and_then(|x| x.as_array()) {
        for permission in permissions {
            let parsed_rule = parse_rule("permission", permission)?;
            rules_vec.push(parsed_rule);
        }
    }
    if let Some(prohibitions) = policy.get("prohibition").and_then(|x| x.as_array()) {
        for prohibition in prohibitions {
            let parsed_rule = parse_rule("prohibition", prohibition)?;
            rules_vec.push(parsed_rule);
        }
    }
    if let Some(duties) = policy.get("duty").and_then(|x| x.as_array()) {
        for duty in duties {
            let parsed_rule = parse_rule("duty", duty)?;
            rules_vec.push(parsed_rule);
        }
    }
    if let Some(obligations) = policy.get("obligation").and_then(|x| x.as_array()) {
        for obligation in obligations {
            let parsed_rule = parse_rule("obligation", obligation)?;
            rules_vec.push(parsed_rule);
        }
    }

    let mut profiles_vec: Vec<IRI> = vec![];

    if let Some(profiles) = policy.get("profiles").and_then(|x| x.as_array()) {
        for profile in profiles {
            let profile_uid = profile.to_string().replace("\"", "");
            profiles_vec.push(profile_uid);
        }
    }

    let mut inherit_from_vec: Vec<IRI> = vec![];

    if let Some(inherits) = policy.get("inheritFrom").and_then(|x| x.as_array()) {
        for inherit in inherits {
            let inherit_uid = inherit.to_string().replace("\"", "");
            inherit_from_vec.push(inherit_uid);
        }
    }

    let mut conflict_term: Option<ConflictTerm> = None;

    if let Some(conflict) = policy["conflict"].as_str() {
        match conflict {
            "perm" => conflict_term = Some(ConflictTerm::Perm),
            "prohibit" => conflict_term = Some(ConflictTerm::Prohibit),
            "invalid" => conflict_term = Some(ConflictTerm::Invalid),
            _ => conflict_term = None,
        }
    }

    let mut policy_obj: Policy = Default::default();

    /* Parse policy type */
    if let Some(type_) = policy.get("@type") {
        if type_ == "set" || type_ == "Set"{
            let set_policy: Policy = Policy::SetPolicy(SetPolicy::new(policy_uid, rules_vec, profiles_vec, inherit_from_vec, conflict_term, vec![], None, None));
            policy_obj = set_policy;
        } else if type_ == "offer" || type_ == "Offer" {
            if policy.is_object() {
                // Check for permission, prohibition, duty, obligation
                if let Some(permissions) = policy.get("permission").and_then(|x| x.as_array()) {
                    // get assigner from one of the permissions
                    for perm in permissions {
                        if perm.get("assigner").is_some() {
                            let assigner_uid = perm.get("assigner").ok_or("No assigner field").unwrap();
                            let assigner = Party::new(Some(assigner_uid.to_string().replace("\"", "")), vec![], Function::Assigner, None);
                            let offer_policy: Policy = Policy::OfferPolicy(OfferPolicy::new(policy_uid, assigner, rules_vec, profiles_vec, inherit_from_vec, conflict_term, vec![], None, None));
                            policy_obj = offer_policy;
                            break;
                        }
                    }
                } else if let Some(prohibitions) = policy.get("prohibition").and_then(|x| x.as_array()) {
                    // get assigner from one of the prohibitions
                    for proh in prohibitions {
                        if proh.get("assigner").is_some() {
                            let assigner_uid = proh.get("assigner").ok_or("No assigner field").unwrap();
                            let assigner = Party::new(Some(assigner_uid.to_string().replace("\"", "")), vec![], Function::Assigner, None);
                            let offer_policy: Policy = Policy::OfferPolicy(OfferPolicy::new(policy_uid, assigner, rules_vec, profiles_vec, inherit_from_vec, conflict_term, vec![], None, None));
                            policy_obj = offer_policy;
                            break;
                        }
                    }
                } else if let Some(duties) = policy.get("duty").and_then(|x| x.as_array()) {
                    // get assigner from one of the duties
                    for duty in duties {
                        if duty.get("assigner").is_some() {
                            let assigner_uid = duty.get("assigner").ok_or("No assigner field").unwrap();
                            let assigner = Party::new(Some(assigner_uid.to_string().replace("\"", "")), vec![], Function::Assigner, None);
                            let offer_policy: Policy = Policy::OfferPolicy(OfferPolicy::new(policy_uid, assigner, rules_vec, profiles_vec, inherit_from_vec, conflict_term, vec![], None, None));
                            policy_obj = offer_policy;
                            break;
                        }
                    }
                } else if let Some(obligations) = policy.get("obligation").and_then(|x| x.as_array()) {
                    // get assigner from one of the obligations
                    for obli in obligations {
                        if obli.get("assigner").is_some() {
                            let assigner_uid = obli.get("assigner").ok_or("No assigner field").unwrap();
                            let assigner = Party::new(Some(assigner_uid.to_string().replace("\"", "")), vec![], Function::Assigner, None);
                            let offer_policy: Policy = Policy::OfferPolicy(OfferPolicy::new(policy_uid, assigner, rules_vec, profiles_vec, inherit_from_vec, conflict_term, vec![], None, None));
                            policy_obj = offer_policy;
                            break;
                        }
                    }
                }
            } else if policy.is_array() {
                let assigner_uid = policy.get("assigner").ok_or("No assigner field").unwrap();
                let assigner = Party::new(Some(assigner_uid.to_string().replace("\"", "")), vec![], Function::Assigner, None);
                let offer_policy: Policy = Policy::OfferPolicy(OfferPolicy::new(policy_uid, assigner, rules_vec, profiles_vec, inherit_from_vec, conflict_term, vec![], None, None));
                policy_obj = offer_policy;
            }
        }
        else if type_ == "agreement" || type_ == "Agreement"{
            if policy.is_object() {
                if let Some(permissions) = policy.get("permission").and_then(|x| x.as_array()) {
                    // get assigner and assignee from the permission
                    for perm in permissions {
                        if perm.get("assigner").is_some() && perm.get("assignee").is_some() {
                            let assigner_uid = perm.get("assigner").ok_or("No assigner field").unwrap();
                            let assigner = Party::new(Some(assigner_uid.to_string().replace("\"", "")), vec![], Function::Assigner, None);
                            let assignee_uid = perm.get("assignee").ok_or("No assignee field").unwrap();
                            let assignee = Party::new(Some(assignee_uid.to_string().replace("\"", "")), vec![], Function::Assignee, None);
                            let agreement_policy: Policy = Policy::AgreementPolicy(AgreementPolicy::new(policy_uid, assigner, assignee, rules_vec, profiles_vec, inherit_from_vec, conflict_term, vec![], None, None));
                            policy_obj = agreement_policy;
                            break;
                        }
                    }
                } else if let Some(prohibitions) = policy.get("prohibition").and_then(|x| x.as_array()) {
                    // get assigner and assignee from the prohibition
                    for proh in prohibitions {
                        if proh.get("assigner").is_some() && proh.get("assignee").is_some() {
                            let assigner_uid = proh.get("assigner").ok_or("No assigner field").unwrap();
                            let assigner = Party::new(Some(assigner_uid.to_string().replace("\"", "")), vec![], Function::Assigner, None);
                            let assignee_uid = proh.get("assignee").ok_or("No assignee field").unwrap();
                            let assignee = Party::new(Some(assignee_uid.to_string().replace("\"", "")), vec![], Function::Assignee, None);
                            let agreement_policy: Policy = Policy::AgreementPolicy(AgreementPolicy::new(policy_uid, assigner, assignee, rules_vec, profiles_vec, inherit_from_vec, conflict_term, vec![], None, None));
                            policy_obj = agreement_policy;
                            break;
                        }
                    }
                } else if let Some(duties) = policy.get("duty").and_then(|x| x.as_array()) {
                    // get assigner and assignee from the duty
                    for duty in duties {
                        if duty.get("assigner").is_some() && duty.get("assignee").is_some() {
                            let assigner_uid = duty.get("assigner").ok_or("No assigner field").unwrap();
                            let assigner = Party::new(Some(assigner_uid.to_string().replace("\"", "")), vec![], Function::Assigner, None);
                            let assignee_uid = duty.get("assignee").ok_or("No assignee field").unwrap();
                            let assignee = Party::new(Some(assignee_uid.to_string().replace("\"", "")), vec![], Function::Assignee, None);
                            let agreement_policy: Policy = Policy::AgreementPolicy(AgreementPolicy::new(policy_uid, assigner, assignee, rules_vec, profiles_vec, inherit_from_vec, conflict_term, vec![], None, None));
                            policy_obj = agreement_policy;
                            break;
                        }
                    }
                } else if let Some(obligations) = policy.get("obligation").and_then(|x| x.as_array()) {
                    // get assigner and assignee from the obligation
                    for obli in obligations {
                        if obli.get("assigner").is_some() && obli.get("assignee").is_some() {
                            let assigner_uid = obli.get("assigner").ok_or("No assigner field").unwrap();
                            let assigner = Party::new(Some(assigner_uid.to_string().replace("\"", "")), vec![], Function::Assigner, None);
                            let assignee_uid = obli.get("assignee").ok_or("No assignee field").unwrap();
                            let assignee = Party::new(Some(assignee_uid.to_string().replace("\"", "")), vec![], Function::Assignee, None);
                            let agreement_policy: Policy = Policy::AgreementPolicy(AgreementPolicy::new(policy_uid, assigner, assignee, rules_vec, profiles_vec, inherit_from_vec, conflict_term, vec![], None, None));
                            policy_obj = agreement_policy;
                            break;
                        }
                    }
                }
            } else if policy.is_array() {
                let assigner_uid = policy.get("assigner").ok_or("No assigner field").unwrap();
                let assigner = Party::new(Some(assigner_uid.to_string()), vec![], Function::Assigner, None);
                let assignee_uid = policy.get("assignee").ok_or("No assignee field").unwrap();
                let assignee = Party::new(Some(assignee_uid.to_string()), vec![], Function::Assignee, None);
                let agreement_policy: Policy = Policy::AgreementPolicy(AgreementPolicy::new(policy_uid, assigner, assignee, rules_vec, profiles_vec, inherit_from_vec, conflict_term, vec![], None, None));
                policy_obj = agreement_policy;
            }
        } else {
            println!("Unknown policy type: {}\n", type_);
        }
    }

    Ok(policy_obj)
}

fn parse_rule(rule_type: &str, rule: &Value) -> Result<Rule> {

    /* Common fields for rules:
        uid: Option<IRI>,
        action: Action,
        relation: Option<Asset>,           | not part of Duty
        function: Vec<Party>,              | not part of Duty
        failures: Vec<Rule>,               | not part of Duty
        constraints: Vec<Constraint>,      | not part of Duty

        target: Asset,
        assigner: Option<Party>,
        assignee: Option<Party>,

     */

    let uid: Option<IRI>;
    if let Some(rule_uid) = rule.get("uid") {
        uid = Some(rule_uid.to_string().replace("\"", ""));
    } else {
        uid = None;
    }

    let action: Action;

    match rule.get("action") {
        Some(action_value) => {
            action = parse_action(action_value)?;
        },
        None => {
            action = Action::new("Unknown", None, None, vec![]);
            println!("No action field\n")
        },
    }

    let target = rule.get("target").ok_or("No target field").unwrap().to_string().replace("\"", "");
    let asset = Asset::new(None, Some(target), None, vec![], None, None, None, None, None, None);

    let assigner: Option<Party>;
    let assignee: Option<Party>;

    if let Some(assigner_uid) = rule.get("assigner") {
        assigner = Some(Party::new(Some(assigner_uid.to_string().replace("\"", "")), vec![], Function::Assigner, None));
    } else {
        assigner = None;
    }

    if let Some(assignee_uid) = rule.get("assignee") {
        assignee = Some(Party::new(Some(assignee_uid.to_string().replace("\"", "")), vec![], Function::Assignee, None));
    } else {
        assignee = None;
    }

    let mut constraint_vec: Vec<Constraint> = vec![];

    if let Some(constraints) = rule.get("constraint").and_then(|x| x.as_array()) {
        for constraint in constraints {
            let parsed_constraint = parse_constraint(constraint)?;
            constraint_vec.push(parsed_constraint);
        }
    }

    let parsed_rule: Rule;

    match rule_type {
        "permission" => {
            /* permission specific fields:
                duties: Vec<Duty>,
             */
            let mut duties_vec: Vec<Duty> = vec![];

            if let Some(duties) = rule.get("duty").and_then(|x| x.as_array()) {
                for duty in duties {
                    let parsed_duty = parse_rule("duty", duty)?;
                    match parsed_duty {
                        Rule::Duty(duty) => duties_vec.push(duty),
                        _ => println!("Unknown rule type in duties: {}\n", rule_type),
                    }
                }
            }

            parsed_rule = Rule::Permission(Permission::new(uid, action, None, vec![], vec![], constraint_vec, asset, assigner, assignee, duties_vec));

        },
        "prohibition" => {
            /* prohibition specific fields:
                remedies: Vec<Duty>,
             */
            let mut remedies_vec: Vec<Duty> = vec![];

            if let Some(remedies) = rule.get("remedy").and_then(|x| x.as_array()) {
                for remedy in remedies {
                    let parsed_remedy = parse_rule("duty", remedy)?;
                    match parsed_remedy {
                        Rule::Duty(remedy) => remedies_vec.push(remedy),
                        _ => println!("Unknown rule type in remedies: {}\n", rule_type),
                    }
                }
            }

            parsed_rule = Rule::Prohibition(Prohibition::new(uid, action, None, vec![], vec![], constraint_vec, asset, assigner, assignee, remedies_vec));

        },
        "duty" => {
            /* duty specific fields:
                consequences: Vec<Duty>,
                pre_condition: Option<Vec<Duty>>,
             */
            let mut consequences_vec: Vec<Duty> = vec![];
            let mut pre_conditions_vec: Vec<Duty> = vec![];

            if let Some(consequences) = rule.get("consequence").and_then(|x| x.as_array()) {
                for consequence in consequences {
                    let parsed_consequence = parse_rule("duty", consequence)?;
                    match parsed_consequence {
                        Rule::Duty(consequence) => consequences_vec.push(consequence),
                        _ => println!("Unknown rule type in consequences: {}\n", rule_type),
                    }
                }
            }
            if let Some(pre_conditions) = rule.get("preCondition").and_then(|x| x.as_array()) {
                for pre_condition in pre_conditions {
                    let parsed_pre_condition = parse_rule("duty", pre_condition)?;
                    match parsed_pre_condition {
                        Rule::Duty(pre_condition) => pre_conditions_vec.push(pre_condition),
                        _ => println!("Unknown rule type in pre_conditions: {}\n", rule_type),
                    }
                }
            }

            if pre_conditions_vec.len() > 0 {
                parsed_rule = Rule::Duty(Duty::new(uid, action, None, vec![], vec![], constraint_vec, Some(asset), assigner, assignee, consequences_vec, Some(pre_conditions_vec)));
            } else {
                parsed_rule = Rule::Duty(Duty::new(uid, action, None, vec![], vec![], constraint_vec, Some(asset), assigner, assignee, consequences_vec, None));
            }

        },
        "obligation" => {
            /* obligation specific fields:
                consequence: Vec<Duty>,
             */
            let mut consequences_vec: Vec<Duty> = vec![];

            if let Some(consequences) = rule.get("consequence").and_then(|x| x.as_array()) {
                for consequence in consequences {
                    let parsed_consequence = parse_rule("duty", consequence)?;
                    match parsed_consequence {
                        Rule::Duty(consequence) => consequences_vec.push(consequence),
                        _ => println!("Unknown rule type in consequences: {}\n", rule_type),
                    }
                }
            }

            parsed_rule = Rule::Obligation(Obligation::new(uid, asset, assigner.unwrap(), assignee.unwrap(), action, consequences_vec));

        }
        _ => {
            parsed_rule = Rule::Permission(Permission::new(None, action, None, vec![], vec![], vec![], asset, None, None, vec![]));
            println!("Unknown rule type: {}\n", rule_type);
        }
    }

    Ok(parsed_rule)

}

fn parse_action(action: &Value) -> Result<Action> {

    let mut action_name: String = String::new();
    let mut refinements: Option<Refinements> = None;

    if action.is_string() {
        action_name = action.to_string().replace("\"", "");
    } else if action.is_array() {

        for action_item in action.as_array().unwrap() {
            let rdf_value = action_item
                .get("rdf:value")
                .and_then(|value| value.get("@id"))
                .and_then(|id| id.as_str());
            action_name = rdf_value.unwrap().to_string().replace("\"", "").replace("odrl:", "");

            if let Some(refinement_value) = action_item.get("refinement") {
                if refinement_value.is_object() {
                    let parsed_refinement = parse_logical_constraint(refinement_value)?;
                    refinements = Some(Refinements::LogicalConstraints(vec![parsed_refinement.clone()]));
                } else if refinement_value.is_array() {
                    for refinement in refinement_value.as_array().unwrap() {
                        let parsed_refinement = parse_constraint(refinement)?;
                        refinements = Some(Refinements::Constraints(vec![parsed_refinement]));
                    }
                }
            }
        }

    }

    let action_obj = Action::new(action_name.as_str(), refinements, None, vec![]);

    Ok(action_obj)

}

fn parse_constraint(constraint: &Value) -> Result<Constraint> {

    /* fields for constraints:
        uid: Option<IRI>,
        left_operand: LeftOperand,
        operator: Operator,
        right_operand: RightOperand,
        data_type: Option<IRI>,
        unit: Option<IRI>,
        status: Option<IRI>,
     */

    let uid: Option<IRI>;
    if let Some(rule_uid) = constraint.get("uid") {
        uid = Some(rule_uid.to_string().replace("\"", ""));
    } else {
        uid = None;
    }

    let left_operand: LeftOperand;

    if let Some(left_operand_value) = constraint["leftOperand"].as_str() {
        // Check if it's an IRI / Reference
        if left_operand_value.starts_with("http://") || left_operand_value.starts_with("https://") {
            left_operand = LeftOperand::IRI(left_operand_value.to_string());
        } else { // Literal
            left_operand = LeftOperand::Literal(left_operand_value.to_string());
        }
    } else {
        left_operand = LeftOperand::Literal("Unknown".to_string());
    }

    let operator: Operator;

    if let Some(operator_value) = constraint["operator"].as_str() {
        match operator_value {
            "eq" => operator = Operator::Equal,
            "neq" => operator = Operator::NotEqual,
            "gt" => operator = Operator::GreaterThan,
            "lt" => operator = Operator::LessThan,
            "gteq" => operator = Operator::GreaterThanOrEqual,
            "lteq" => operator = Operator::LessThanOrEqual,
            _ => operator = Operator::Equal,
        }
    } else {
        operator = Operator::Equal;
    }

    let right_operand: RightOperand;

    if let Some(right_operand_value) = constraint["rightOperand"].as_str() {
        // Check if it's an IRI / Reference
        if right_operand_value.starts_with("http://") || right_operand_value.starts_with("https://") {
            right_operand = RightOperand::IRI(right_operand_value.to_string());
        } else { // Literal
            right_operand = RightOperand::Literal(right_operand_value.to_string());
        }
    } else if let Some(value_obj) = constraint["rightOperand"].as_object() {
        if let Some(value) = value_obj.get("@value").and_then(|v| v.as_str()) {
            if value.starts_with("http://") || value.starts_with("https://") {
                right_operand = RightOperand::IRI(value.to_string());
            } else {
                right_operand = RightOperand::Literal(value.to_string());
            }
        } else {
            right_operand = RightOperand::Literal("Unknown".to_string());
        }
    } else {
        right_operand = RightOperand::Literal("Unknown".to_string());
    }

    let unit: Option<IRI>;

    if let Some(unit_value) = constraint["unit"].as_str() {
        unit = Some(unit_value.to_string());
    } else {
        unit = None;
    }

    let constraint_obj = Constraint::new(uid, left_operand, operator, right_operand, None, unit, "".to_string());

    Ok(constraint_obj)
}

fn parse_logical_constraint(logical_constraint: &Value) -> Result<LogicalConstraint> {

    let mut operator: LogicalOperator = LogicalOperator::And;
    let mut constraint_vec: Vec<IRI> = vec![];

    if let Some(obj) = logical_constraint.as_object() {
        // Logical Constraints always contain just one key value pair, therefore there's no need to iterate over more than one element
        // Thus, we can use obj.iter().next() to get the first and only element
        // The key is the logical operator and the value is a list of constraints
        if let Some((logical_type, value)) = obj.iter().next() {
            operator = match logical_type.as_str() {
                "xone" => LogicalOperator::Xone,
                "or" => LogicalOperator::Or,
                "and" => LogicalOperator::And,
                "andSequence" => LogicalOperator::AndSequence,
                _ => LogicalOperator::And,
            };
            let list = value.get("@list").and_then(|l| l.as_array()).unwrap();
            for element in list {
                for (_tag, uid) in element.as_object().unwrap() {       // _tag always equals "@id" in jsonld
                    constraint_vec.push(uid.to_string().replace("\"", ""));
                }
            }
        }
    }

    let operand = (operator, constraint_vec);

    let logical_constraint_obj = LogicalConstraint::new(None, Some(operand));

    Ok(logical_constraint_obj)

}


pub fn parse(json_data: &str) -> Result<ParsedData> {

    // Deserialize JSON data into serde_json::Value
    let v: Value = serde_json::from_str(json_data)?;

    let mut parsed_policies = Vec::new();
    let mut parsed_constraints = Vec::new();

    if v.is_array() {
        for entry in v.as_array().unwrap() {
            if let Some(policy) = entry.get("policy") {
                let parsed_policy = parse_policy(policy)?;
                parsed_policies.push(parsed_policy);
                //println!("Parsed Policy: {:#?}\n", parsed_policy);
            }

            // JsonLD-representation
            if let Some(object_type) = entry.get("@type").and_then(|x| x.as_str()) {
                if object_type.to_lowercase() == "constraint" {
                    let parsed_constraint = parse_constraint(entry)?;
                    parsed_constraints.push(parsed_constraint);
                    //println!("Parsed Constraint: {:#?}\n", parsed_constraint);
                } else if object_type.to_lowercase() == "set" || object_type.to_lowercase() == "offer" || object_type.to_lowercase() == "agreement" {
                    let parsed_policy = parse_policy(entry)?;
                    parsed_policies.push(parsed_policy);
                    //println!("Parsed Policy: {:#?}\n", parsed_policy);
                }
            }
        }
    } else if v.is_object() {
        let obj = v.as_object().unwrap();
        if obj.contains_key("policy") {
            let parsed_policy = parse_policy(obj.get("policy").unwrap())?;
            parsed_policies.push(parsed_policy);
        }
        if let Some(object_type) = obj.get("@type").and_then(|x| x.as_str()) {
            if object_type.to_lowercase() == "constraint" {
                let parsed_constraint = parse_constraint(&v)?;
                parsed_constraints.push(parsed_constraint);
            } else if object_type.to_lowercase() == "set" || object_type.to_lowercase() == "offer" || object_type.to_lowercase() == "agreement" {
                let parsed_policy = parse_policy(&v)?;
                parsed_policies.push(parsed_policy);
            }
        }
    }

    let parsed_data = ParsedData::new(parsed_policies, parsed_constraints);

    Ok(parsed_data)

}