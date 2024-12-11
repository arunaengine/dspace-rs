// Examples based on https://www.w3.org/TR/odrl-model

#[cfg(test)]
mod json_serializer_test {
    extern crate odrl;

    use odrl::model::action::{Action, Refinements};
    use odrl::model::asset::Asset;
    use odrl::model::constraint::{
        Constraint, LeftOperand, LogicalConstraint, LogicalOperator, Operator, RightOperand,
    };
    use odrl::model::party::Party;
    use odrl::model::policy::{AgreementPolicy, OfferPolicy, SetPolicy};
    use odrl::model::rule::{Duty, Obligation, Permission, Prohibition, Rule};

    #[test]
    fn test_set_policy_with_one_rule_serialize() {
        let action = Action::new("use", None, None, vec![]);
        let mut asset = Asset::default();
        asset.uid = Some("https://example.com/asset:9898.movie".to_string());
        let mut permission = Permission::default();
        permission.target = asset;
        permission.action = action;

        let mut policy = SetPolicy::default();
        policy.uid = "https://example.com/policy:1010".to_string();
        policy.rules = vec![Rule::Permission(permission)];

        let serialized_policy = serde_json::to_string_pretty(&policy).unwrap();
        let expected_policy = r#"
        {
            "@type": "Set",
            "uid": "https://example.com/policy:1010",
            "permission": [{
            "target": "https://example.com/asset:9898.movie",
            "action": "use"
            }]
        }"#
        .to_string();

        assert_eq!(
            serialized_policy.parse::<serde_json::Value>().unwrap(),
            expected_policy.parse::<serde_json::Value>().unwrap()
        );
    }

    #[test]
    fn test_set_policy_with_two_rules_serialize() {
        let action_1 = Action::new("play", None, None, vec![]);
        let mut asset_1 = Asset::default();
        asset_1.uid = Some("https://example.com/asset:9898.movie".to_string());
        let mut permission_1 = Permission::default();
        permission_1.target = asset_1;
        permission_1.action = action_1;

        let action_2 = Action::new("use", None, None, vec![]);
        let mut asset_2 = Asset::default();
        asset_2.uid = Some("https://example.com/asset:1212".to_string());
        let mut permission_2 = Permission::default();
        permission_2.target = asset_2;
        permission_2.action = action_2;

        let mut policy = SetPolicy::default();
        policy.uid = "https://example.com/policy:1010".to_string();
        policy.rules = vec![
            Rule::Permission(permission_1),
            Rule::Permission(permission_2),
        ];

        let serialized_policy = serde_json::to_string_pretty(&policy).unwrap();
        let expected_policy = r#"
        {
            "@type": "Set",
            "uid": "https://example.com/policy:1010",
            "permission": [
                {
                    "target": "https://example.com/asset:9898.movie",
                    "action": "play"
                },
                {
                    "target": "https://example.com/asset:1212",
                    "action": "use"
                }
            ]
        }"#
        .to_string();

        assert_eq!(
            serialized_policy.parse::<serde_json::Value>().unwrap(),
            expected_policy.parse::<serde_json::Value>().unwrap()
        );
    }

    #[test]
    fn test_set_policy_with_several_complex_rules_serialize() {
        // Permission 1
        let mut asset_1 = Asset::default();
        asset_1.uid = Some("https://example.com/asset:9898.movie".to_string());
        let mut party_1 = Party::default();
        party_1.uid = Some("https://example.com/party:org:abc".to_string());
        let action_1 = Action::new("play", None, None, vec![]);
        let mut permission_1 = Permission::default();
        permission_1.target = asset_1;
        permission_1.assigner = Some(party_1);
        permission_1.action = action_1;

        // Permission 2
        let mut asset_2 = Asset::default();
        asset_2.uid = Some("https://example.com/asset:1212".to_string());
        let mut party_2 = Party::default();
        party_2.uid = Some("https://example.com/owner:182".to_string());
        let mut action_2 = Action::default();
        action_2.name = "display".to_string();
        let mut permission_2 = Permission::default();
        permission_2.target = asset_2.clone();
        permission_2.assigner = Some(party_2);
        permission_2.action = action_2;

        // Prohibition 1
        let mut action_3 = Action::default();
        action_3.name = "print".to_string();
        let mut prohibition_1 = Prohibition::default();
        prohibition_1.target = asset_2.clone();
        prohibition_1.action = action_3;

        // Permission 3 with Constraints
        let mut asset_3 = Asset::default();
        asset_3.uid = Some("https://example.com/document:1234".to_string());
        let mut party_3 = Party::default();
        party_3.uid = Some("https://example.com/org:616".to_string());
        let mut action_4 = Action::default();
        action_4.name = "distribute".to_string();
        let mut permission_constraint = Constraint::default();
        permission_constraint.left_operand = LeftOperand::Literal("dateTime".to_string());
        permission_constraint.operator = Operator::LessThan;
        permission_constraint.right_operand = RightOperand::Literal("2018-01-01".to_string());
        let mut permission_3 = Permission::default();
        permission_3.target = asset_3.clone();
        permission_3.assigner = Some(party_3);
        permission_3.action = action_4;
        permission_3.constraints = vec![permission_constraint];

        // Obligation 1
        let mut obligation_action = Action::default();
        obligation_action.name = "delete".to_string();
        let mut obligation_asset = Asset::default();
        obligation_asset.uid = Some("https://example.com/document:XZY".to_string());
        let mut obligation_assigner = Party::default();
        obligation_assigner.uid = Some("https://example.com/org:43".to_string());
        let mut obligation_assignee = Party::default();
        obligation_assignee.uid = Some("https://example.com/person:44".to_string());

        // Obligation 2 with action refinement
        let mut consequence = Duty::default();
        let mut consequence_action = Action::default();
        let mut action_refinement = Refinements::default();
        let mut refinement_constraint = Constraint::default();
        refinement_constraint.left_operand = LeftOperand::Literal("payAmount".to_string());
        refinement_constraint.operator = Operator::Equal;
        refinement_constraint.right_operand = RightOperand::Literal("10.00".to_string());
        refinement_constraint.unit = Some("https://dbpedia.org/resource/Euro".to_string());
        let mut refinement_logical = LogicalConstraint::default();
        let c1_iri = "https://example.com/p:88/C1".to_string();
        let c2_iri = "https://example.com/p:88/C2".to_string();
        refinement_logical.operand = Some((LogicalOperator::Xone, vec![c1_iri, c2_iri]));
        action_refinement = Refinements::Constraints(vec![refinement_constraint]);
        consequence_action.name = "compensate".to_string();
        consequence_action.refinements = Some(action_refinement);
        let mut consequence_party = Party::default();
        consequence_party.uid = Some("https://wwf.org".to_string());
        consequence.function = vec![consequence_party];
        consequence.action = consequence_action;
        let mut obligation_1 = Obligation::default();
        obligation_1.action = obligation_action;
        obligation_1.target = obligation_asset;
        obligation_1.consequence = vec![consequence];
        obligation_1.assigner = obligation_assigner;
        obligation_1.assignee = obligation_assignee;

        let mut policy = SetPolicy::default();
        policy.uid = "https://example.com/policy:1011".to_string();
        policy.profiles = vec!["https://example.com/odrl:profile:01".to_string()];
        policy.rules = vec![
            Rule::Permission(permission_1),
            Rule::Permission(permission_2),
            Rule::Permission(permission_3),
            Rule::Prohibition(prohibition_1),
            Rule::Obligation(obligation_1),
        ];
        let serialized_policy = serde_json::to_string_pretty(&policy).unwrap();

        let expected_policy = r#"
        {
            "@type": "Set",
            "uid": "https://example.com/policy:1011",
            "profile": "https://example.com/odrl:profile:01",
            "permission": [
                {
                    "action": "play",
                    "assigner": "https://example.com/party:org:abc",
                    "target": "https://example.com/asset:9898.movie"
                },
                {
                    "action": "display",
                    "assigner": "https://example.com/owner:182",
                    "target": "https://example.com/asset:1212"
                },
                {
                    "action": "distribute",
                    "assigner": "https://example.com/org:616",
                    "constraint": [
                        {
                            "leftOperand": "dateTime",
                            "operator": "lt",
                            "rightOperand": "2018-01-01"
                        }
                    ],
                    "target": "https://example.com/document:1234"
                }
            ],
            "prohibition": [
                {
                    "action": "print",
                    "target": "https://example.com/asset:1212"
                }
            ],
            "obligation": [
                {
                    "action": "delete",
                    "assignee": "https://example.com/person:44",
                    "assigner": "https://example.com/org:43",
                    "consequence": [
                        {
                            "action": {
                                "name": "compensate",
                                "refinement": [{
                                    "leftOperand": "payAmount",
                                    "operator": "eq",
                                    "rightOperand": "10.00",
                                    "unit": "https://dbpedia.org/resource/Euro"
                                }]
                            },
                            "compensatedParty": "https://wwf.org"
                        }
                    ],
                    "target": "https://example.com/document:XZY"
                }
            ]
        }"#
        .to_string();

        assert_eq!(
            serialized_policy.parse::<serde_json::Value>().unwrap(),
            expected_policy.parse::<serde_json::Value>().unwrap()
        );
    }

    #[test]
    fn test_offer_policy_serialize() {
        let action = Action::new("play", None, None, vec![]);
        let mut asset = Asset::default();
        asset.uid = Some("https://example.com/asset:9898.movie".to_string());
        let mut permission = Permission::default();
        permission.target = asset;
        permission.action = action;

        let mut assigner = Party::default();
        assigner.uid = Some("https://example.com/party:org:abc".to_string());

        let mut policy = OfferPolicy::default();
        policy.uid = "https://example.com/policy:1011".to_string();
        policy.rules = vec![Rule::Permission(permission)];
        policy.assigner = assigner;
        policy.profiles = vec!["https://example.com/odrl:profile:01".to_string()];

        let serialized_policy = serde_json::to_string_pretty(&policy).unwrap();
        let expected_policy = r#"
        {
            "@type": "Offer",
            "uid": "https://example.com/policy:1011",
            "profile": "https://example.com/odrl:profile:01",
            "assigner": "https://example.com/party:org:abc",
            "permission": [{
                "target": "https://example.com/asset:9898.movie",
                "action": "play"
            }]
        }"#
        .to_string();

        assert_eq!(
            serialized_policy.parse::<serde_json::Value>().unwrap(),
            expected_policy.parse::<serde_json::Value>().unwrap()
        );
    }

    #[test]
    fn test_agreement_policy_serialize() {
        let action = Action::new("play", None, None, vec![]);
        let mut asset = Asset::default();
        asset.uid = Some("https://example.com/asset:9898.movie".to_string());
        let mut permission = Permission::default();
        permission.target = asset;
        permission.action = action;

        let mut assigner = Party::default();
        assigner.uid = Some("https://example.com/party:org:abc".to_string());
        let mut assignee = Party::default();
        assignee.uid = Some("https://example.com/party:person:billie".to_string());

        let mut policy = AgreementPolicy::default();
        policy.uid = "https://example.com/policy:1012".to_string();
        policy.rules = vec![Rule::Permission(permission)];
        policy.assigner = assigner;
        policy.assignee = assignee;
        policy.profiles = vec!["https://example.com/odrl:profile:01".to_string()];

        let serialized_policy = serde_json::to_string_pretty(&policy).unwrap();
        let expected_policy = r#"
        {
            "@type": "Agreement",
            "uid": "https://example.com/policy:1012",
            "profile": "https://example.com/odrl:profile:01",
            "assigner": "https://example.com/party:org:abc",
            "assignee": "https://example.com/party:person:billie",
            "permission": [{
                "target": "https://example.com/asset:9898.movie",
                "action": "play"
            }]
        }"#
        .to_string();

        assert_eq!(
            serialized_policy.parse::<serde_json::Value>().unwrap(),
            expected_policy.parse::<serde_json::Value>().unwrap()
        );
    }
}
