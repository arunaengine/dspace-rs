use regex::Regex;
use serde_with::{formats::PreferMany, serde_as, OneOrMany};
use thiserror::Error;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Policy {
    #[serde(flatten)]
    pub policy_type: PolicyType,
}

impl Policy {
    pub fn new(policy_type: PolicyType) -> Policy {
        Policy { policy_type }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum PolicyType {
    MessageOffer(MessageOffer),
    Offer(Offer),
    Agreement(Agreement),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Target {
    #[serde(rename = "@id")]
    pub id: String,
}

impl Target {
    pub fn new(id: String) -> Target {
        Target { id }
    }
}

#[serde_as]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PolicyClass {
    #[serde(flatten)]
    pub abstract_policy_rule: AbstractPolicyRule,
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "odrl:profile", skip_serializing_if = "Vec::is_empty")]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    pub profile: Vec<Reference>,
    #[serde(rename = "odrl:permission", skip_serializing_if = "Vec::is_empty")]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    pub permission: Vec<Permission>,
    #[serde(rename = "odrl:obligation", skip_serializing_if = "Vec::is_empty")]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    pub obligation: Vec<Duty>,
    #[serde(rename = "odrl:target")]
    pub target: Target,
}

impl PolicyClass {
    pub fn new(
        abstract_policy_rule: AbstractPolicyRule,
        id: String,
        profile: Vec<Reference>,
        permission: Vec<Permission>,
        obligation: Vec<Duty>,
        target: Target,
    ) -> PolicyClass {
        PolicyClass {
            abstract_policy_rule,
            id,
            profile,
            permission,
            obligation,
            target,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AbstractPolicyRule {
    #[serde(rename = "odrl:assigner", skip_serializing_if = "Option::is_none")]
    pub assigner: Option<String>,
    #[serde(rename = "odrl:assignee", skip_serializing_if = "Option::is_none")]
    pub assignee: Option<String>,
}

impl AbstractPolicyRule {
    pub fn new(assigner: Option<String>, assignee: Option<String>) -> AbstractPolicyRule {
        AbstractPolicyRule { assigner, assignee }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageOffer {
    #[serde(flatten)]
    pub policy_class: PolicyClass,
    #[serde(rename = "@type")]
    pub odrl_type: String,
}

impl MessageOffer {
    pub fn new(policy_class: PolicyClass, odrl_type: String) -> MessageOffer {
        MessageOffer {
            policy_class,
            odrl_type,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Offer {
    #[serde(flatten)]
    pub message_offer: MessageOffer,
}

impl Offer {
    pub fn new(policy_class: PolicyClass, odrl_type: String) -> Offer {
        Offer {
            message_offer: MessageOffer::new(policy_class, odrl_type),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Agreement {
    #[serde(flatten)]
    pub policy_class: PolicyClass,
    #[serde(rename = "@type")]
    pub odrl_type: String,
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "odrl:target")]
    pub target: String,
    #[serde(
        rename = "dspace:timestamp",
        skip_serializing_if = "Option::is_none",
        with = "timestamp_format"
    )]
    pub timestamp: Option<String>,
}

impl Agreement {
    pub fn new(
        policy_class: PolicyClass,
        odrl_type: String,
        id: String,
        target: String,
        timestamp: Option<String>,
    ) -> Result<Agreement, TimestampError> {
        // Validate the timestamp format if it is present
        if let Some(ref ts) = timestamp {
            let timestamp_pattern = r"-?([1-9][0-9]{3,}|0[0-9]{3})-(0[1-9]|1[0-2])-(0[1-9]|[12][0-9]|3[01])T(([01][0-9]|2[0-3]):[0-5][0-9]:[0-5][0-9](\\.[0-9]+)?|(24:00:00(\\.0+)?))(Z|(\\+|-)((0[0-9]|1[0-3]):[0-5][0-9]|14:00))?";

            let re = Regex::new(timestamp_pattern).unwrap();
            if !re.is_match(ts) {
                return Err(TimestampError::InvalidFormat);
            }
        }

        Ok(Agreement {
            policy_class,
            odrl_type,
            id,
            target,
            timestamp,
        })
    }
}

#[serde_as]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RuleClass {
    #[serde(flatten)]
    pub abstract_policy_rule: AbstractPolicyRule,
    #[serde(rename = "odrl:action")]
    pub action: Action,
    #[serde(rename = "odrl:constraint", skip_serializing_if = "Vec::is_empty")]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    pub constraint: Vec<Constraint>,
}

impl RuleClass {
    pub fn new(
        abstract_policy_rule: AbstractPolicyRule,
        action: Action,
        constraint: Vec<Constraint>,
    ) -> RuleClass {
        RuleClass {
            abstract_policy_rule,
            action,
            constraint,
        }
    }
}

#[serde_as]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Permission {
    #[serde(flatten)]
    pub abstract_policy_rule: AbstractPolicyRule,
    #[serde(rename = "odrl:action")]
    pub action: Action,
    #[serde(rename = "odrl:constraint", skip_serializing_if = "Vec::is_empty")]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    pub constraint: Vec<Constraint>,
    #[serde(rename = "odrl:duty", skip_serializing_if = "Option::is_none")]
    pub duty: Option<Duty>,
}

impl Permission {
    pub fn new(
        abstract_policy_rule: AbstractPolicyRule,
        action: Action,
        constraint: Vec<Constraint>,
        duty: Option<Duty>,
    ) -> Permission {
        Permission {
            abstract_policy_rule,
            action,
            constraint,
            duty,
        }
    }
}

#[serde_as]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Duty {
    #[serde(flatten)]
    pub abstract_policy_rule: AbstractPolicyRule,
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "odrl:action")]
    pub action: Action,
    #[serde(rename = "odrl:constraint", skip_serializing_if = "Vec::is_empty")]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    pub constraint: Vec<Constraint>,
}

impl Duty {
    pub fn new(
        abstract_policy_rule: AbstractPolicyRule,
        id: Option<String>,
        action: Action,
        constraint: Vec<Constraint>,
    ) -> Duty {
        Duty {
            abstract_policy_rule,
            id,
            action,
            constraint,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Action {
    #[serde(rename = "odrl:delete")]
    Delete,
    #[serde(rename = "odrl:execute")]
    Execute,
    #[serde(rename = "cc:SourceCode")]
    SourceCode,
    #[serde(rename = "odrl:anonymize")]
    Anonymize,
    #[serde(rename = "odrl:extract")]
    Extract,
    #[serde(rename = "odrl:read")]
    Read,
    #[serde(rename = "odrl:index")]
    Index,
    #[serde(rename = "odrl:compensate")]
    Compensate,
    #[serde(rename = "odrl:sell")]
    Sell,
    #[serde(rename = "odrl:derive")]
    Derive,
    #[serde(rename = "odrl:ensureExclusivity")]
    EnsureExclusivity,
    #[serde(rename = "odrl:annotate")]
    Annotate,
    #[serde(rename = "cc:Reproduction")]
    Reproduction,
    #[serde(rename = "odrl:translate")]
    Translate,
    #[serde(rename = "odrl:include")]
    Include,
    #[serde(rename = "cc:DerivativeWorks")]
    DerivativeWorks,
    #[serde(rename = "cc:Distribution")]
    Distribution,
    #[serde(rename = "odrl:textToSpeech")]
    TextToSpeech,
    #[serde(rename = "odrl:inform")]
    Inform,
    #[serde(rename = "odrl:grantUse")]
    GrantUse,
    #[serde(rename = "odrl:archive")]
    Archive,
    #[serde(rename = "odrl:modify")]
    Modify,
    #[serde(rename = "odrl:aggregate")]
    Aggregate,
    #[serde(rename = "odrl:attribute")]
    Attribute,
    #[serde(rename = "odrl:nextPolicy")]
    NextPolicy,
    #[serde(rename = "odrl:digitize")]
    Digitize,
    #[serde(rename = "cc:Attribution")]
    Attribution,
    #[serde(rename = "odrl:install")]
    Install,
    #[serde(rename = "odrl:concurrentUse")]
    ConcurrentUse,
    #[serde(rename = "odrl:distribute")]
    Distribute,
    #[serde(rename = "odrl:synchronize")]
    Synchronize,
    #[serde(rename = "odrl:move")]
    Move,
    #[serde(rename = "odrl:obtainConsent")]
    ObtainConsent,
    #[serde(rename = "odrl:print")]
    Print,
    #[serde(rename = "cc:Notice")]
    Notice,
    #[serde(rename = "odrl:give")]
    Give,
    #[serde(rename = "odrl:uninstall")]
    Uninstall,
    #[serde(rename = "cc:Sharing")]
    Sharing,
    #[serde(rename = "odrl:reviewPolicy")]
    ReviewPolicy,
    #[serde(rename = "odrl:watermark")]
    Watermark,
    #[serde(rename = "odrl:play")]
    Play,
    #[serde(rename = "odrl:reproduce")]
    Reproduce,
    #[serde(rename = "odrl:transform")]
    Transform,
    #[serde(rename = "odrl:display")]
    Display,
    #[serde(rename = "odrl:stream")]
    Stream,
    #[serde(rename = "cc:ShareAlike")]
    ShareAlike,
    #[serde(rename = "odrl:acceptTracking")]
    AcceptTracking,
    #[serde(rename = "cc:CommericalUse")]
    CommercialUse,
    #[serde(rename = "odrl:present")]
    Present,
    #[serde(rename = "odrl:use")]
    Use,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Constraint {
    #[serde(rename = "odrl:rightOperand", skip_serializing_if = "Option::is_none")]
    pub right_operand: Option<RightOperand>,
    #[serde(
        rename = "odrl:rightOperandReference",
        skip_serializing_if = "Option::is_none"
    )]
    pub right_operand_reference: Option<Reference>,
    #[serde(rename = "odrl:leftOperand")]
    pub left_operand: LeftOperand,
    #[serde(rename = "odrl:operator")]
    pub operator: Operator,
}

impl Constraint {
    pub fn new(
        right_operand: Option<RightOperand>,
        right_operand_reference: Option<Reference>,
        left_operand: LeftOperand,
        operator: Operator,
    ) -> Constraint {
        Constraint {
            right_operand,
            right_operand_reference,
            left_operand,
            operator,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Operator {
    #[serde(rename = "odrl:eq")]
    Eq,
    #[serde(rename = "odrl:gt")]
    Gt,
    #[serde(rename = "odrl:gteq")]
    Gteq,
    #[serde(rename = "odrl:hasPart")]
    HasPart,
    #[serde(rename = "odrl:isA")]
    IsA,
    #[serde(rename = "odrl:isAllOf")]
    IsAllOf,
    #[serde(rename = "odrl:isAnyOf")]
    IsAnyOf,
    #[serde(rename = "odrl:isNoneOf")]
    IsNoneOf,
    #[serde(rename = "odrl:isPartOf")]
    IsPartOf,
    #[serde(rename = "odrl:lt")]
    Lt,
    #[serde(rename = "odrl:term-lteq")]
    Lteq,
    #[serde(rename = "odrl:neq")]
    Neq,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum RightOperand {
    String(String),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum LeftOperand {
    #[serde(rename = "odrl:absolutePosition")]
    AbsolutePosition,
    #[serde(rename = "odrl:absoluteSize")]
    AbsoluteSize,
    #[serde(rename = "odrl:absoluteSpatialPosition")]
    AbsoluteSpatialPosition,
    #[serde(rename = "odrl:absoluteTemporalPosition")]
    AbsoluteTemporalPosition,
    #[serde(rename = "odrl:count")]
    Count,
    #[serde(rename = "odrl:dateTime")]
    DateTime,
    #[serde(rename = "odrl:delayPeriod")]
    DelayPeriod,
    #[serde(rename = "odrl:deliveryChannel")]
    DeliveryChannel,
    #[serde(rename = "odrl:device")]
    Device,
    #[serde(rename = "odrl:elapsedTime")]
    ElapsedTime,
    #[serde(rename = "odrl:event")]
    Event,
    #[serde(rename = "odrl:fileFormat")]
    FileFormat,
    #[serde(rename = "odrl:industry")]
    Industry,
    #[serde(rename = "odrl:language")]
    Language,
    #[serde(rename = "odrl:media")]
    Media,
    #[serde(rename = "odrl:meteredTime")]
    MeteredTime,
    #[serde(rename = "odrl:payAmount")]
    PayAmount,
    #[serde(rename = "odrl:percentage")]
    Percentage,
    #[serde(rename = "odrl:product")]
    Product,
    #[serde(rename = "odrl:purpose")]
    Purpose,
    #[serde(rename = "odrl:recipient")]
    Recipient,
    #[serde(rename = "odrl:relativePosition")]
    RelativePosition,
    #[serde(rename = "odrl:relativeSize")]
    RelativeSize,
    #[serde(rename = "odrl:relativeSpatialPosition")]
    RelativeSpatialPosition,
    #[serde(rename = "odrl:relativeTemporalPosition")]
    RelativeTemporalPosition,
    #[serde(rename = "odrl:resolution")]
    Resolution,
    #[serde(rename = "odrl:spatial")]
    Spatial,
    #[serde(rename = "odrl:spatialCoordinates")]
    SpatialCoordinates,
    #[serde(rename = "odrl:system")]
    System,
    #[serde(rename = "odrl:systemDevice")]
    SystemDevice,
    #[serde(rename = "odrl:timeInterval")]
    TimeInterval,
    #[serde(rename = "odrl:unitOfCount")]
    UnitOfCount,
    #[serde(rename = "odrl:version")]
    Version,
    #[serde(rename = "odrl:virtualLocation")]
    VirtualLocation,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Reference {
    #[serde(rename = "@id")]
    pub id: String,
}

impl Reference {
    pub fn new(id: String) -> Reference {
        Reference { id }
    }
}

#[derive(Debug, Error)]
pub enum TimestampError {
    #[error("Invalid timestamp format")]
    InvalidFormat,
}

// Validation of timestamp format while deserializing and serializing
mod timestamp_format {
    use super::*;
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(timestamp: &Option<String>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match timestamp {
            Some(ref ts) => serializer.serialize_str(ts),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let timestamp: Option<String> = Option::deserialize(deserializer)?;
        if let Some(ref ts) = timestamp {
            Agreement::new(
                PolicyClass {
                    abstract_policy_rule: AbstractPolicyRule {
                        assigner: None,
                        assignee: None,
                    },
                    id: String::new(),
                    profile: vec![],
                    permission: vec![],
                    obligation: vec![],
                    target: Target::new(String::new()),
                },
                String::new(),
                String::new(),
                String::new(),
                Some(ts.clone()),
            )
            .map_err(serde::de::Error::custom)?;
        }
        Ok(timestamp)
    }
}
