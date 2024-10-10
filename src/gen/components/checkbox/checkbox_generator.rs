use crate::{
    gen::{
        component_generator::{Component, ComponentGenerator},
        components::checkbox::ComponentCheckbox,
    },
    get_value_from_properties,
    schema::{Node, NodeType},
};
use crate::gen::node_util::find_node_from_children;

pub struct CheckboxGenerator {}

impl CheckboxGenerator {
    pub fn new() -> Self {
        CheckboxGenerator {}
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
        if let Some(label_node) = find_node_from_children(node, "Label", NodeType::Text, 0, 1) {
            if let Some(label) = &label_node.characters {
                checkbox.label = Some(label.clone());
            }
        }

        Box::new(checkbox)
    }
}
