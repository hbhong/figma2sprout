use crate::{
    gen::components::{alert::AlertGenerator, checkbox::CheckboxGenerator},
    schema::{Node, NodeType},
};
use std::{any::Any, rc::Rc};

const DOC_ROOT_LINK: &str = "https://rd-sprout.qliktech.com/sprout/?path=/docs";
pub trait Component: Any {
    fn name(&self) -> String;

    fn link(&self) -> String;
    fn doc_link(&self) -> String {
        format!("{}/{}", DOC_ROOT_LINK, self.link())
    }
    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;
}
#[macro_export]
macro_rules! impl_component {
    ($component:ident, $name:expr, $link:expr) => {
        impl Component for $component {
            fn name(&self) -> String {
                $name.to_string()
            }

            fn link(&self) -> String {
                $link.to_string()
            }

            fn as_any(&self) -> &dyn std::any::Any {
                self
            }

            fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                self
            }
        }
    };
}

pub trait ComponentGenerator {
    fn is_instance_type(&self, node: &Node) -> bool {
        node.r#type == NodeType::Instance
    }
    fn can_gen_component(&self, node: &Node) -> bool;

    fn gen_component(&self, node: &Node) -> Box<dyn Component>;
}

pub struct Generators {
    generators: Vec<Box<dyn ComponentGenerator>>,
}
impl Generators {
    pub fn new() -> Self {
        Generators { generators: Vec::new() }
    }
    pub fn register_generators(&mut self) {
        self.generators.push(Box::new(AlertGenerator::new()));
        self.generators.push(Box::new(CheckboxGenerator::new()));
    }

    pub fn gen_component(&self, node: &Node) -> Option<Box<dyn Component>> {
        for gen in &self.generators {
            if gen.can_gen_component(node) {
                let component = gen.gen_component(node);
                return Some(component);
            }
        }
        None
    }
}

#[macro_export]
macro_rules! get_value_from_properties {
    ($node:expr, $prefix:expr) => {{
        if let Some(properties) = &$node.component_properties {
            let value = properties
                .iter()
                .find(|p| p.0.starts_with($prefix))
                .map(|p| p.1.clone());
            if let Some(component_prop) = value {
                component_prop.into()
            } else {
                None
            }
        } else {
            None
        }
    }};
}
