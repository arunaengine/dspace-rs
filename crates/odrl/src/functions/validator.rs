use serde_json::Value;

use crate::name_spaces;

pub struct Validator {}

fn validate_uid(v: &Value) -> bool {
    println!("Validating uid: {:?}", v);
    if v.is_string() {
        // Check if the uid is a valid URI
        if !v.as_str().unwrap().starts_with("http") && !v.as_str().unwrap().starts_with("https://")
        {
            println!("Invalid uid: {}", v.as_str().unwrap());
            return false;
        }
    } else {
        println!("Invalid uid: {}", v.as_str().unwrap());
        return false;
    }
    true
}

fn validate_context(v: &Value) -> bool {
    let context = v;
    println!("Validating context: {:?}", context);
    let name_spaces = vec![
        name_spaces::RDF_NS,
        name_spaces::RDFS_NS,
        name_spaces::OWL_NS,
        name_spaces::XSD_NS,
        name_spaces::SKOS_NS,
        name_spaces::DCTERMS_NS,
        name_spaces::VCARD_NS,
        name_spaces::FOAF_NS,
        name_spaces::SCHEMA_NS,
        name_spaces::CC_NS,
        name_spaces::LD_NS,
        name_spaces::ODRL_NS,
        name_spaces::EDC_NS,
    ];
    if context.is_object() {
        if context["@vocab"].is_string() {
            // Check if the @vocab is a valid URI provided in the name_spaces module
            let vocab = context["@vocab"].as_str().unwrap();
            if !name_spaces.contains(&vocab) {
                println!("Invalid @vocab: {}", vocab);
                return false;
            }
        } else {
            return false;
        }
    } else if context.is_string() {
        // Check if the @vocab is a valid URI provided in the name_spaces module
        let mut vocab = context.as_str().unwrap().to_owned();
        if !vocab.ends_with("#") {
            vocab.push('#');
        }
        if !name_spaces.contains(&vocab.as_str()) {
            println!("Invalid @vocab: {}", vocab);
            return false;
        }
    } else {
        return false;
    }
    true
}

fn validate_conflict(v: &Value) -> bool {
    println!("Validating conflict: {:?}", v);

    let valid_conflict_term = vec![
        "perm",     // the Permissions MUST override the Prohibitions
        "prohibit", // the Prohibitions MUST override the Permissions
        "invalid",  // the entire Policy MUST be void if any conflict is detected
    ];

    if v.is_string() {
        // Check if the conflict is a valid ConflictTerm
        if !valid_conflict_term.contains(&v.as_str().unwrap()) {
            println!("Invalid conflict: {}", v.as_str().unwrap());
            return false;
        }
    } else {
        return false;
    }

    true
}

fn validate_policy(v: &Value) -> bool {
    println!("Validating policy: {:?}", v);

    /*
    A Policy MUST have one uid property value (of type IRI [rfc3987]) to identify the Policy.
    A Policy MUST have at least one permission, prohibition, or obligation property values of type Rule.
    A Policy MAY have none, one, or many profile property values (of type IRI [rfc3987]) to identify the ODRL Profile that this Policy conforms to.
    A Policy MAY have none, one, or many inheritFrom property values (of type IRI [rfc3987]) to identify the parent Policy from which this child Policy inherits from.
    A Policy MAY have none or one conflict property values (of type ConflictTerm) for Conflict Strategy Preferences indicating how to handle Policy conflicts.
    */

    // Validate uid key
    if v.get("uid").is_none() {
        println!("No uid key found");
        return false;
    } else {
        // Check if the uid is a valid URI
        if !validate_uid(&v["uid"]) {
            println!("Invalid uid");
            return false;
        }
    }

    // Validate rule keys are given
    if v.get("permission").is_some() {
        for i in v["permission"].as_array().unwrap() {
            if !validate_rule(i, "permission") {
                println!("Invalid permission");
                return false;
            }
        }
    } else if v.get("prohibition").is_some() {
        for i in v["prohibition"].as_array().unwrap() {
            if !validate_rule(i, "prohibition") {
                println!("Invalid prohibition");
                return false;
            }
        }
    } else if v.get("obligation").is_some() {
        for i in v["obligation"].as_array().unwrap() {
            if !validate_rule(i, "obligation") {
                println!("Invalid obligation");
                return false;
            }
        }
    } else {
        println!("No permission, prohibition or obligation key found");
        return false;
    }

    // Validate optional profile key
    if v.get("profile").is_some() {
        for i in v["profile"].as_array().unwrap() {
            // Check if the profile is a valid URI
            if !i.as_str().unwrap().starts_with("http")
                && !i.as_str().unwrap().starts_with("https://")
            {
                println!("Invalid profile: {}", i.as_str().unwrap());
                return false;
            }
        }
    }

    // Validate optional inheritFrom key
    if v.get("inheritFrom").is_some() {
        for i in v["inheritFrom"].as_array().unwrap() {
            // Check if the inheritFrom is a valid URI
            if !i.as_str().unwrap().starts_with("http")
                && !i.as_str().unwrap().starts_with("https://")
            {
                println!("Invalid inheritFrom: {}", i.as_str().unwrap());
                return false;
            }
        }
    }

    // Validate optional conflict key
    if v.get("conflict").is_some() {
        // Check if the conflict is a valid ConflictTerm
        if !validate_conflict(&v["conflict"]) {
            println!("Invalid conflict");
            return false;
        }
    }

    true
}

fn validate_set(v: &Value) -> bool {
    println!("Validating policy of type Set: {:?}", v);

    /*
    A Policy MUST have one uid property value (of type IRI [rfc3987]) to identify the Policy.
    A Policy MUST have at least one permission, prohibition, or obligation property values of type Rule.
    A Policy MAY have none, one, or many profile property values (of type IRI [rfc3987]) to identify the ODRL Profile that this Policy conforms to.
    A Policy MAY have none, one, or many inheritFrom property values (of type IRI [rfc3987]) to identify the parent Policy from which this child Policy inherits from.
    A Policy MAY have none or one conflict property values (of type ConflictTerm) for Conflict Strategy Preferences indicating how to handle Policy conflicts.

    An ODRL Policy of subclass Set represents any combination of Rules. The Set Policy subclass is also the default subclass of Policy (if none is specified).
    */

    // Validate uid key
    if v.get("uid").is_none() {
        println!("No uid key found");
        return false;
    } else {
        // Check if the uid is a valid URI
        if !validate_uid(&v["uid"]) {
            println!("Invalid uid");
            return false;
        }
    }

    // Validate rule keys are given
    if v.get("permission").is_some() {
        for i in v["permission"].as_array().unwrap() {
            if !validate_rule(i, "permission") {
                println!("Invalid permission");
                return false;
            }
        }
    } else if v.get("prohibition").is_some() {
        for i in v["prohibition"].as_array().unwrap() {
            if !validate_rule(i, "prohibition") {
                println!("Invalid prohibition");
                return false;
            }
        }
    } else if v.get("obligation").is_some() {
        for i in v["obligation"].as_array().unwrap() {
            if !validate_rule(i, "obligation") {
                println!("Invalid obligation");
                return false;
            }
        }
    } else {
        println!("No permission, prohibition or obligation key found");
        return false;
    }

    // Validate optional profile key
    if v.get("profile").is_some() {
        for i in v["profile"].as_array().unwrap() {
            // Check if the profile is a valid URI
            if !i.as_str().unwrap().starts_with("http")
                && !i.as_str().unwrap().starts_with("https://")
            {
                println!("Invalid profile: {}", i.as_str().unwrap());
                return false;
            }
        }
    }

    // Validate optional inheritFrom key
    if v.get("inheritFrom").is_some() {
        for i in v["inheritFrom"].as_array().unwrap() {
            // Check if the inheritFrom is a valid URI
            if !i.as_str().unwrap().starts_with("http")
                && !i.as_str().unwrap().starts_with("https://")
            {
                println!("Invalid inheritFrom: {}", i.as_str().unwrap());
                return false;
            }
        }
    }

    // Validate optional conflict key
    if v.get("conflict").is_some() {
        // Check if the conflict is a valid ConflictTerm
        if !validate_conflict(&v["conflict"]) {
            println!("Invalid conflict");
            return false;
        }
    }

    true
}

fn validate_offer(v: &Value) -> bool {
    println!("Validating policy of type Offer: {:?}", v);

    /*
    A Policy MUST have one uid property value (of type IRI [rfc3987]) to identify the Policy.
    A Policy MUST have at least one permission, prohibition, or obligation property values of type Rule.
    A Policy MAY have none, one, or many profile property values (of type IRI [rfc3987]) to identify the ODRL Profile that this Policy conforms to.
    A Policy MAY have none, one, or many inheritFrom property values (of type IRI [rfc3987]) to identify the parent Policy from which this child Policy inherits from.
    A Policy MAY have none or one conflict property values (of type ConflictTerm) for Conflict Strategy Preferences indicating how to handle Policy conflicts.

    An ODRL Policy of subclass Offer:
    MUST have one assigner property value (of type Party) to indicate the functional role in the same Rules.
    */

    // Validate uid key
    if v.get("uid").is_none() {
        println!("No uid key found");
        return false;
    } else {
        // Check if the uid is a valid URI
        if !validate_uid(&v["uid"]) {
            println!("Invalid uid");
            return false;
        }
    }

    // Validate rule keys are given
    if v.get("permission").is_some() {
        for i in v["permission"].as_array().unwrap() {
            if i.get("assigner").is_none() {
                println!("No assigner key found in permission. Invalid Offer.");
                return false;
            }
            if !validate_rule(i, "permission") {
                println!("Invalid permission");
                return false;
            }
        }
    } else if v.get("prohibition").is_some() {
        for i in v["prohibition"].as_array().unwrap() {
            if i.get("assigner").is_none() {
                println!("No assigner key found in prohibition. Invalid Offer.");
                return false;
            }
            if !validate_rule(i, "prohibition") {
                println!("Invalid prohibition");
                return false;
            }
        }
    } else if v.get("obligation").is_some() {
        for i in v["obligation"].as_array().unwrap() {
            if i.get("assigner").is_none() {
                println!("No assigner key found in obligation. Invalid Offer.");
                return false;
            }
            if !validate_rule(i, "obligation") {
                println!("Invalid obligation");
                return false;
            }
        }
    } else {
        println!("No permission, prohibition or obligation key found");
        return false;
    }

    // Validate optional profile key
    if v.get("profile").is_some() {
        for i in v["profile"].as_array().unwrap() {
            // Check if the profile is a valid URI
            if !i.as_str().unwrap().starts_with("http")
                && !i.as_str().unwrap().starts_with("https://")
            {
                println!("Invalid profile: {}", i.as_str().unwrap());
                return false;
            }
        }
    }

    // Validate optional inheritFrom key
    if v.get("inheritFrom").is_some() {
        for i in v["inheritFrom"].as_array().unwrap() {
            // Check if the inheritFrom is a valid URI
            if !i.as_str().unwrap().starts_with("http")
                && !i.as_str().unwrap().starts_with("https://")
            {
                println!("Invalid inheritFrom: {}", i.as_str().unwrap());
                return false;
            }
        }
    }

    // Validate optional conflict key
    if v.get("conflict").is_some() {
        // Check if the conflict is a valid ConflictTerm
        if !validate_conflict(&v["conflict"]) {
            println!("Invalid conflict");
            return false;
        }
    }

    true
}

fn validate_agreement(v: &Value) -> bool {
    println!("Validating policy of type Agreement: {:?}", v);

    /*
    A Policy MUST have one uid property value (of type IRI [rfc3987]) to identify the Policy.
    A Policy MUST have at least one permission, prohibition, or obligation property values of type Rule.
    A Policy MAY have none, one, or many profile property values (of type IRI [rfc3987]) to identify the ODRL Profile that this Policy conforms to.
    A Policy MAY have none, one, or many inheritFrom property values (of type IRI [rfc3987]) to identify the parent Policy from which this child Policy inherits from.
    A Policy MAY have none or one conflict property values (of type ConflictTerm) for Conflict Strategy Preferences indicating how to handle Policy conflicts.

    An ODRL Policy of subclass Agreement:
    MUST have one assigner property value (of type Party) to indicate the functional role in the same Rules.
    MUST have one assignee property value (of type Party) to indicate the functional role in the same Rules.
    */

    // Validate uid key
    if v.get("uid").is_none() {
        println!("No uid key found");
        return false;
    } else {
        // Check if the uid is a valid URI
        if !validate_uid(&v["uid"]) {
            println!("Invalid uid");
            return false;
        }
    }

    // Validate rule keys are given
    if v.get("permission").is_some() {
        for i in v["permission"].as_array().unwrap() {
            if i.get("assigner").is_none() || i.get("assignee").is_none() {
                println!("No assigner or assignee key found in permission. Invalid Agreement.");
                return false;
            }
            if !validate_rule(i, "permission") {
                println!("Invalid permission");
                return false;
            }
        }
    } else if v.get("prohibition").is_some() {
        for i in v["prohibition"].as_array().unwrap() {
            if i.get("assigner").is_none() || i.get("assignee").is_none() {
                println!("No assigner or assignee key found in prohibition. Invalid Agreement.");
                return false;
            }
            if !validate_rule(i, "prohibition") {
                println!("Invalid prohibition");
                return false;
            }
        }
    } else if v.get("obligation").is_some() {
        for i in v["obligation"].as_array().unwrap() {
            if i.get("assigner").is_none() || i.get("assignee").is_none() {
                println!("No assigner or assignee key found in obligation. Invalid Agreement.");
                return false;
            }
            if !validate_rule(i, "obligation") {
                println!("Invalid obligation");
                return false;
            }
        }
    } else {
        println!("No permission, prohibition or obligation key found");
        return false;
    }

    // Validate optional profile key
    if v.get("profile").is_some() {
        for i in v["profile"].as_array().unwrap() {
            // Check if the profile is a valid URI
            if !i.as_str().unwrap().starts_with("http")
                && !i.as_str().unwrap().starts_with("https://")
            {
                println!("Invalid profile: {}", i.as_str().unwrap());
                return false;
            }
        }
    }

    // Validate optional inheritFrom key
    if v.get("inheritFrom").is_some() {
        for i in v["inheritFrom"].as_array().unwrap() {
            // Check if the inheritFrom is a valid URI
            if !i.as_str().unwrap().starts_with("http")
                && !i.as_str().unwrap().starts_with("https://")
            {
                println!("Invalid inheritFrom: {}", i.as_str().unwrap());
                return false;
            }
        }
    }

    // Validate optional conflict key
    if v.get("conflict").is_some() {
        // Check if the conflict is a valid ConflictTerm
        if !validate_conflict(&v["conflict"]) {
            println!("Invalid conflict");
            return false;
        }
    }

    true
}

fn validate_rule(v: &Value, t: &str) -> bool {
    println!("Validating {}: {:?}", t, v);

    if t == "permission" {
        if !validate_permission(v) {
            println!("Invalid permission");
            return false;
        }
    } else if t == "prohibition" {
        if !validate_prohibition(v) {
            println!("Invalid prohibition");
            return false;
        }
    } else if t == "duty" {
        if !validate_duty(v) {
            println!("Invalid duty");
            return false;
        }
    } else if t == "obligation" {
        if !validate_obligation(v) {
            println!("Invalid obligation");
            return false;
        }
    }

    true
}

fn validate_permission(v: &Value) -> bool {
    /*
    A Permission MUST have one target property value of type Asset.
    A Permission MAY have none or one assigner and/or assignee property values (of type Party) for functional roles.
    A Permission MAY have none, one, or more duty property values of type Duty.

    A Rule MUST have one action property value of type Action.
    A Rule MAY have none, one or many constraint property values of type Constraint/LogicalConstraint.
    A Rule MAY have none or one uid property values (of type IRI [rfc3987]) to identify the Rule so it MAY be referenced by other Rules.
    */

    // Validate optional uid key
    if v.get("uid").is_some() {
        validate_uid(&v["uid"]);
    }

    // Validate the target key
    if v.get("target").is_none() {
        println!("No target key found");
        return false;
    } else {
        // Check if the target is a valid URI
        if !v["target"].as_str().unwrap().starts_with("http")
            && !v["target"].as_str().unwrap().starts_with("https://")
        {
            println!("Invalid target: {}", v["target"].as_str().unwrap());
            return false;
        }
    }

    // Validate optional assigner key
    if v.get("assigner").is_some() {
        // Check if the assigner is a valid URI
        if !v["assigner"].as_str().unwrap().starts_with("http")
            && !v["assigner"].as_str().unwrap().starts_with("https://")
        {
            println!("Invalid assigner: {}", v["assigner"].as_str().unwrap());
            return false;
        }
    }

    // Validate optional assignee key
    if v.get("assignee").is_some() {
        // Check if the assignee is a valid URI
        if !v["assignee"].as_str().unwrap().starts_with("http")
            && !v["assignee"].as_str().unwrap().starts_with("https://")
        {
            println!("Invalid assignee: {}", v["assignee"].as_str().unwrap());
            return false;
        }
    }

    // Validate action key
    if v.get("action").is_none() {
        println!("No action key found");
        return false;
    } else {
        // Check if the action is a valid action
        if !validate_action(&v["action"]) {
            println!("Invalid action");
            return false;
        }
    }

    // Validate optional constraints key
    if v.get("constraint").is_some() {
        for i in v["constraint"].as_array().unwrap() {
            if !validate_constraint(i) {
                println!("Invalid constraint");
                return false;
            }
        }
    }

    // Validate optional duties key
    if v.get("duty").is_some() {
        for i in v["duty"].as_array().unwrap() {
            if !validate_duty(i) {
                println!("Invalid duty");
                return false;
            }
        }
    }

    true
}

fn validate_prohibition(v: &Value) -> bool {
    /*
    A Prohibition MUST have one target property value of type Asset. (Other relation sub-properties MAY be used.)
    A Prohibition MAY have none or one assigner and/or assignee property values (of type Party) for functional roles. (Other function sub-properties MAY be used.)
    A Prohibition MAY have none, one, or more remedy property values of type Duty.

    A Rule MUST have one action property value of type Action.
    A Rule MAY have none, one or many constraint property values of type Constraint/LogicalConstraint.
    A Rule MAY have none or one uid property values (of type IRI [rfc3987]) to identify the Rule so it MAY be referenced by other Rules.
    */

    // Validate optional uid key
    if v.get("uid").is_some() {
        validate_uid(&v["uid"]);
    }

    // Validate the target key
    if v.get("target").is_none() {
        println!("No target key found");
        return false;
    } else {
        // Check if the target is a valid URI
        if !v["target"].as_str().unwrap().starts_with("http")
            && !v["target"].as_str().unwrap().starts_with("https://")
        {
            println!("Invalid target: {}", v["target"].as_str().unwrap());
            return false;
        }
    }

    // Validate optional assigner key
    if v.get("assigner").is_some() {
        // Check if the assigner is a valid URI
        if !v["assigner"].as_str().unwrap().starts_with("http")
            && !v["assigner"].as_str().unwrap().starts_with("https://")
        {
            println!("Invalid assigner: {}", v["assigner"].as_str().unwrap());
            return false;
        }
    }

    // Validate optional assignee key
    if v.get("assignee").is_some() {
        // Check if the assignee is a valid URI
        if !v["assignee"].as_str().unwrap().starts_with("http")
            && !v["assignee"].as_str().unwrap().starts_with("https://")
        {
            println!("Invalid assignee: {}", v["assignee"].as_str().unwrap());
            return false;
        }
    }

    // Validate action key
    if v.get("action").is_none() {
        println!("No action key found");
        return false;
    } else {
        // Check if the action is a valid action
        if !validate_action(&v["action"]) {
            println!("Invalid action");
            return false;
        }
    }

    // Validate optional constraints key
    if v.get("constraint").is_some() {
        for i in v["constraint"].as_array().unwrap() {
            if !validate_constraint(i) {
                println!("Invalid constraint");
                return false;
            }
        }
    }

    // Validate optional remedies key
    if v.get("remedy").is_some() {
        for i in v["remedy"].as_array().unwrap() {
            if !validate_remedy(i) {
                println!("Invalid remedy");
                return false;
            }
        }
    }

    true
}

fn validate_remedy(v: &Value) -> bool {
    /*
    The remedy property expresses an agreed Duty that MUST be fulfilled in case that a Prohibition has been infringed by being exercised.
    If the Prohibition action is exercised, then all remedy Duties MUST be fulfilled to address the infringement of the Prohibition and set it to the state not infringed.
    The remedy property is a sub-property of the failure property.

    A remedy MUST NOT refer to a Duty that includes a consequence Duty.
    */

    // Validate optional uid key
    if v.get("uid").is_some() {
        validate_uid(&v["uid"]);
    }

    // Validate optional target key
    if v.get("target").is_some() {
        // Check if the target is a valid URI
        if !v["target"].as_str().unwrap().starts_with("http")
            && !v["target"].as_str().unwrap().starts_with("https://")
        {
            println!("Invalid target: {}", v["target"].as_str().unwrap());
            return false;
        }
    }

    // Validate optional assigner key
    if v.get("assigner").is_some() {
        // Check if the assigner is a valid URI
        if !v["assigner"].as_str().unwrap().starts_with("http")
            && !v["assigner"].as_str().unwrap().starts_with("https://")
        {
            println!("Invalid assigner: {}", v["assigner"].as_str().unwrap());
            return false;
        }
    }

    // Validate optional assignee key
    if v.get("assignee").is_some() {
        // Check if the assignee is a valid URI
        if !v["assignee"].as_str().unwrap().starts_with("http")
            && !v["assignee"].as_str().unwrap().starts_with("https://")
        {
            println!("Invalid assignee: {}", v["assignee"].as_str().unwrap());
            return false;
        }
    }

    // Validate action key
    if v.get("action").is_none() {
        println!("No action key found");
        return false;
    } else {
        // Check if the action is a valid action
        if !validate_action(&v["action"]) {
            println!("Invalid action");
            return false;
        }
    }

    // Validate there are no consequence key
    if v.get("consequence").is_some() {
        println!("Consequence key found. Invalid Remedy. A remedy MUST NOT refer to a Duty that includes a consequence Duty.");
        return false;
    }

    true
}

fn validate_duty(v: &Value) -> bool {
    /*
    A Duty MAY have none or one target property values (of type Asset) to indicate the Asset that is the primary subject to which the Duty directly applies.
    A Duty MAY have none or one assigner and/or assignee property values (of type Party) for functional roles.
    A Duty MAY have none, one or many consequence property values of type Duty only when the Duty is referenced by a Rule with the duty or obligation properties.

    A Rule MUST have one action property value of type Action.
    A Rule MAY have none, one or many constraint property values of type Constraint/LogicalConstraint.
    A Rule MAY have none or one uid property values (of type IRI [rfc3987]) to identify the Rule so it MAY be referenced by other Rules.
    */

    // Validate optional uid key
    if v.get("uid").is_some() {
        validate_uid(&v["uid"]);
    }

    // Validate optional target key
    if v.get("target").is_some() {
        // Check if the target is a valid URI
        if !v["target"].as_str().unwrap().starts_with("http")
            && !v["target"].as_str().unwrap().starts_with("https://")
        {
            println!("Invalid target: {}", v["target"].as_str().unwrap());
            return false;
        }
    }

    // Validate optional assigner key
    if v.get("assigner").is_some() {
        // Check if the assigner is a valid URI
        if !v["assigner"].as_str().unwrap().starts_with("http")
            && !v["assigner"].as_str().unwrap().starts_with("https://")
        {
            println!("Invalid assigner: {}", v["assigner"].as_str().unwrap());
            return false;
        }
    }

    // Validate optional assignee key
    if v.get("assignee").is_some() {
        // Check if the assignee is a valid URI
        if !v["assignee"].as_str().unwrap().starts_with("http")
            && !v["assignee"].as_str().unwrap().starts_with("https://")
        {
            println!("Invalid assignee: {}", v["assignee"].as_str().unwrap());
            return false;
        }
    }

    // Validate action key
    if v.get("action").is_none() {
        println!("No action key found");
        return false;
    } else {
        // Check if the action is a valid action
        if !validate_action(&v["action"]) {
            println!("Invalid action");
            return false;
        }
    }

    // Validate optional constraints key
    if v.get("constraint").is_some() {
        for i in v["constraint"].as_array().unwrap() {
            if !validate_constraint(i) {
                println!("Invalid constraint");
                return false;
            }
        }
    }

    // Validate optional consequences key
    if v.get("consequence").is_some() {
        for i in v["consequence"].as_array().unwrap() {
            // Check if the consequence contains a valid action
            if !validate_action(&i["action"]) {
                println!("Invalid consequence action");
                return false;
            }
        }
    }

    true
}

fn validate_obligation(v: &Value) -> bool {
    /*
    A Policy MAY include an obligation to fulfil a Duty. The obligation is fulfilled if all constraints are satisfied and if its action, with all refinements satisfied, has been exercised.

    A Rule MUST have one action property value of type Action.
    A Rule MAY have none, one or many constraint property values of type Constraint/LogicalConstraint.
    A Rule MAY have none or one uid property values (of type IRI [rfc3987]) to identify the Rule so it MAY be referenced by other Rules.
    */

    // Validate optional uid key
    if v.get("uid").is_some() {
        validate_uid(&v["uid"]);
    }

    // Validate optional target key
    if v.get("target").is_some() {
        // Check if the target is a valid URI
        if !v["target"].as_str().unwrap().starts_with("http")
            && !v["target"].as_str().unwrap().starts_with("https://")
        {
            println!("Invalid target: {}", v["target"].as_str().unwrap());
            return false;
        }
    }

    // Validate optional assigner key
    if v.get("assigner").is_some() {
        // Check if the assigner is a valid URI
        if !v["assigner"].as_str().unwrap().starts_with("http")
            && !v["assigner"].as_str().unwrap().starts_with("https://")
        {
            println!("Invalid assigner: {}", v["assigner"].as_str().unwrap());
            return false;
        }
    }

    // Validate optional assignee key
    if v.get("assignee").is_some() {
        // Check if the assignee is a valid URI
        if !v["assignee"].as_str().unwrap().starts_with("http")
            && !v["assignee"].as_str().unwrap().starts_with("https://")
        {
            println!("Invalid assignee: {}", v["assignee"].as_str().unwrap());
            return false;
        }
    }

    // Validate action key
    if v.get("action").is_none() {
        println!("No action key found");
        return false;
    } else {
        // Check if the action is a valid action
        if !validate_action(&v["action"]) {
            println!("Invalid action");
            return false;
        }
    }

    // Validate optional constraints key
    if v.get("constraint").is_some() {
        for i in v["constraint"].as_array().unwrap() {
            if !validate_constraint(i) {
                println!("Invalid constraint");
                return false;
            }
        }
    }

    true
}

fn validate_operator(v: &Value) -> bool {
    println!("Validating operator: {:?}", v);

    let valid_constraint_operators = vec![
        "eq", // The “Equals” operator indicating that a given value equals the rightOperand of the Constraint
        "gt", // The “Greater Than” operator indicating that a given value is greater than the rightOperand of the Constraint
        "gteq", // The “Greater Than or Equal To” operator indicating that a given value is greater than or equal to the rightOperand of the Constraint
        "hasPart", // The “Has Part” operator indicating that a given value contains the rightOperand of the Constraint
        "isA", // The “Is A” operator indicating that a given value is an instance of the rightOperand of the Constraint
        "isAllOf", // The “Is All Of” operator indicating that a given value is all of the rightOperand of the Constraint
        "isAnyOf", // The “Is Any Of” operator indicating that a given value is any of the rightOperand of the Constraint
        "isNoneOf", // The “Is None Of” operator indicating that a given value is none of the rightOperand of the Constraint
        "isPartOf", // The “Is Part Of” operator indicating that a given value is part of the rightOperand of the Constraint
        "lt", // The “Less Than” operator indicating that a given value is less than the rightOperand of the Constraint
        "lteq", // The “Less Than or Equal To” operator indicating that a given value is less than or equal to the rightOperand of the Constraint
        "neq", // The “Not Equal To” operator indicating that a given value is not equal to the rightOperand of the Constraint
    ];

    if v.is_string() {
        // Check if the operator is a valid constraint operator
        if !valid_constraint_operators.contains(&v.as_str().unwrap()) {
            println!("Invalid operator: {}", v.as_str().unwrap());
            return false;
        }
    } else {
        return false;
    }

    true
}

fn validate_operand(v: &Value, t: &str) -> bool {
    println!("Validating {}: {:?}", t, v);

    let valid_left_operand_terms = vec![
        /* A point defined with absolute coordinates
        For example, JPEG image must be positioned at 100×100 pixel location. This may be used to express [PLUS] semantics. */
        "absolutePosition",
        /* The absolute dimension that the Asset may be resized
        For example, JPEG image must be reproduced onto an area no larger than A0. This may be used to express [PLUS] semantics. */
        "absoluteSize",
        /* The absolute spatial positions of four corners of a rectangle on a 2D-canvas or the eight corners of a cuboid in a 3D-space for the target Asset to fit.
        Example: The upper left corner of a picture may be constrained to a specific position of the canvas rendering it. */
        "absoluteSpatialPosition",
        /* The absolute temporal positions in a media stream the target Asset has to fit.
        Use with Actions including the target Asset in a larger media stream. The fragment part of a Media Fragment URI (https://www.w3.org/TR/media-frags/) may be used for the right operand.
        See the Left Operand realativeTemporalPosition.
        Example: The MP3 music file must be positioned between second 192 and 250 of the temporal length of a stream. */
        "absoluteTemporalPosition",
        /* 	The numeric count indicating the number of times the corresponding entity may be exercised
        Should be a positive integer. */
        "count",
        /* The date (and optional time and timezone) representing a point in time or period
        Date and Time value must conform to [ISO-8601] as represented in [W3CXMLSCHEMA]. The use of Timezone information is strongly recommended. */
        "dateTime",
        /* A time delay period prior to exercising the action of the Rule.
        The point in time triggering this period MAY be defined by another temporal Constraint combined by a Logical Constraint (utilising the odrl:andSequence operand).
        Right operand value MUST be an xsd:duration as defined by [xmlschema11-2].
        Only the eq, gt, gteq operators SHOULD be used.
        Example: delayPeriod eq P60M indicates a delay of 60 Minutes before exercising the action.*/
        "delayPeriod",
        /* The delivery channel used for storing or communicating the asset
        For example, the asset may be distributed only on mobile networks. */
        "deliveryChannel",
        /* Deprecated by http://www.w3.org/ns/odrl/2/systemDevice */
        "device",
        /* A period of time in which the policy action can be exercised.
        The start of the period is when the action is first exercised. */
        "elapsedTime",
        /* 	Specification of a defined event applicable to the asset usage
        For example, asset may be used at the “FIFA World Cup” only. To express events related to undertaking Duties, a specific event value has been defined:
            policyUsage – the time period whilst the policy is being exercised
        This will enable constraints to be expressed such as “event lt o:policyUsage” indicating before the policy is exercised. */
        "event",
        /* The file format applicable to the Asset
        For example, this may be used to express [PLUS] semantics; only JPEG image may be distributed. */
        "fileFormat",
        /* The defined industry sector applicable to the asset usage
        For example, publishing, financial. */
        "industry",
        /* The natural language applicable to the asset usage
        For example, this may be used to express [PLUS] semantics; JPEG image may only be reproduced with Spanish text. Must use [BCP-47] codes. */
        "language",
        /* The media type in which the asset may be used
        For example, electronic, print, advertising, marketing. This may be used to express [PLUS] semantics. */
        "media",
        /* The maximum period of metered usage time
        Value must conform to [ISO-8601] as represented in [W3CXMLSCHEMA]. For example “P30H” indicates a 30-hour period. */
        "meteredTime",
        /* The value of the financial payment
        The dataType attribute may be used to indicate the type of the value (eg decimal) and the unit attribute to indicate the currency.
        May be used for compensation duties. */
        "payAmount",
        /* The amount (as a percentage) of the action applicable to the asset
        A numeric value from 0 to 100. For example, extract a maximum of 50% of the asset */
        "percentage",
        /* The specified Product or Service name
        For example, this may be used to express [PLUS] semantics; images may only be reproduced in the XYZ Magazine. */
        "product",
        /* Specification of a defined purpose applicable to the asset usage
        For example, educational use. [P3P] Purpose values may also be used. */
        "purpose",
        /* The party that receives the result of the Action on the Asset
        The right operand must identify one or more specific parties or categories of party */
        "recipient",
        /* 	A point defined with reference to another position
        For example, this may be used to express [PLUS] semantics; JPEG image must be positioned at the Top of the Page. */
        "relativePosition",
        /* The relative dimension that the Asset may be resized
        For example, this may be used to express [PLUS] semantics; JPEG image resized to maximum of 200%. */
        "relativeSize",
        /* The relative spatial positions - expressed as percentages of full values - of four corners of a rectangle on a 2D-canvas or the eight corners of a cuboid in a 3D-space of the target Asset. */
        "relativeSpatialPosition",
        /* A point in space or time defined with coordinates relative to full measures the positioning of the target Asset.
        Example: The MP3 music file must be positioned between the positions at 33% and 48% of the temporal length of a stream.*/
        "relativeTemporalPosition",
        /* The resolution at which the asset may be used
        For example, may be printed at 1200dpi. */
        "resolution",
        /* A code representing a geospatial area
        The code value and code source must be represented.
        For example, the ISO-3166 Country Codes and the Getty Thesaurus of Geographic Names. A URI should be used to represent this value. */
        "spatial",
        /* A set of coordinates setting the borders of a geospatial area used for exercising the action of the Rule. The coordinates MUST include longitude and latitude, they MAY include altitude and the geodetic datum.
        The default values are the altitude of earth's surface at this location and the WGS 84 datum. */
        "spatialCoordinates",
        /* Deprecated by http://www.w3.org/ns/odrl/2/systemDevice */
        "system",
        /* An identified computing system or computing device used for exercising the action of the Rule.
        Example: The system device can be identified by a unique code created from the used hardware. */
        "systemDevice",
        /* Recurring period of time in which the usage may be exercised
        Interval value must conform to [ISO-8601] as represented in [W3CXMLSCHEMA]. For example, “P7D” indicates a 7 day period. */
        "timeInterval",
        /* The unit of measure used for counting the executions of the action of the Rule.
        Note: Typically used with Duties to indicate the unit entity to be counted of the Action.
        Example: A duty to compensate and a unitOfCount constraint of 'perUser' would indicate that the compensation by multiplied by the 'number of users'. */
        "unitOfCount",
        /* The scope of versions for the asset
        For example, Single Paperback, or Multiple Issues. This may be used to express [PLUS] semantics. */
        "version",
        /* Specification of a digital locale
        For example, an Internet domain or IP address range */
        "virtualLocation",
    ];

    // https://www.w3.org/TR/xmlschema11-2/#built-in-datatypes
    let valid_right_operand_type = vec![
        "xsd:anyType",
        "xsd:anySimpleType",
        "xsd:anyAtomicType",
        "xsd:anyURI",
        "xsd:base64Binary",
        "xsd:boolean",
        "xsd:date",
        "xsd:dateTime",
        "xsd:dateTimeStamp",
        "xsd:decimal",
        "xsd:integer",
        "xsd:long",
        "xsd:int",
        "xsd:short",
        "xsd:byte",
        "xsd:nonNegativeInteger",
        "xsd:positiveInteger",
        "xsd:unsignedLong",
        "xsd:unsignedInt",
        "xsd:unsignedShort",
        "xsd:unsignedByte",
        "xsd:nonPositiveInteger",
        "xsd:negativeInteger",
        "xsd:double",
        "xsd:duration",
        "xsd:dayTimeDuration",
        "xsd:yearMonthDuration",
        "xsd:float",
        "xsd:gDay",
        "xsd:gMonth",
        "xsd:gMonthDay",
        "xsd:gYear",
        "xsd:gYearMonth",
        "xsd:hexBinary",
        "xsd:NOTATION",
        "xsd:QName",
        "xsd:string",
        "xsd:normalizedString",
        "xsd:token",
        "xsd:language",
        "xsd:Name",
        "xsd:NCName",
        "xsd:ENTITY",
        "xsd:ID",
        "xsd:IDREF",
        "xsd:NMTOKEN",
        "xsd:time",
        "xsd:ENTITIES",
        "xsd:IDREFS",
        "xsd:NMTOKENS",
    ];

    if t == "leftOperand" {
        if v.is_string() {
            // Check if the leftOperand is a valid URI or a valid constraint term
            if !v.as_str().unwrap().starts_with("http")
                && !v.as_str().unwrap().starts_with("https://")
                && !valid_left_operand_terms.contains(&v.as_str().unwrap())
            {
                println!("Invalid leftOperand: {}", v.as_str().unwrap());
                return false;
            }
        } else {
            return false;
        }
    } else if t == "rightOperand" {
        if v.is_string() {
            // Check if the rightOperand is a valid UR
            if !v.as_str().unwrap().starts_with("http")
                && !v.as_str().unwrap().starts_with("https://")
            {
                println!("Invalid rightOperand: {}", v.as_str().unwrap());
                return false;
            }
        } else if v.is_object() {
            // Check if the rightOperand is a valid JSON object
            if v.get("@value").is_none() || v.get("@type").is_none() {
                println!("Invalid rightOperand: {:?}", v);
                return false;
            }
            // Check if the type of the rightOperand is a valid xsd type provided by XMLSchema https://www.w3.org/2001/XMLSchema#
            let data_type = v["@type"].as_str().unwrap();
            if !valid_right_operand_type.contains(&data_type) {
                println!("Invalid rightOperand type: {}", data_type);
                return false;
            }
            // TODO: Check if the value of the rightOperand is valid (check format, etc.) based on the data type
        } else {
            return false;
        }
    }
    true
}

fn validate_constraint(v: &Value) -> bool {
    println!("Validating constraint: {:?}", v);

    // Check if the constraint has an leftOperand / operator / rightOperand key
    if v.get("leftOperand").is_none() {
        println!("No leftOperand key found");
        return false;
    } else if v.get("operator").is_none() {
        println!("No operator key found");
        return false;
    } else if v.get("rightOperand").is_none() {
        println!("No rightOperand key found");
        return false;
    }

    // Validate the leftOperand / operator / rightOperand
    if !validate_operand(&v["leftOperand"], "leftOperand") {
        println!("Invalid leftOperand");
        return false;
    } else if !validate_operator(&v["operator"]) {
        println!("Invalid operator");
        return false;
    } else if !validate_operand(&v["rightOperand"], "rightOperand") {
        println!("Invalid rightOperand");
        return false;
    }

    // Validate optional unit key
    if v.get("unit").is_some() {
        if !v["unit"].as_str().unwrap().starts_with("http")
            && !v["unit"].as_str().unwrap().starts_with("https://")
        {
            println!("Invalid unit: {}", v["unit"].as_str().unwrap());
            return false;
        }
    }

    // Validate optional uid key
    if v.get("uid").is_some() {
        validate_uid(&v["uid"]);
    }

    true
}

fn validate_logical_constraint(v: &Value) -> bool {
    println!("Validating logical constraint: {:?}", v);

    // Validate optional uid key
    if v.get("uid").is_some() {
        validate_uid(&v["uid"]);
    }

    if v.get("and").is_some() {
        // Validate list of constraints
        if v["and"].get("@list").is_none() {
            println!("No @list key found");
            return false;
        } else {
            for i in v["and"]["@list"].as_array().unwrap() {
                if i.get("@id").is_none() {
                    println!("No @id key found");
                    return false;
                } else {
                    validate_uid(&i["@id"]);
                }
            }
        }
    } else if v.get("andSequence").is_some() {
        // Validate list of constraints
        if v["andSequence"].get("@list").is_none() {
            println!("No @list key found");
            return false;
        } else {
            for i in v["andSequence"]["@list"].as_array().unwrap() {
                if i.get("@id").is_none() {
                    println!("No @id key found");
                    return false;
                } else {
                    validate_uid(&i["@id"]);
                }
            }
        }
    } else if v.get("or").is_some() {
        // Validate list of constraints
        if v["or"].get("@list").is_none() {
            println!("No @list key found");
            return false;
        } else {
            for i in v["or"]["@list"].as_array().unwrap() {
                if i.get("@id").is_none() {
                    println!("No @id key found");
                    return false;
                } else {
                    validate_uid(&i["@id"]);
                }
            }
        }
    } else if v.get("xone").is_some() {
        // Validate list of constraints
        if v["xone"].get("@list").is_none() {
            println!("No @list key found");
            return false;
        } else {
            for i in v["xone"]["@list"].as_array().unwrap() {
                if i.get("@id").is_none() {
                    println!("No @id key found");
                    return false;
                } else {
                    validate_uid(&i["@id"]);
                }
            }
        }
    } else {
        println!("No valid logical operand found. Must have and/andSequence/or/xone key.");
        return false;
    }

    true
}

fn validate_action(v: &Value) -> bool {
    println!("Validating action: {:?}", v);

    let valid_odrl_actions = vec![
        "Attribution", // Credit be given to copyright holder and/or author. -> Included in use
        "CommercialUse", // Exercising rights for commercial purposes. -> Included in use
        "DerivativeWorks", // Distribution of derivative works. -> Included in use
        "Distribution", // Distribution, public display, and publicly performance. -> Included in use
        "Notice",       // Copyright and license notices be kept intact. -> Included in use
        "Reproduction", // Making multiple copies. -> Included in use
        "ShareAlike", // Derivative works be licensed under the same terms or compatible terms as the original work. -> Included in use
        "Sharing", // Permits commercial derivatives, but only non-commercial distribution. -> Included in use
        "SourceCode", // Source code (the preferred form for making modifications) must be provided when exercising some rights granted by the license. -> Included in use
        "acceptTracking", // To accept that the use of the Asset may be tracked. -> Included in use
        "adHocShare", // Deprecated
        "aggregate", // To use the Asset or parts of it as part of a composite collection. -> Included in use
        "annotate", // To add explanatory notations/commentaries to the Asset without modifying the Asset in any other way. -> Included in use
        "anonymize", // To anonymize all or parts of the Asset. -> Included in use
        "append",   // Deprecated by http://www.w3.org/ns/odrl/2/modify
        "appendTo", // Deprecated by http://www.w3.org/ns/odrl/2/modify
        "archive",  // To store the Asset (in a non-transient form). -> Included in use
        "attachPolicy", // Deprecated by http://creativecommons.org/ns#Notice
        "attachSource", // Deprecated by http://creativecommons.org/ns#SourceCode
        "attribute", // To attribute the use of the Asset. -> Included in use
        "commercialize", // Deprecated by http://creativecommons.org/ns#CommercialUse
        "compensate", // To compensate by transfer of some amount of value, if defined, for using or selling the Asset. -> Included in use
        "concurrentUse", // To create multiple copies of the Asset that are being concurrently used. -> Included in use
        "copy",          // Deprecated by http://www.w3.org/ns/odrl/2/reproduce
        "delete", // To permanently remove all copies of the Asset after it has been used. -> Included in use
        "derive", // To create a new derivative Asset from this Asset and to edit or modify the derivative. -> Included in use
        "digitize", // To produce a digital copy of (or otherwise digitize) the Asset from its analogue form. -> Included in use
        "display",  // To create a static and transient rendition of an Asset. -> Included in play
        "distribute", // To supply the Asset to third-parties. -> Included in use
        "ensureExclusivity", // To ensure that the Rule on the Asset is exclusive. -> Included in use
        "execute",           // To run the computer program Asset. -> Included in use
        "export",            // Deprecated by http://www.w3.org/ns/odrl/2/transform
        "extract", // To extract parts of the Asset and to use it as a new Asset. -> Included in reproduce
        "extractChar", // Deprecated
        "extractPage", // Deprecated
        "extractWord", // Deprecated
        "give", // To transfer the ownership of the Asset to a third party without compensation and while deleting the original asset. -> Included in transfer
        "grantUse", // To grant the use of the Asset to third parties. -> Included in use
        "include", // To include other related assets in the Asset. -> Included in use
        "index", // To record the Asset in an index. -> Included in use
        "inform", // To inform that an action has been performed on or in relation to the Asset. -> Included in use
        "install", // To load the computer program Asset onto a storage device which allows operating or running the Asset. -> Included in use
        "lease",   // Deprecated
        "lend",    // Deprecated
        "license", // Deprecated by http://www.w3.org/ns/odrl/2/grantUse
        "modify", // To change existing content of the Asset. A new asset is not created by this action. -> Included in use
        "move", // To move the Asset from one digital location to another including deleting the original copy. -> Included in use
        "nextPolicy", // To grant the specified Policy to a third party for their use of the Asset. -> Included in use
        "obtainConsent", // To obtain verifiable consent to perform the requested action in relation to the Asset. -> Included in use
        "pay",           // Deprecated by http://www.w3.org/ns/odrl/2/compensate
        "play", // To create a sequential and transient rendition of an Asset. -> Included in use
        "present", // To publicly perform the Asset. -> Included in use
        "preview", // Deprecated
        "print", // To create a tangible and permanent rendition of an Asset. -> Included in use
        "read", // To obtain data from the Asset. -> Included in use
        "reproduce", // To make duplicate copies the Asset in any material form. -> Included in use
        "reviewPolicy", // To review the Policy applicable to the Asset. -> Included in use
        "secondaryUse", // Deprecated
        "sell", // To transfer the ownership of the Asset to a third party with compensation and while deleting the original asset. -> Included in transfer
        "share", // Deprecated by http://creativecommons.org/ns#Sharing
        "shareAlike", // Deprecated by http://creativecommons.org/ns#ShareAlike
        "stream", // To deliver the Asset in real-time. -> Included in use
        "synchronize", // To use the Asset in timed relations with media (audio/visual) elements of another Asset. -> Included in use
        "textToSpeech", // To have a text Asset read out loud. -> Included in use
        "transfer",    // To transfer the ownership of the Asset in perpetuity.
        "transform",   // To convert the Asset into a different format. -> Included in use
        "translate", // To translate the original natural language of an Asset into another natural language. -> Included in use
        "uninstall", // To unload and delete the computer program Asset from a storage device and disable its readiness for operation. -> Included in use
        "use", // To use the Asset. Note: Use is the most generic action for all non-third-party usage. More specific types of the use action can be expressed by more targeted actions.
        "watermark", // To apply a watermark to the Asset. -> Included in use
        "write", // Deprecated by http://www.w3.org/ns/odrl/2/modify
        "writeTo", // Deprecated by http://www.w3.org/ns/odrl/2/modify
    ];

    if v.is_string() {
        // Check if the action is a valid ODRL action
        if !valid_odrl_actions.contains(&v.as_str().unwrap()) {
            println!("Invalid action: {}", v.as_str().unwrap());
            return false;
        }
    } else if v.is_object() {
        // Check if the action has a rdf:value key
        if v.get("rdf:value").is_none() {
            println!("No rdf:value key found");
            return false;
        }

        // Validate the rdf:value
        if v["rdf:value"].as_object().unwrap().contains_key("@id") {
            let action = v["rdf:value"].as_object().unwrap()["@id"].as_str().unwrap();
            if !valid_odrl_actions.contains(&action.replace("odrl:", "").as_str()) {
                println!("Invalid action: {}", action);
                return false;
            }
        } else {
            println!("No @id key found");
            return false;
        }

        // Validate optional refinement key
        if v.get("refinement").is_some() {
            if v["refinement"].is_array() {
                // must be a constraint
                for i in v["refinement"].as_array().unwrap() {
                    if !validate_constraint(&i) {
                        println!("Invalid constraint");
                        return false;
                    }
                }
            } else if v["refinement"].is_object() {
                // must be a logical constraint
                if !validate_logical_constraint(&v["refinement"]) {
                    println!("Invalid logical constraint: {:?}", v["refinement"]);
                    return false;
                }
            } else {
                println!("Invalid refinement: {:?}", v["refinement"]);
                return false;
            }
        }
    } else if v.is_array() {
        for i in v.as_array().unwrap() {
            if i.is_string() {
                // Check if the action is a valid ODRL action
                if !valid_odrl_actions.contains(&i.as_str().unwrap()) {
                    println!("Invalid action: {}", i.as_str().unwrap());
                    return false;
                }
            } else if i.is_object() {
                // Check if the action has a rdf:value key
                if i.get("rdf:value").is_none() {
                    println!("No rdf:value key found");
                    return false;
                }

                // Validate the rdf:value
                if i["rdf:value"].as_object().unwrap().contains_key("@id") {
                    let action = i["rdf:value"].as_object().unwrap()["@id"].as_str().unwrap();
                    if !valid_odrl_actions.contains(&action.replace("odrl:", "").as_str()) {
                        println!("Invalid action: {}", action);
                        return false;
                    }
                } else {
                    println!("No @id key found");
                    return false;
                }

                // Validate optional refinement key
                if i.get("refinement").is_some() {
                    if i["refinement"].is_array() {
                        // must be a constraint
                        for j in i["refinement"].as_array().unwrap() {
                            if !validate_constraint(&j) {
                                println!("Invalid constraint");
                                return false;
                            }
                        }
                    } else if i["refinement"].is_object() {
                        // must be a logical constraint
                        if !validate_logical_constraint(&i["refinement"]) {
                            println!("Invalid logical constraint: {:?}", i["refinement"]);
                            return false;
                        }
                    } else {
                        println!("Invalid refinement: {:?}", i["refinement"]);
                        return false;
                    }
                }
            }
        }
    } else {
        println!("Invalid action: {:?}", v);
        return false;
    }

    true
}

impl Validator {
    pub fn new() -> Validator {
        Validator {}
    }

    pub fn validate(&self, json_data: &str) -> bool {
        println!("Validating the ODRL policy");

        let v: Value = serde_json::from_str(json_data).unwrap();

        for i in v.as_array().unwrap() {
            if i.get("@context").is_none() {
                println!("No @context key found");
                return false;
            }

            if i.get("@type").is_none() && i.get("policy").is_none() {
                println!("No @type key found");
                return false;
            }

            if !validate_context(&i["@context"]) {
                println!("Invalid context");
                return false;
            }

            // Validate Constraints
            if &i["@type"] == "Constraint" {
                if !validate_constraint(&i) {
                    println!("Invalid constraint");
                    return false;
                }
            }

            // Validate Policies
            if &i["@type"] == "Set" {
                if !validate_set(&i) {
                    println!("Invalid set");
                    return false;
                }
            }
            if &i["@type"] == "Offer" {
                if !validate_offer(&i) {
                    println!("Invalid offer");
                    return false;
                }
            }
            if &i["@type"] == "Agreement" {
                if !validate_agreement(&i) {
                    println!("Invalid agreement");
                    return false;
                }
            }

            // Validate policies with later known type -> Case if "policy" key is present
            if i.get("policy").is_some() {
                if !validate_policy(&i["policy"]) {
                    println!("Invalid policy");
                    return false;
                }
            }
        }

        println!("\n*** Valid ODRL json data ***\n");

        true
    }
}
