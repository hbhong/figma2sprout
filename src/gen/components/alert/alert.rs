use std::any::Any;

use crate::{gen::component_generator::Component, impl_component};

#[derive(Default, Debug, Clone)]
pub struct ComponentAlert {
    pub severity: Option<String>,
    pub message: Option<String>,
    pub has_title: bool,
    pub has_actions: bool,
    pub dismissable: bool,
}
impl ComponentAlert {
    pub fn new() -> Self {
        ComponentAlert {
            severity: None,
            message: None,
            has_title: false,
            has_actions: false,
            dismissable: false,
        }
    }
}

impl_component!(ComponentAlert, "Alert", "/mui-components-alerts-beta--docs");
