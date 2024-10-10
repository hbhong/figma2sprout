mod figma_api;
mod gen;
mod schema;
mod ui;

use crate::gen::node_util::{convert_json_to_figma, find_figma_node};
use crate::schema::File as FigmaFile;
use iced::advanced::Widget;
use iced::application::View;
use iced::theme;
use iced::widget::{column, container, text_input, Button};
use iced::{Element, Font, Pixels, Task, Theme};
use iced_widget::{button, scrollable};
use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use std::sync::Arc;
use ui::tree::{parse_file_to_tree, NodeMessage, TreeNode};

fn save_to_file(data: &str, file_path: &str) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(file_path)?;

    file.write_all(data.as_bytes())?;
    Ok(())
}
async fn fetch_save_figma_file(
    file_key: &str,
    access_token: &str,
    file_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let response = figma_api::fetch_figma_file(file_key, access_token).await?;
    save_to_file(&response, file_path)
}
fn read_json_file(file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let mut reader = std::io::BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    Ok(contents)
}

#[derive(Debug, Clone)]
pub enum Message {
    TokenChanged(String),
    FileIDChanged(String),
    TreeNode(String, NodeMessage),
    ParseJson,
    FetchJson,
    JsonFetched(Result<String, String>),
    JsonIsParsed(Result<Vec<TreeNode>, String>),
}

pub struct FigmaClient {
    pub token: String,
    pub file_id: String,
    pub root_node: Option<Vec<TreeNode>>,
    figma_file: Option<Arc<FigmaFile>>,
    fetching: bool,
}

impl FigmaClient {
    pub fn new() -> Self {
        FigmaClient {
            // token: String::new(),
            token: "".to_string(),
            file_id: String::new(),
            root_node: Some(vec![]),
            figma_file: None,
            fetching: false,
        }
    }
    fn update(&mut self, event: Message) -> Task<Message> {
        match event {
            Message::TokenChanged(token) => {
                self.token = token;
                Task::none()
            },
            Message::FileIDChanged(file_id) => {
                self.file_id = file_id;
                Task::none()
            },
            Message::TreeNode(path, msg) => {
                if let Some(root) = &mut self.root_node {
                    for node in root.iter_mut() {
                        if let Some(found_child) = node.find_child(path.clone()) {
                            if msg == NodeMessage::Select {
                                if let Some(figma_node) = find_figma_node(
                                    &self.figma_file.clone().unwrap().clone(),
                                    found_child.node_paths.clone(),
                                ) {
                                    println!("{}", figma_node.name);
                                }
                            }
                            found_child.update(msg);

                            break;
                        }
                    }
                }
                Task::none()
            },
            Message::FetchJson => {
                if !self.fetching {
                    self.fetching = true;
                    let token = self.token.clone();
                    let file_id = self.file_id.clone();
                    Task::perform(
                        async move {
                            match fetch_save_figma_file(&file_id, &token, "demo.json").await {
                                Ok(()) => Ok("File successfully saved".to_string()),
                                Err(e) => Err(e.to_string()),
                            }
                        },
                        Message::JsonFetched,
                    )
                } else {
                    Task::none()
                }
            },
            Message::ParseJson => {
                if let Ok(json) = read_json_file("demo.json") {
                    match convert_json_to_figma(json) {
                        Ok(figma_file) => {
                            let figma_file = Arc::new(figma_file);
                            self.figma_file = Some(figma_file.clone());
                            let result = Task::perform(
                                parse_file_to_tree(figma_file),
                                Message::JsonIsParsed,
                            );
                            return result;
                        },
                        Err(e) => {
                            println!("{}", e);
                        },
                    }
                }
                Task::none()
            },
            Message::JsonIsParsed(result) => {
                if let Ok(nodes) = result {
                    self.root_node = Some(nodes);
                }
                Task::none()
            },
            _ => Task::none(),
        }
    }

    fn view(&self) -> Element<Message> {
        let token_input = text_input("Personal Access Token", &self.token)
            .on_input(Message::TokenChanged)
            .icon(text_input::Icon {
                font: Font::with_name("my_fonts"),
                code_point: '\u{E800}',
                size: Some(Pixels(20.0)),
                spacing: 5.0,
                side: text_input::Side::Right,
            })
            .padding(5)
            .size(20);
        let file_id_input = text_input("File id", &self.file_id)
            .on_input(Message::FileIDChanged)
            .padding(5.)
            .size(20);
        let mut column = column!(token_input, file_id_input).spacing(10);
        let parse_button = button(if self.fetching { "Fetching..." } else { "Parse" })
            .on_press(Message::ParseJson)
            .style(if self.fetching { button::secondary } else { button::primary });
        let fetch_button: Button<'_, Message> =
            button(if self.fetching { "Fetching..." } else { "Fetch" })
                .on_press(Message::FetchJson)
                .style(if self.fetching { button::secondary } else { button::primary });
        column = column.push(fetch_button).push(parse_button);
        if let Some(root_node) = &self.root_node {
            for node in root_node {
                let tree_container = container(scrollable(node.view()));
                column = column.push(tree_container);
            }
        };

        container(column).padding(10).into()
    }
}
impl Default for FigmaClient {
    fn default() -> Self {
        FigmaClient::new()
    }
}
fn main() {
    let file_key = "";
    let access_token = "";

    let font = include_bytes!("../fonts/my_fonts.ttf");
    iced::application("Figma2Sprout", FigmaClient::update, FigmaClient::view)
        .font(font)
        .centered()
        .run();
}
