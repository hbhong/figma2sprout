use std::sync::Arc;
use iced::{Element, Font};
use iced::widget::{button, Button, Column};
use iced_widget::{horizontal_space, row, text, Component};
use figma_schema::{File as FigmaFile, Node as FigmaNode, NodeType as FigmaNodeType};
use crate::Message;


#[derive(Debug, Clone, Eq, PartialEq)]
pub enum NodeMessage {
    Toggle,
    Select,
}
#[derive(Debug, Clone)]
pub struct TreeNode {
    pub name: String,
    pub node_type: NodeType,
    pub is_expanded: bool,
    pub children: Vec<TreeNode>,
    pub id: String,
    pub id_paths: String,
    pub node_paths: String,
}
#[derive(Debug, Clone)]
pub enum NodeType {
    Unknown,
    Canvas,
    Instance,
    Frame,
    Text,
}
impl From<FigmaNodeType> for NodeType {
    fn from(value: FigmaNodeType) -> Self {
        match value {
            FigmaNodeType::Canvas => NodeType::Canvas,
            FigmaNodeType::Instance => NodeType::Instance,
            FigmaNodeType::Frame => NodeType::Frame,
            FigmaNodeType::Text => NodeType::Text,
            _ => NodeType::Unknown,
        }
    }
}
impl TreeNode {
    pub fn new(name: String, node_type: NodeType, parent_path: String, id: String, node_paths: String) -> Self {
        let id_paths = {
            if parent_path == "" {
                id.clone()
            } else {
                format!("{}|{}", parent_path, id)
            }
        };
        TreeNode {
            id: id.clone(),
            name,
            node_type: node_type,
            is_expanded: false,
            children: vec![],
            id_paths,
            node_paths,
        }
    }
    pub fn find_child(&mut self, path: String) -> Option<&mut TreeNode> {
        if self.id_paths == path {
            return Some(self);
        }
        if path.starts_with(self.id_paths.as_str()) {
            let remain_path = &path[self.id_paths.len() + 1..];
            let paths: Vec<&str> = remain_path.split('|').collect();
            if let Some(id) = paths.first() {
                let child_id = id.to_string();
                for c in self.children.iter_mut() {
                    if c.id == child_id {
                        return c.find_child(path);
                    }
                }
            }
        }
        None
    }
    fn icon<'a, Message>(codepoint: char) -> Element<'a, Message> {
        const ICON_FONT: Font = Font::with_name("my_fonts");
        text(codepoint).font(ICON_FONT).into()
    }
    pub fn view(&self) -> Element<Message> {
        // expand/collapse button
        let mut column = Column::new();
        let expand_text = Self::icon(if self.is_expanded { '\u{E803}' } else { '\u{E802}' });
        let expand_button = button(expand_text).style(button::text).width(22)
            .on_press(Message::TreeNode(self.id_paths.clone(), NodeMessage::Toggle));
        // label
        let label = button(self.name.as_str()).style(button::text)
            .on_press(Message::TreeNode(self.id_paths.clone(), NodeMessage::Select));
        let path_depth = self.id_paths.split('|').collect::<Vec<_>>().len();


        let left_padding = horizontal_space().width((path_depth - 1) as u16 * 10);
        column = column.push(row!(left_padding,expand_button,label));

        if self.is_expanded {
            for child in &self.children {
                column = column.push(child.view());
            }
        }

        column.into()
    }
    pub fn update(&mut self, msg: NodeMessage) {
        match msg {
            NodeMessage::Toggle => {
                self.is_expanded = !self.is_expanded;
            }
            NodeMessage::Select => {}
        }
    }
}

pub async fn parse_file_to_tree(file: Arc<FigmaFile>) -> Result<Vec<TreeNode>, String> {
    let mut result = Vec::new();
    let document_id = file.document.id.clone();
    if let Some(pages) = &file.document.children {
        for page in pages {
            if let Ok(page) = parse_node(page, "".to_string(), document_id.clone()) {
                result.push(page);
            }
        }
    }
    Ok(result)
}

fn parse_node(node: &FigmaNode, parent_path: String, parent_node_paths: String) -> Result<TreeNode, String> {
    let name = node.name.clone();
    let node_type = node.r#type.into();
    let id = node.id.clone();
    let node_paths = format!("{}|{}", parent_node_paths, id);
    let mut tree_node = TreeNode::new(name, node_type, parent_path, id, node_paths);
    if let Some(children) = &node.children {
        for child in children {
            if let Ok(child) = parse_node(child, tree_node.id_paths.clone(), tree_node.node_paths.clone()) {
                tree_node.children.push(child);
            }
        }
    }

    Ok(tree_node)
}