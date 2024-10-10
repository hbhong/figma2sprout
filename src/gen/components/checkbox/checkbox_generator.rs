use crate::{
    gen::{
        component_generator::{Component, ComponentGenerator},
        components::checkbox::ComponentCheckbox,
    },
    get_value_from_properties,
    schema::{ComponentProperty, Node, NodeType},
};
use serde_json::Value;

pub struct CheckboxGenerator {}

impl CheckboxGenerator {
    pub fn new() -> Self {
        CheckboxGenerator {}
    }
    pub fn get_name_from_children(
        node: &Node,
        node_type: NodeType,
        cur_depth: usize,
        max_depth: usize,
    ) -> Option<String> {
        if let Some(children) = &node.children {
            let cur_depth = cur_depth + 1;
            if cur_depth <= max_depth {
                for child in children {
                    if child.r#type == node_type {
                        return Some(child.name.clone());
                    }
                    if let Some(value) =
                        Self::get_name_from_children(child, node_type, cur_depth, max_depth)
                    {
                        return Some(value);
                    }
                }
            }
        }
        None
    }
}
impl ComponentGenerator for CheckboxGenerator {
    fn can_gen_component(&self, node: &Node) -> bool {
        self.is_instance_type(node) && node.name == "Checkbox"
    }

    fn gen_component(&self, node: &Node) -> Box<dyn Component> {
        let mut checkbox = ComponentCheckbox::new();

        checkbox.has_label = get_value_from_properties!(node, "Label").unwrap_or(false);
        checkbox.disabled = get_value_from_properties!(node, "State")
            .map(|val: String| val == "Disabled")
            .unwrap_or(false);
        checkbox.dismissable = false; // cannot get it from Figma
        checkbox.checked = get_value_from_properties!(node, "Checked")
            .map(|val: String| val.clone())
            .unwrap()
            .as_str()
            .into();
        checkbox.label = Self::get_name_from_children(node, NodeType::Text, 0, 1);

        Box::new(checkbox)
    }
}
