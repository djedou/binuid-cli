use binuid_shared_wasm::{serde_json::json, serde::{self, Serialize, Deserialize}};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "self::serde")]
pub struct Metadata {
    pub path: Vec<String>,
    pub component_struct: String,
    pub component_function: String
}