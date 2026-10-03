use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct Request {
    #[serde(rename = "type")]
    pub request_type: String,

    pub header: String,

    #[serde(default)]
    pub data: Value,
}