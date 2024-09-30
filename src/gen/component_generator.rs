use std::rc::Rc;
use figma_schema::{Node, NodeType};
pub trait Component {}

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
        Generators {
            generators: Vec::new(),
        }
    }
    pub fn register_generators(&mut self) {}
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