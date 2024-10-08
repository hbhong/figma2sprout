use super::Node;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct File {
    pub document: Node,
    pub name: String,
    pub version: String,
}
