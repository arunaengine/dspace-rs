use serde_json::Value;
use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref DEFAULT_CONTEXT: HashMap<String, Value> = {
        HashMap::from([
            ("@vocab".to_string(), Value::String("https://w3id.org/edc/v0.0.1/ns/".to_string())),
            ("edc".to_string(), Value::String("https://w3id.org/edc/v0.0.1/ns/".to_string())),
            ("dcat".to_string(), Value::String("http://www.w3.org/ns/dcat#".to_string())),
            ("dct".to_string(), Value::String("http://purl.org/dc/terms/".to_string())),
            ("odrl".to_string(), Value::String("http://www.w3.org/ns/odrl/2/".to_string())),
            ("dspace".to_string(), Value::String("https://w3id.org/dspace/v0.8/".to_string())),
        ])
    };
}