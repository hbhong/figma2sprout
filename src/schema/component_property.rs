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
impl Into<Option<String>> for &ComponentProperty {
    fn into(self) -> Option<String> {
        match self.value_type {
            ComponentPropertyType::Variant | ComponentPropertyType::Text => {
                if let Value::String(value) = &self.value {
                    Some(value.clone())
                } else {
                    None
                }
            },
            _ => None,
        }
    }
}
impl Into<Option<bool>> for &ComponentProperty {
    fn into(self) -> Option<bool> {
        match self.value_type {
            ComponentPropertyType::Boolean => {
                if let Value::Bool(value) = &self.value {
                    Some(value.clone())
                } else {
                    None
                }
            },
            _ => None,
        }
    }
}
