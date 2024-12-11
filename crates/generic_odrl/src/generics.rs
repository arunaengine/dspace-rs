// Asset, Party, Action

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, PartialOrd, Ord)]
#[serde(untagged)]
pub enum StringOrX<X> {
    String(String),
    VecX(Vec<X>),
    VecString(Vec<String>),
    X(X),
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_serde_string_or_x() {
        use super::StringOrX;
        use serde_json;

        let string = StringOrX::String("string".to_string());
        let x = StringOrX::X(1);

        let string_json = serde_json::to_string(&string).unwrap();
        let x_json = serde_json::to_string(&x).unwrap();

        assert_eq!(string_json, r#""string""#);
        assert_eq!(x_json, r#"1"#);

        let string_de: StringOrX<i32> = serde_json::from_str(&string_json).unwrap();
        let x_de: StringOrX<i32> = serde_json::from_str(&x_json).unwrap();

        assert_eq!(string_de, string);
        assert_eq!(x_de, x);
    }
}
