use serde_json::Value;

pub fn remove_prefixes_from_value(mut value: Value) -> Value {
    match &mut value {
        Value::Object(map) => {
            let keys: Vec<String> = map.keys().cloned().collect();
            for key in keys {
                if let Some(mut v) = map.remove(&key) {
                    v = remove_prefixes_from_value(v);
                    let new_key = key
                        .trim_start_matches("edc:")
                        .trim_start_matches("odrl:")
                        .to_string();
                    map.insert(new_key, v);
                }
            }
        }
        Value::Array(arr) => {
            for elem in arr.iter_mut() {
                *elem = remove_prefixes_from_value(elem.take());
            }
        }
        _ => {}
    }
    value
}
