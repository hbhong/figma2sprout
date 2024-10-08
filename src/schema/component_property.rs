use serde::{Deserialize, Serialize};
use serde_json::Value;
#[derive(Debug, Deserialize, Serialize, PartialEq, Copy, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ComponentPropertyType {
    Boolean,
    Variant,
    Text,
    #[serde(rename = "INSTANCE_SWAP")]
    InstanceSwap,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ComponentProperty {
    value: Value,
    #[serde(rename = "type")]
    value_type: ComponentPropertyType,
}