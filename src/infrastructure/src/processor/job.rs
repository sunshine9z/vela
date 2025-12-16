use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

#[derive(Debug, Deserialize, Serialize)]
pub struct Job {
    pub queue: String,
    pub args: JsonValue,
    pub retry: bool,
    pub class: String,
}
