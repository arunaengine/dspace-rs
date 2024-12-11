// Asset, Party, Action

pub enum StringOrX {
    String(String),
    X(serde_json::Value),
}