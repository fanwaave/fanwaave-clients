#![forbid(unsafe_code)]

pub use fanwaave_interfaces::Health;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ResourceEnvelope {
    pub id: String,
    pub revision: String,
    #[serde(default)]
    pub payload: Value,
}

impl ResourceEnvelope {
    pub const RESOURCE: &'static str = "NotificationDispatch";
}

