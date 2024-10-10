use crate::{
    gen::{
        component_generator::{Component, ComponentGenerator},
        components::alert::ComponentAlert,
    },
    get_value_from_properties,
    schema::{ComponentProperty, Node},
};
use serde_json::Value;

pub struct AlertGenerator {}

impl AlertGenerator {
    pub fn new() -> Self {
        AlertGenerator {}
    }
}
impl ComponentGenerator for AlertGenerator {
    fn can_gen_component(&self, node: &Node) -> bool {
        self.is_instance_type(node) && node.name == "Alert.Inline"
    }

    fn gen_component(&self, node: &Node) -> Box<dyn Component> {
        let mut alert = ComponentAlert::new();

        alert.message = get_value_from_properties!(node, "Message text");
        alert.severity = get_value_from_properties!(node, "Severity");
        alert.has_title = get_value_from_properties!(node, "Title").unwrap_or(false);
        alert.dismissable = get_value_from_properties!(node, "Dismissable").unwrap_or(false);
        alert.has_actions = get_value_from_properties!(node, "has Actions").unwrap_or(false);
        Box::new(alert)
    }
}
