use std::sync::Arc;
use crate::schema::{File, Node};
use serde_json::from_str;
use crate::ui::tree::TreeNode;

pub fn convert_json_to_figma(json: String) -> Result<File, String> {
    let file = from_str(&json)
        .map_err(|e| e.to_string());
    file
}
pub fn find_figma_node(file: &Arc<File>, paths: String) -> Option<&Node> {
    let path_list: Vec<&str> = paths.split("|").collect();
    if path_list[0] == file.document.id {
        let new_path: Vec<&str> = path_list.into_iter().skip(1).collect();
        if let Some(pages) = &file.document.children {
            for page in pages {
                if let Some(node) = find_node(page, new_path.clone()) {
                    return Some(node);
                }
            }
        }
    }

    None
}
fn find_node<'a>(node: &'a Node, paths: Vec<&str>) -> Option<&'a Node> {
    let path_len = paths.len();
    if path_len > 0 {
        if node.id == paths[0] {
            if path_len == 1 {
                return Some(node);
            } else {
                if let Some(children) = &node.children {
                    let new_paths: Vec<&str> = paths.into_iter().skip(1).collect();
                    for child in children {
                        if let Some(found) = find_node(child, new_paths.clone()) {
                            return Some(found);
                        }
                    }
                }
            }
        }
    }
    None
}