#[cfg(test)]
mod json_parser_test {
    extern crate odrl;

    use odrl::functions::json_parser;
    use odrl::model::action::Refinements;
    use odrl::model::constraint::{
        Constraint, LeftOperand, LogicalConstraint, LogicalOperator, Operator, RightOperand,
    };
    use odrl::model::party::Function::{Assignee, Assigner};
    use odrl::model::{action, asset, party, policy, rule};

    #[test]
    fn test_parse_json_set_policy() {
        let json_set_policy = r#"
        {
            "@context": "http://www.w3.org/ns/odrl.jsonld",
            "@type": "Set",
            "uid": "https://example.com/policy:1010",
            "permission": [{
                "target": "https://example.com/asset:9898.movie",
                "action": "use"
            }]
        }
        "#;

        let parsed_data = json_parser::parse(json_set_policy).unwrap();
        assert_eq!(parsed_data.parsed_policies.len(), 1);

        let comp_asset = asset::Asset {
            uid: Option::from("https://example.com/asset:9898.movie".to_string()),
            ..Default::default()
        };

        let comp_action = action::Action {
            name: "use".to_string(),
            ..Default::default()
        };

        let comp_rule = rule::Rule::Permission(rule::Permission {
            target: comp_asset,
            action: comp_action,
            ..Default::default()
        });

        let comp_policy = policy::Policy::SetPolicy(policy::SetPolicy {
            uid: "https://example.com/policy:1010".to_string(),
            rules: vec![comp_rule],
            ..Default::default()
        });

        assert_eq!(parsed_data.parsed_policies[0], comp_policy);
    }

    #[test]
    fn test_parse_json_offer_policy() {
        let json_offer_policy = r#"
        {
            "@context": "http://www.w3.org/ns/odrl.jsonld",
            "@type": "Offer",
            "uid": "https://example.com/policy:1011",
            "profile": "https://example.com/odrl:profile:01",
                "permission": [{
                "target": "https://example.com/asset:9898.movie",
                "assigner": "https://example.com/party:org:abc",
                "action": "play"
            }]
        }
        "#;

        let parsed_data = json_parser::parse(json_offer_policy).unwrap();
        assert_eq!(parsed_data.parsed_policies.len(), 1);

        let comp_asset = asset::Asset {
            uid: Option::from("https://example.com/asset:9898.movie".to_string()),
            ..Default::default()
        };

        let comp_action = action::Action {
            name: "play".to_string(),
            ..Default::default()
        };

        let comp_party = party::Party {
            uid: Option::from("https://example.com/party:org:abc".to_string()),
            function: Assigner,
            ..Default::default()
        };

        let comp_rule = rule::Rule::Permission(rule::Permission {
            target: comp_asset,
            action: comp_action,
            assigner: Some(comp_party.clone()),
            ..Default::default()
        });

        let comp_policy = policy::Policy::OfferPolicy(policy::OfferPolicy {
            uid: "https://example.com/policy:1011".to_string(),
            rules: vec![comp_rule],
            assigner: comp_party.clone(),
            ..Default::default()
        });

        assert_eq!(parsed_data.parsed_policies[0], comp_policy);
    }

    #[test]
    fn test_parse_json_agreement_policy() {
        let json_agreement_policy = r#"
        {
            "@context": "http://www.w3.org/ns/odrl.jsonld",
            "@type": "Agreement",
            "uid": "https://example.com/policy:1012",
            "profile": "https://example.com/odrl:profile:01",
            "permission": [{
                "target": "https://example.com/asset:9898.movie",
                "assigner": "https://example.com/party:org:abc",
                "assignee": "https://example.com/party:person:billie",
                "action": "play"
            }]
        }
        "#;

        let parsed_data = json_parser::parse(json_agreement_policy).unwrap();
        assert_eq!(parsed_data.parsed_policies.len(), 1);

        let comp_asset = asset::Asset {
            uid: Option::from("https://example.com/asset:9898.movie".to_string()),
            ..Default::default()
        };

        let comp_action = action::Action {
            name: "play".to_string(),
            ..Default::default()
        };

        let comp_assigner = party::Party {
            uid: Option::from("https://example.com/party:org:abc".to_string()),
            function: Assigner,
            ..Default::default()
        };

        let comp_assignee = party::Party {
            uid: Option::from("https://example.com/party:person:billie".to_string()),
            function: Assignee,
            ..Default::default()
        };

        let comp_rule = rule::Rule::Permission(rule::Permission {
            target: comp_asset,
            action: comp_action,
            assigner: Some(comp_assigner.clone()),
            assignee: Some(comp_assignee.clone()),
            ..Default::default()
        });

        let comp_policy = policy::Policy::AgreementPolicy(policy::AgreementPolicy {
            uid: "https://example.com/policy:1012".to_string(),
            rules: vec![comp_rule],
            assigner: comp_assigner.clone(),
            assignee: comp_assignee.clone(),
            ..Default::default()
        });

        assert_eq!(parsed_data.parsed_policies[0], comp_policy);
    }

    #[test]
    fn test_json_policy_and_constraint() {
        let json_policy_and_constraint = r#"
        [{
            "@context": {
                "@vocab": "https://w3id.org/edc/v0.0.1/ns/"
            },
            "@id": "definition-id",
            "policy": {
                "@context": "http://www.w3.org/ns/odrl.jsonld",
                "@type": "Set",
                "uid": "https://example.com/policy:1010",
                "permission": [
                    {
                        "target": "https://example.com/asset:9898.movie",
                        "action": "play",
                        "constraint": [
                            {
                                "leftOperand": "spatial",
                                "operator": "eq",
                                "rightOperand": "https://www.wikidata.org/wiki/Q183",
                                "comment": "i.e. Germany"
                            }
                        ]
                    },
                    {
                        "target": "https://example.com/document:1234",
                        "assigner": "https://example.com/org:616",
                        "action": [
                            {
                            "rdf:value": { "@id": "odrl:print" },
                            "refinement": [
                                {
                                    "leftOperand": "resolution",
                                    "operator": "lteq",
                                    "rightOperand": { "@value": "1200", "@type": "xsd:integer" },
                                    "unit": "https://dbpedia.org/resource/Dots_per_inch"
                                }
                            ]
                            }
                        ]
                    },
                    {
                        "target": "https://example.com/book/1999",
                        "assigner": "https://example.com/org/paisley-park",
                        "action": [
                            {
                                "rdf:value": { "@id": "odrl:reproduce" },
                                "refinement":
                                    {
                                        "xone":
                                            {
	                                            "@list": [
		                                            { "@id": "https://example.com/p:88/C1" },
                                                    { "@id": "https://example.com/p:88/C2" }
		                                        ]
	                                        }
                                    }
                            }
                        ]
                    }
                ],
                "prohibition": [
                    {
                        "target": "https://example.com/photoAlbum:55",
                        "action": "archive",
                        "assigner": "https://example.com/MyPix:55",
                        "assignee": "https://example.com/assignee:55"
                    }
                ]
            }
        },
        {
            "@context": "http://www.w3.org/ns/odrl.jsonld",
            "@type": "Constraint",
            "uid": "https://example.com/p:88/C1",
            "leftOperand": "media",
            "operator": "eq",
            "rightOperand": { "@value": "online", "@type": "xsd:string" }
        },
        {
            "@context": "http://www.w3.org/ns/odrl.jsonld",
            "@type": "Constraint",
            "uid": "https://example.com/p:88/C2",
            "leftOperand": "media",
            "operator": "eq",
            "rightOperand": { "@value": "print", "@type": "xsd:string" }
        },
        {
            "@context": "http://www.w3.org/ns/odrl.jsonld",
            "@type": "Set",
            "uid": "https://example.com/policy:1010",
            "permission": [{
                "target": "https://example.com/asset:9898.movie",
                "action": "use"
            }]
        }]
        "#;

        let parsed_data = json_parser::parse(json_policy_and_constraint).unwrap();
        assert_eq!(parsed_data.parsed_policies.len(), 2);
        assert_eq!(parsed_data.parsed_constraints.len(), 2);

        let mut policies: Vec<policy::Policy> = Vec::new();

        // First Policy
        let first_policy_first_permission = rule::Permission {
            target: asset::Asset {
                uid: Option::from("https://example.com/asset:9898.movie".to_string()),
                ..Default::default()
            },
            action: action::Action {
                name: "play".to_string(),
                ..Default::default()
            },
            constraints: vec![Constraint {
                left_operand: LeftOperand::Literal("spatial".to_string()),
                operator: Operator::Equal,
                right_operand: RightOperand::IRI("https://www.wikidata.org/wiki/Q183".to_string()),
                ..Default::default()
            }],
            ..Default::default()
        };

        let first_policy_second_permission = rule::Permission {
            target: asset::Asset {
                uid: Option::from("https://example.com/document:1234".to_string()),
                ..Default::default()
            },
            action: action::Action {
                name: "print".to_string(),
                refinements: Some(Refinements::Constraints(vec![Constraint {
                    left_operand: LeftOperand::Literal("resolution".to_string()),
                    operator: Operator::LessThanOrEqual,
                    right_operand: RightOperand::Literal("1200".to_string()),
                    unit: Option::from("https://dbpedia.org/resource/Dots_per_inch".to_string()),
                    ..Default::default()
                }])),
                ..Default::default()
            },
            assigner: Some(party::Party {
                uid: Option::from("https://example.com/org:616".to_string()),
                function: Assigner,
                ..Default::default()
            }),
            ..Default::default()
        };

        let first_policy_third_permission = rule::Permission {
            target: asset::Asset {
                uid: Option::from("https://example.com/book/1999".to_string()),
                ..Default::default()
            },
            action: action::Action {
                name: "reproduce".to_string(),
                refinements: Some(Refinements::LogicalConstraints(vec![LogicalConstraint {
                    operand: Some((
                        LogicalOperator::Xone,
                        vec![
                            "https://example.com/p:88/C1".to_string(),
                            "https://example.com/p:88/C2".to_string(),
                        ],
                    )),
                    ..Default::default()
                }])),
                ..Default::default()
            },
            assigner: Some(party::Party {
                uid: Option::from("https://example.com/org/paisley-park".to_string()),
                function: Assigner,
                ..Default::default()
            }),
            ..Default::default()
        };

        let first_policy_first_prohibition = rule::Rule::Prohibition(rule::Prohibition {
            target: asset::Asset {
                uid: Option::from("https://example.com/photoAlbum:55".to_string()),
                ..Default::default()
            },
            action: action::Action {
                name: "archive".to_string(),
                ..Default::default()
            },
            assigner: Some(party::Party {
                uid: Option::from("https://example.com/MyPix:55".to_string()),
                function: Assigner,
                ..Default::default()
            }),
            assignee: Some(party::Party {
                uid: Option::from("https://example.com/assignee:55".to_string()),
                function: Assignee,
                ..Default::default()
            }),
            ..Default::default()
        });

        let first_policy = policy::Policy::SetPolicy(policy::SetPolicy {
            uid: "https://example.com/policy:1010".to_string(),
            rules: vec![
                rule::Rule::Permission(first_policy_first_permission),
                rule::Rule::Permission(first_policy_second_permission),
                rule::Rule::Permission(first_policy_third_permission),
                first_policy_first_prohibition,
            ],
            ..Default::default()
        });

        policies.push(first_policy);

        // Second Policy
        let sec_policy_asset = asset::Asset {
            uid: Option::from("https://example.com/asset:9898.movie".to_string()),
            ..Default::default()
        };

        let sec_policy_action = action::Action {
            name: "use".to_string(),
            ..Default::default()
        };

        let sec_policy_rule = rule::Rule::Permission(rule::Permission {
            target: sec_policy_asset,
            action: sec_policy_action,
            ..Default::default()
        });

        let sec_policy = policy::Policy::SetPolicy(policy::SetPolicy {
            uid: "https://example.com/policy:1010".to_string(),
            rules: vec![sec_policy_rule],
            ..Default::default()
        });

        policies.push(sec_policy);

        assert_eq!(parsed_data.parsed_policies, policies);

        // First Constraint
        let mut comp_constraints: Vec<Constraint> = Vec::new();

        let first_left_op = LeftOperand::Literal("media".to_string());
        let first_op = Operator::Equal;
        let first_right_op = RightOperand::Literal("online".to_string());

        let comp_first_constraint = Constraint {
            uid: Option::from("https://example.com/p:88/C1".to_string()),
            left_operand: first_left_op,
            operator: first_op,
            right_operand: first_right_op,
            ..Default::default()
        };

        comp_constraints.push(comp_first_constraint);

        // Second Constraint
        let second_left_op = LeftOperand::Literal("media".to_string());
        let second_op = Operator::Equal;
        let second_right_op = RightOperand::Literal("print".to_string());

        let comp_second_constraint = Constraint {
            uid: Option::from("https://example.com/p:88/C2".to_string()),
            left_operand: second_left_op,
            operator: second_op,
            right_operand: second_right_op,
            ..Default::default()
        };

        comp_constraints.push(comp_second_constraint);

        assert_eq!(parsed_data.parsed_constraints, comp_constraints);
    }
}
