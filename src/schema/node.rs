use crate::schema::ComponentProperty;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize, PartialEq, Copy, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum NodeType {
    Document,
    Canvas,
    Frame,
    Group,
    Vector,
    BooleanOperation,
    Star,
    Line,
    Ellipse,
    RegularPolygon,
    Rectangle,
    Text,
    Slice,
    Component,
    ComponentSet,
    Instance,
    Sticky,
    ShapeWithText,
    Connector,
    Section,
}
/// [Figma documentation](https://www.figma.com/developers/api#node-types)
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    /// A string uniquely identifying this node within the document.
    pub id: String,
    /// The name given to the node by the user in the tool.
    pub name: String,
    /// Whether or not the node is visible on the canvas.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible: Option<bool>,
    /// The type of the node
    pub r#type: NodeType,
    /// An array of nodes that are direct children of this node
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_properties: Option<HashMap<String, ComponentProperty>>,
    pub characters: Option<String>,
}
