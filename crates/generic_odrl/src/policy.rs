use crate::{
    generic_rule::{Obligation, Permission, Prohibition, Rule},
    generics::StringOrX,
};
use odrl::model::action::Action;
use odrl::model::asset::Asset;
use odrl::model::conflict_term::ConflictTerm;
use odrl::model::party::Party;
use odrl::model::type_alias::IRI;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// A Policy MAY include an obligation to fulfil a Duty. The obligation is fulfilled if all constraints are satisfied and if its action, with all refinements satisfied, has been exercised.

// Validate required fields depending on provided type

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GenericPolicy {
    #[serde(rename = "@context", skip_serializing_if = "Option::is_none")]
    pub context: Option<StringOrX<HashMap<String, serde_json::Value>>>,
    #[serde(rename = "uid", skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
    #[serde(rename = "@type")]
    pub policy_type: String,
    #[serde(rename = "assigner", skip_serializing_if = "Option::is_none")]
    pub assigner: Option<StringOrX<Party>>,
    #[serde(rename = "assignee", skip_serializing_if = "Option::is_none")]
    pub assignee: Option<StringOrX<Party>>,
    #[serde(rename = "target", skip_serializing_if = "Option::is_none")]
    pub target: Option<StringOrX<Box<Asset>>>,
    #[serde(rename = "action", skip_serializing_if = "Option::is_none")]
    pub action: Option<StringOrX<Action>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission: Option<Vec<Permission>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prohibition: Option<Vec<Prohibition>>,
    #[serde(rename = "profile", skip_serializing_if = "Option::is_none")]
    pub profiles: Option<StringOrX<Vec<IRI>>>,
    #[serde(rename = "inheritFrom", skip_serializing_if = "Option::is_none")]
    pub inherit_from: Option<StringOrX<Vec<IRI>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflict: Option<ConflictTerm>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obligation: Option<Vec<Obligation>>,
}

impl GenericPolicy {
    pub fn new(
        context: Option<StringOrX<HashMap<String, serde_json::Value>>>,
        uid: Option<String>,
        policy_type: String,
        assigner: Option<StringOrX<Party>>,
        assignee: Option<StringOrX<Party>>,
        target: Option<StringOrX<Box<Asset>>>,
        action: Option<StringOrX<Action>>,
        permission: Option<Vec<Permission>>,
        prohibition: Option<Vec<Prohibition>>,
        profiles: Option<StringOrX<Vec<IRI>>>,
        inherit_from: Option<StringOrX<Vec<IRI>>>,
        conflict: Option<ConflictTerm>,
        obligation: Option<Vec<Obligation>>,
    ) -> Self {
        GenericPolicy {
            context,
            uid,
            policy_type,
            assigner,
            assignee,
            target,
            action,
            permission,
            prohibition,
            profiles,
            inherit_from,
            conflict,
            obligation,
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    #[test]
    fn deserialize_example1() {
        let example1 = r#"
            {
                "@context": "http://www.w3.org/ns/odrl.jsonld",
                "@type": "Set",
                "uid": "http://example.com/policy:1010",
                "permission": [{
                    "target": "http://example.com/asset:9898.movie",
                    "action": "use"
                }]
            }
        "#;
        serde_json::from_str::<super::GenericPolicy>(example1).unwrap();
    }
    #[test]
    fn deserialize_example2() {
        let example2 = r#"
            {
                "@context": "http://www.w3.org/ns/odrl.jsonld",
                "@type": "Offer",
                "uid": "http://example.com/policy:1011",
                "profile": "http://example.com/odrl:profile:01",
                "permission": [{
                    "target": "http://example.com/asset:9898.movie",
                    "assigner": "http://example.com/party:org:abc",
                    "action": "play"
                }]
            }
        "#;
        serde_json::from_str::<super::GenericPolicy>(example2).unwrap();
    }

    #[test]
    fn deserialize_example3() {
        let example = r#"
            {
                "@context": "http://www.w3.org/ns/odrl.jsonld",
                "@type": "Agreement",
                "uid": "http://example.com/policy:1012",
                "profile": "http://example.com/odrl:profile:01",
                "permission": [{
                    "target": "http://example.com/asset:9898.movie",
                    "assigner": "http://example.com/party:org:abc",
                    "assignee": "http://example.com/party:person:billie",
                    "action": "play"
                }]
            }
        "#;
        serde_json::from_str::<super::GenericPolicy>(example).unwrap();
    }

    #[test]
    fn deserialize_example4() {
        let example = r#"
            {
               "@context": "http://www.w3.org/ns/odrl.jsonld",
               "@type": "Offer",
               "uid": "http://example.com/policy:3333",
               "profile": "http://example.com/odrl:profile:02",
               "permission": [{
                   "target": "http://example.com/asset:3333",
                   "action": "display",
                   "assigner": "http://example.com/party:0001"
               }]
            }
        "#;
        serde_json::from_str::<super::GenericPolicy>(example).unwrap();
    }

    #[test]
    fn deserialize_example5() {
        let example = r#"
            {
                "@context": "http://www.w3.org/ns/odrl.jsonld",
                "@type": "Policy",
                "uid": "http://example.com/policy:1011",
                "profile": "http://example.com/odrl:profile:03",
                "permission": [{
                    "target": {
                       "@type": "AssetCollection",
                       "uid":  "http://example.com/archive1011" },
                    "action": "index",
                    "summary": "http://example.com/x/database"
                }]
            }
        "#;
        serde_json::from_str::<super::GenericPolicy>(example).unwrap();
    }

    #[test]
    fn deserialize_example6() {
        let example = r#"
            {
               "@type": "dc:Document",
               "@id": "http://example.com/asset:111.doc",
               "dc:title": "Annual Report",
               "odrl:partOf": "http://example.com/archive1011"
            }
        "#;
        serde_json::from_str::<super::GenericPolicy>(example).unwrap();
    }

    #[test]
    fn deserialize_example7() {
        let example = r#"
            {
               "@type": "dc:MovingImage",
               "@id": "http://example.com/asset:9999.movie",
               "dc:publisher": "ABC Pictures",
               "dc:creator": "Allen, Woody",
               "dc:issued": "2017",
               "dc:subject": "Musical Comedy",
               "odrl:hasPolicy": "http://example.com/policy:1010"
            }
        "#;
        serde_json::from_str::<super::GenericPolicy>(example).unwrap();
    }

    #[test]
    fn deserialize_example8() {
        let example = r#"
            {
                "@context": "http://www.w3.org/ns/odrl.jsonld",
                "@type": "Agreement",
                "uid": "http://example.com/policy:8888",
                "profile": "http://example.com/odrl:profile:04",
                "permission": [{
                    "target": "http://example.com/music/1999.mp3",
                    "assigner": "http://example.com/org/sony-music",
                    "assignee": "http://example.com/people/billie",
                    "action": "play"
                }]
            }
        "#;
        serde_json::from_str::<super::GenericPolicy>(example).unwrap();
    }

    #[test]
    fn deserialize_example9() {
        let example = r#"
            {
                "@context": [
                    "http://www.w3.org/ns/odrl.jsonld",
                    { "vcard": "http://www.w3.org/2006/vcard/ns#" }
                ],
                "@type": "Agreement",
                "uid": "http://example.com/policy:777",
                "profile": "http://example.com/odrl:profile:05",
                "permission": [{
                    "target": "http://example.com/looking-glass.ebook",
                    "assigner": {
                        "@type": [ "Party", "vcard:Organization" ],
                        "uid":  "http://example.com/org/sony-books",
                        "vcard:fn": "Sony Books LCC",
                        "vcard:hasEmail": "sony-contact@example.com" },
                    "assignee": {
                        "@type": [ "PartyCollection", "vcard:Group" ],
                        "uid":  "http://example.com/team/A",
                        "vcard:fn": "Team A",
                        "vcard:hasEmail": "teamA@example.com"},
                    "action": "use"
                }]
            }
        "#;
        //serde_json::from_str::<super::GenericPolicy>(example).unwrap();
        //TODO: Line 6 - "data did not match any variant of untagged enum StringOrX"
    }

    #[test]
    fn deserialize_example10() {
        let example = r#"
            {
               "@type": "vcard:Individual",
               "@id": "http://example.com/person/murphy",
               "vcard:fn": "Murphy",
               "vcard:hasEmail": "murphy@example.com",
               "odrl:partOf": "http://example.com/team/A"
            }
        "#;
        serde_json::from_str::<super::GenericPolicy>(example).unwrap();
    }

    #[test]
    fn deserialize_example11() {
        let example = r#"
            {
               "@type": "vcard:Individual",
               "@id": "http://example.com/person/billie",
               "vcard:fn": "Billie",
               "vcard:hasEmail": "billie@example.com",
               "odrl:assigneeOf": "http://example.com/policy:1011"
            }
        "#;
        serde_json::from_str::<super::GenericPolicy>(example).unwrap();
    }

    #[test]
    fn deserialize_example12() {
        let example = r#"
            {
                "@context": "http://www.w3.org/ns/odrl.jsonld",
                "@type": "Offer",
                "uid": "http://example.com/policy:1012",
                "profile": "http://example.com/odrl:profile:06",
                "permission": [{
                        "target": "http://example.com/music:1012",
                        "assigner": "http://example.com/org:abc",
                        "action": "play"
                 }]
            }
        "#;
        serde_json::from_str::<super::GenericPolicy>(example).unwrap();
    }

    #[test]
    fn deserialize_example13() {
        let example = r#"
            {
                "@context": "http://www.w3.org/ns/odrl.jsonld",
                "@type": "Offer",
                "uid": "http://example.com/policy:6163",
                "profile": "http://example.com/odrl:profile:10",
                "permission": [{
                   "target": "http://example.com/document:1234",
                   "assigner": "http://example.com/org:616",
                   "action": "distribute",
                   "constraint": [{
                       "leftOperand": "dateTime",
                       "operator": "lt",
                       "rightOperand":  { "@value": "2018-01-01", "@type": "xsd:date" }
                   }]
               }]
            }
        "#;
        //serde_json::from_str::<super::GenericPolicy>(example).unwrap();
        //TODO Check dateTime Constraint
    }

    #[test]
    fn deserialize_example14() {
        let example = r#"
            {
                "@context": "http://www.w3.org/ns/odrl.jsonld",
                "@type": "Offer",
                "uid": "http://example.com/policy:6161",
                "profile": "http://example.com/odrl:profile:10",
                "permission": [{
                   "target": "http://example.com/document:1234",
                   "assigner": "http://example.com/org:616",
                   "action": [{
                      "rdf:value": { "@id": "odrl:print" },
                      "refinement": [{
                         "leftOperand": "resolution",
                         "operator": "lteq",
                         "rightOperand": { "@value": "1200", "@type": "xsd:integer" },
                         "unit": "http://dbpedia.org/resource/Dots_per_inch"
                      }]
                  }]
               }]
            }
        "#;
        //serde_json::from_str::<super::GenericPolicy>(example).unwrap();
        //TODO Line 19???
    }

    #[test]
    fn deserialize_example15() {
        let example = r#"
            {
                "@context": "http://www.w3.org/ns/odrl.jsonld",
                "@type": "Offer",
                "uid": "http://example.com/policy:88",
                "profile": "http://example.com/odrl:profile:10",
                "permission": [{
                    "target": "http://example.com/book/1999",
                    "assigner": "http://example.com/org/paisley-park",
                    "action": [{
                       "rdf:value": { "@id": "odrl:reproduce" },
                       "refinement": {
                           "xone": {
                               "@list": [
                                    { "@id": "http://example.com/p:88/C1" },
                                    { "@id": "http://example.com/p:88/C2" }
                               ]
                           }
                       }
                    }]
                }]
            }

            {
               "@context": "http://www.w3.org/ns/odrl.jsonld",
               "@type": "Constraint",
               "uid": "http://example.com/p:88/C1",
               "leftOperand": "media",
               "operator": "eq",
               "rightOperand": { "@value": "online", "@type": "xsd:string" }
            }

            {
               "@context": "http://www.w3.org/ns/odrl.jsonld",
               "@type": "Constraint",
               "uid": "http://example.com/p:88/C2",
               "leftOperand": "media",
               "operator": "eq",
               "rightOperand": { "@value": "print", "@type": "xsd:string" }
            }
        "#;
        //serde_json::from_str::<super::GenericPolicy>(example).unwrap();
        //TODO Line 21???
    }

    #[test]
    fn deserialize_example16() {
        let example = r#"
            {
              "@context": "http://www.w3.org/ns/odrl.jsonld",
              "@type": "Offer",
              "uid": "http://example.com/policy:4444",
              "profile": "http://example.com/odrl:profile:11",
              "permission": [{
                "assigner": "http://example.com/org88",
                "target": {
                  "@type": "AssetCollection",
                  "source":  "http://example.com/media-catalogue",
                  "refinement": [{
                    "leftOperand": "runningTime",
                    "operator": "lt",
                    "rightOperand": { "@value": "60", "@type": "xsd:integer" },
                    "unit": "http://qudt.org/vocab/unit/MinuteTime"
                  }]
                },
                "action": "play"
              }]
            }
        "#;
        //serde_json::from_str::<super::GenericPolicy>(example).unwrap();
        //TODO Line 18
    }

    #[test]
    fn deserialize_example17() {
        let example = r#"
            {
              "@context": "http://www.w3.org/ns/odrl.jsonld",
              "@type": "Agreement",
              "uid": "http://example.com/policy:4444",
              "profile": "http://example.com/odrl:profile:12",
              "permission": [{
                "target": "http://example.com/myPhotos:BdayParty",
                "assigner": "http://example.com/user44",
                "assignee": {
                  "@type": "PartyCollection",
                  "source":  "http://example.com/user44/friends",
                  "refinement": [{
                    "leftOperand": "foaf:age",
                    "operator": "gt",
                    "rightOperand": { "@value": "17", "@type": "xsd:integer" }
                  }]
                },
                "action": { "@id": "ex:view" }
              }]
            }
        "#;
        //serde_json::from_str::<super::GenericPolicy>(example).unwrap();
        //TODO Line 18
    }

    #[test]
    fn deserialize_example18() {
        let example = r#"
            {
               "@context": "http://www.w3.org/ns/odrl.jsonld",
               "@type": "Offer",
               "uid": "http://example.com/policy:9090",
               "profile": "http://example.com/odrl:profile:07",
               "permission": [{
                   "target": "http://example.com/game:9090",
                   "assigner": "http://example.com/org:xyz",
                   "action": "play",
                   "constraint": [{
                       "leftOperand": "dateTime",
                       "operator": "lteq",
                       "rightOperand": { "@value": "2017-12-31", "@type": "xsd:date" }
                   }]
               }]
            }
        "#;
        //serde_json::from_str::<super::GenericPolicy>(example).unwrap();
        //TODO Check dateTime
    }

    #[test]
    fn deserialize_example19() {
        let example = r#"
            {
                "@context": "http://www.w3.org/ns/odrl.jsonld",
                "@type": "Agreement",
                "uid": "http://example.com/policy:5555",
                "profile": "http://example.com/odrl:profile:08",
                "conflict": "perm",
                "permission": [{
                    "target": "http://example.com/photoAlbum:55",
                    "action": "display",
                    "assigner": "http://example.com/MyPix:55",
                    "assignee": "http://example.com/assignee:55"
                }],
                "prohibition": [{
                    "target": "http://example.com/photoAlbum:55",
                    "action": "archive",
                    "assigner": "http://example.com/MyPix:55",
                    "assignee": "http://example.com/assignee:55"
                }]
            }
        "#;
        serde_json::from_str::<super::GenericPolicy>(example).unwrap();
    }

    #[test]
    fn deserialize_example20() {
        let example = r#"
            {
              "@context": "http://www.w3.org/ns/odrl.jsonld",
              "@type": "Agreement",
              "uid": "http://example.com/policy:42",
              "profile": "http://example.com/odrl:profile:09",
              "obligation": [{
                  "assigner": "http://example.com/org:43",
                  "assignee": "http://example.com/person:44",
                  "action": [{
                      "rdf:value": {
                        "@id": "odrl:compensate"
                      },
                      "refinement": [
                        {
                          "leftOperand": "payAmount",
                          "operator": "eq",
                          "rightOperand": { "@value": "500.00", "@type": "xsd:decimal" },
                          "unit": "http://dbpedia.org/resource/Euro"
                        }]
                    }]
                }]
            }
        "#;
        //serde_json::from_str::<super::GenericPolicy>(example).unwrap();
        //TODO Line 22
    }

    #[test]
    fn deserialize_example21() {
        let example = r#"
            {
                "@context": "http://www.w3.org/ns/odrl.jsonld",
                "@type": "Agreement",
                "uid": "http://example.com/policy:42B",
                "profile": "http://example.com/odrl:profile:09",
                "assigner": "http://example.com/org:43",
                "assignee": "http://example.com/person:44",
                "obligation": [{
                   "action": "delete",
                   "target": "http://example.com/document:XZY",
                   "consequence": [{
                     "action": [{
                         "rdf:value": { "@id": "odrl:compensate" },
                         "refinement": [{
                            "leftOperand": "payAmount",
                            "operator": "eq",
                            "rightOperand": { "@value": "10.00", "@type": "xsd:decimal" },
                            "unit": "http://dbpedia.org/resource/Euro"
                         }]
                     }],
                     "compensatedParty": "http://wwf.org"
                     }]
                }]
            }
        "#;
        //serde_json::from_str::<super::GenericPolicy>(example).unwrap();
        //TODO Line 21
    }

    #[test]
    fn deserialize_example22() {
        let example = r#"
            {
                "@context": "http://www.w3.org/ns/odrl.jsonld",
                "@type": "Offer",
                "uid": "http://example.com/policy:88",
                "profile": "http://example.com/odrl:profile:09",
                "permission": [{
                    "assigner": "http://example.com/assigner:sony",
                    "target": "http://example.com/music/1999.mp3",
                    "action": "play",
                    "duty": [{
                       "action": [{
                          "rdf:value": { "@id": "odrl:compensate" },
                          "refinement": [{
                             "leftOperand": "payAmount",
                             "operator": "eq",
                             "rightOperand": { "@value": "5.00", "@type": "xsd:decimal" },
                             "unit": "http://dbpedia.org/resource/Euro"
                          }]
                        }],
                        "constraint": [{
                            "leftOperand": "event",
                            "operator": "lt",
                            "rightOperand": { "@id": "odrl:policyUsage" }
                        }]
                    }]
                }]
            }
        "#;
        //serde_json::from_str::<super::GenericPolicy>(example).unwrap();
        //TODO Line 20
    }

    #[test]
    fn deserialize_example23() {
        let example = r#"
            {
                "@context": "http://www.w3.org/ns/odrl.jsonld",
                "@type": "Agreement",
                "uid": "http://example.com/policy:66",
                "profile": "http://example.com/odrl:profile:09",
                "permission": [{
                    "target": "http://example.com/data:77",
                    "assigner": "http://example.com/org:99",
                    "assignee": "http://example.com/person:88",
                    "action": "distribute",
                    "duty": [{
                        "action": "attribute",
                        "attributedParty": "http://australia.gov.au/",
                        "consequence": [{
                           "action": "acceptTracking",
                           "trackingParty": "http://example.com/dept:100"
                        }]
                    }]
                }]
            }
        "#;
        serde_json::from_str::<super::GenericPolicy>(example).unwrap();
    }

    #[test]
    fn deserialize_example24() {
        let example = r#"
            {
                "@context": "http://www.w3.org/ns/odrl.jsonld",
                "@type": "Agreement",
                "uid": "http://example.com/policy:33CC",
                "profile": "http://example.com/odrl:profile:09",
                "prohibition": [{
                    "target": "http://example.com/data:77",
                    "assigner": "http://example.com/person:88",
                    "assignee": "http://example.com/org:99",
                    "action": "index",
                    "remedy": [{
                        "action": "anonymize",
                        "target": "http://example.com/data:77"
                    }]
                }]
            }
        "#;
        serde_json::from_str::<super::GenericPolicy>(example).unwrap();
    }

    #[test]
    fn deserialize_example25() {
        let example = r#"
            {
              "@context": "http://www.w3.org/ns/odrl.jsonld",
              "@type": "Policy",
              "uid": "http://example.com/policy:7777",
              "profile": "http://example.com/odrl:profile:20",
              "permission": [{
                "target": "http://example.com/music/1999.mp3",
                "assigner": "http://example.com/org/sony-music",
                "action": "play"
              }]
            }
        "#;
        serde_json::from_str::<super::GenericPolicy>(example).unwrap();
    }

    #[test]
    fn deserialize_example26() {
        let example = r#"
            {
                "@context": "http://www.w3.org/ns/odrl.jsonld",
                "@type": "Policy",
                "uid": "http://example.com/policy:8888",
                "profile": "http://example.com/odrl:profile:20",
                "permission": [{
                    "target": [ "http://example.com/music/1999.mp3",
                                "http://example.com/music/PurpleRain.mp3" ],
                    "assigner": "http://example.com/org/sony-music",
                    "action": [ "play", "stream" ]
                }]
            }
        "#;
        serde_json::from_str::<super::GenericPolicy>(example).unwrap();
    }

    #[test]
    fn deserialize_example27() {
        let example = r#"
            {
                "@context": "http://www.w3.org/ns/odrl.jsonld",
                "@type": "Policy",
                "uid": "http://example.com/policy:8888",
                "profile": "http://example.com/odrl:profile:20",
                "permission": [{
                    "target": "http://example.com/music/1999.mp3",
                    "assigner": "http://example.com/org/sony-music",
                    "action": "play"
                },
                {
                    "target": "http://example.com/music/1999.mp3",
                    "assigner": "http://example.com/org/sony-music",
                    "action": "stream"
                },
                {
                    "target": "http://example.com/music/PurpleRain.mp3",
                    "assigner": "http://example.com/org/sony-music",
                    "action": "play"
                },
                {
                    "target": "http://example.com/music/PurpleRain.mp3",
                    "assigner": "http://example.com/org/sony-music",
                    "action": "stream"
                }]
            }
        "#;
        serde_json::from_str::<super::GenericPolicy>(example).unwrap();
    }

    #[test]
    fn deserialize_example28() {
        let example = r#"
            {
                "@context": "http://www.w3.org/ns/odrl.jsonld",
                "@type": "Policy",
                "uid": "http://example.com/policy:8888",
                "profile": "http://example.com/odrl:profile:21",
                "target": "http://example.com/music/1999.mp3",
                "assigner": "http://example.com/org/sony-music",
                "action": "play",
                "permission": [{
                    "assignee": "http://example.com/people/billie"
                    },
                    {
                    "assignee": "http://example.com/people/murphy"
                    }]
            }
        "#;
        serde_json::from_str::<super::GenericPolicy>(example).unwrap();
    }

    #[test]
    fn deserialize_example29() {
        let example = r#"
            {
                "@context": "http://www.w3.org/ns/odrl.jsonld",
                "@type": "Policy",
                "uid": "http://example.com/policy:8888",
                "profile": "http://example.com/odrl:profile:21",
                "permission": [{
                    "assignee": "http://example.com/people/billie",
                    "target": "http://example.com/music/1999.mp3",
                    "assigner": "http://example.com/org/sony-music",
                    "action": "play"
                    },
                    {
                    "assignee": "http://example.com/people/murphy",
                    "target": "http://example.com/music/1999.mp3",
                    "assigner": "http://example.com/org/sony-music",
                    "action": "play"
                    }]
            }
        "#;
        serde_json::from_str::<super::GenericPolicy>(example).unwrap();
    }

    #[test]
    fn deserialize_example30() {
        let example = r#"
            {
                "@context": [
                    "http://www.w3.org/ns/odrl.jsonld"
                ],
                "@type": "Policy",
                "uid": "http://example.com/policy:8888",
                "profile": "http://example.com/odrl:profile:22",
                "dc:creator": "Billie Enterprises LLC",
                "dc:description": "This policy covers...",
                "dc:issued": "2017-01-01T12:00",
                "dc:coverage": { "@id": "https://www.iso.org/obp/ui/#iso:code:3166:AU-QLD" },
                "dc:replaces": { "@id": "http://example.com/policy:8887" },
                "permission": [ { } ]
            }
        "#;
        serde_json::from_str::<super::GenericPolicy>(example).unwrap();
        //Deleted dc namespace
    }

    #[test]
    fn deserialize_example31() {
        let example = r#"
            {
                "@context": "http://www.w3.org/ns/odrl.jsonld",
                "@type": "Policy",
                "uid": "http://example.com/policy:default",
                "profile": "http://example.com/odrl:profile:30",
                "assigner": "http://example.com/org-01",
                "obligation": [{
                    "target": "http://example.com/asset:terms-and-conditions",
                    "action": "reviewPolicy"
                }]
            }
        "#;
        serde_json::from_str::<super::GenericPolicy>(example).unwrap();
    }

    #[test]
    fn deserialize_example32() {
        let example = r#"
            {
                "@context": "http://www.w3.org/ns/odrl.jsonld",
                "@type": "Agreement",
                "uid": "http://example.com/policy:4444",
                "profile": "http://example.com/odrl:profile:30",
                "inheritFrom": "http://example.com/policy:default",
                "assignee": "http://example.com/user:0001",
                "permission": [{
                    "target": "http://example.com/asset:5555",
                    "action":  "display"
                }]
            }
        "#;
        serde_json::from_str::<super::GenericPolicy>(example).unwrap();
    }

    #[test]
    fn deserialize_example33() {
        let example = r#"
            {
                "@context": "http://www.w3.org/ns/odrl.jsonld",
                "@type": "Agreement",
                "uid": "http://example.com/policy:4444",
                "profile": "http://example.com/odrl:profile:30",
                "inheritFrom": "http://example.com/policy:default",
                "permission": [{
                    "target": "http://example.com/asset:5555",
                    "action": "display",
                    "assigner": "http://example.com/org-01",
                    "assignee": "http://example.com/user:0001"
                }],
                "obligation": [{
                    "target": "http://example.com/asset:terms-and-conditions",
                    "action": "reviewPolicy",
                    "assigner": "http://example.com/org-01",
                    "assignee": "http://example.com/user:0001"
                }]
            }
        "#;
        serde_json::from_str::<super::GenericPolicy>(example).unwrap();
    }

    #[test]
    fn deserialize_example34() {
        let example = r#"
            {
                "@context": "http://www.w3.org/ns/odrl.jsonld",
                "@type": "Policy",
                "uid": "http://example.com/policy:0001",
                "profile": "http://example.com/odrl:profile:40",
                "conflict": "perm",
                "permission": [{
                    "target": "http://example.com/asset:1212",
                    "action": "use",
                    "assigner": "http://example.com/owner:181"
                }]
            }
        "#;
        serde_json::from_str::<super::GenericPolicy>(example).unwrap();
    }

    #[test]
    fn deserialize_example35() {
        let example = r#"
            {
                "@context": "http://www.w3.org/ns/odrl.jsonld",
                "@type": "Policy",
                "uid": "http://example.com/policy:0002",
                "profile": "http://example.com/odrl:profile:40",
                "conflict": "perm",
                "permission": [{
                    "target": "http://example.com/asset:1212",
                    "action": "display",
                    "assigner": "http://example.com/owner:182"
                }],
                "prohibition": [{
                    "target": "http://example.com/asset:1212",
                    "action": "print"
                }]
            }
        "#;
        serde_json::from_str::<super::GenericPolicy>(example).unwrap();
    }

}
