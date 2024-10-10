use std::any::Any;

use crate::gen::component_generator::Component;
use crate::impl_component;

#[derive(Default, Debug, Clone)]
pub enum CheckedType {
    True,
    #[default]
    False,
    Indeterminate,
}
impl From<&str> for CheckedType {
    fn from(s: &str) -> Self {
        match s {
            "true" => CheckedType::True,
            "false" => CheckedType::False,
            "indeterminate" => CheckedType::Indeterminate,
            _ => CheckedType::False,
        }
    }
}
#[derive(Default, Debug, Clone)]
pub struct ComponentCheckbox {
    pub label: Option<String>,
    pub has_label: bool,
    pub disabled: bool,
    pub checked: CheckedType,
    pub dismissable: bool,
}
impl ComponentCheckbox {
    pub fn new() -> Self {
        ComponentCheckbox {
            label: None,
            has_label: false,
            disabled: false,
            checked: CheckedType::True,
            dismissable: false,
        }
    }
}

impl_component!(ComponentCheckbox, "Checkbox", "/mui-components-checkbox--docs");