use crate::view::actions::Action;
use crate::view::api_client::get_posts;
use crate::view::view_details::ViewDetails;
use crate::view::view_footer::ViewFooter;
use crate::view::view_list_item::ViewListItem;
use crate::view::view_logo::ViewLogo;
use crate::view::view_text::{TextFormat, ViewText};
use crate::view::view_trait::{EventResult, Page, PostOperation, ViewTrait, ViewType};
use std::{thread, vec};
use tokio::runtime::Runtime;

const GET_POSTS_RETRIES_TIMES: u32 = 5;

#[derive(Clone)]
pub struct ViewList {
    pub details: ViewDetails,
    pub items: Vec<Box<ViewListItem>>,
    pub children: Vec<Box<dyn ViewTrait>>,
    pub selected_index: usize,
    pub offset: u32,
    current_page: u32,
    pages: u32,
}
impl ViewList {
    pub fn new(row: u32, col: u32, w: u32, h: u32) -> Self {
        ViewList {
            details: ViewDetails {
                width: w,
                height: h,
                row,
                col,
                focus: true,
                can_focus: true,
            },
            children: vec![
                Box::new(ViewLogo::new(0, 0)),
                Box::new(ViewFooter::new(h - 1, w, vec![])),
            ],
            selected_index: 0,
            items: vec![],
            offset: 0,
            current_page: 0,
            pages: 0,
        }
    }

    fn update_indicators(&mut self) {
        let mut footer = Box::new(ViewFooter::new(
            self.details.height - 1,
            self.details.width,
            vec![
                Box::new(ViewText::new(
                    TextFormat::PlainText(
                        format!("Pages {}/{} (←h, l→) |", self.current_page, self.pages)
                            .to_string(),
                    ),
                    0,
                    0,
                    20,
                    1,
                )),
                Box::new(ViewText::new(
                    TextFormat::PlainText("↑ (k) |".to_string()),
                    0,
                    0,
                    7,
                    1,
                )),
                Box::new(ViewText::new(
                    TextFormat::PlainText("↓ (j) |".to_string()),
                    0,
                    0,
                    7,
                    1,
                )),
                Box::new(ViewText::new(
                    TextFormat::PlainText("Quit (C+d) |".to_string()),
                    0,
                    0,
                    12,
                    1,
                )),
                Box::new(ViewText::new(
                    TextFormat::PlainText("Open (enter)".to_string()),
                    0,
                    0,
                    12,
                    1,
                )),
            ],
        ));

        self.children = self
            .children
            .iter()
            .filter(|child| match child.view_type() {
                ViewType::Footer => false,
                _ => true,
            })
            .cloned()
            .collect();

        footer.redimension(self.details.width, self.details.height);
        self.children.push(footer);
    }

    fn get_posts_with_retry(&mut self, page: u32) -> Vec<Box<ViewListItem>> {
        let mut retries = GET_POSTS_RETRIES_TIMES;
        loop {
            match self.get_posts(page) {
                Ok(result) => {
                    return result;
                }
                Err(err) => {
                    retries -= 1;
                    if retries == 0 {
                        eprintln!("Failed to get posts: {}", err);
                        return vec![];
                    }
                }
            }
        }
    }

    fn get_posts(&mut self, page: u32) -> Result<Vec<Box<ViewListItem>>, String> {
        let handle = thread::spawn(move || {
            let rt = Runtime::new().unwrap();
            rt.block_on(get_posts(page))
        });

        let result = handle.join().unwrap().unwrap();
        let mut count = 0;
        let mut items = vec![];

        self.pages = result["data"]["posts"]["meta"]["pagination"]["pageCount"]
            .as_u64()
            .expect("Failed to get page count") as u32;
        self.current_page = result["data"]["posts"]["meta"]["pagination"]["page"]
            .as_u64()
            .expect("Failed to get current page") as u32;
        for post in result["data"]["posts"]["data"].as_array().unwrap() {
            items.push(Box::new(ViewListItem::new(
                post["attributes"]["title"]
                    .as_str()
                    .expect("Failed to get post title")
                    .to_string(),
                count + self.offset,
                0,
                post["attributes"]["name"]
                    .as_str()
                    .expect("Failed to get post name")
                    .to_string(),
            )));
            count += 1;
        }

        Ok(items)
    }
}

impl ViewTrait for ViewList {
    fn draw(&mut self, screen: &mut Vec<String>, _parent_details: Option<ViewDetails>) {
        for child in &mut self.children {
            match child.view_type() {
                ViewType::Logo => {
                    self.offset = child.get_details().height;
                }
                _ => {}
            }
        }

        if self.items.is_empty() {
            self.items = self.get_posts_with_retry(self.current_page);
            self.update_indicators();
        }

        for child in &mut self.children {
            child.draw(screen, Some(self.details.clone()));
        }

        for child in &mut self.items {
            child.draw(screen, Some(self.details.clone()));
        }
    }

    fn redimension(&mut self, width: u32, height: u32) {
        self.details.width = width;
        self.details.height = height;

        for child in &mut self.children {
            child.redimension(width, height);
        }
    }

    fn post_operations(&mut self, parent_details: Option<ViewDetails>) -> Vec<PostOperation> {
        let details_inhered = ViewDetails {
            row: self.details.row + parent_details.as_ref().unwrap().row,
            col: self.details.col + parent_details.as_ref().unwrap().col,
            ..self.details.clone()
        };
        self.items[self.selected_index].post_operations(Some(details_inhered))
    }

    fn event(&mut self, action: &Action) -> Option<EventResult> {
        if self.details.focus {
            return match action {
                Action::Up => {
                    if (self.selected_index as u32) > 0 {
                        self.selected_index = self.selected_index - 1;
                    }
                    None
                }
                Action::Down => {
                    if self.items.len() > 0 && self.selected_index < self.items.len() - 1 {
                        self.selected_index = self.selected_index + 1;
                    }
                    None
                }
                Action::Left => {
                    if self.current_page > 1 {
                        self.items = self.get_posts_with_retry(self.current_page - 1);
                        self.selected_index = 0;
                        self.update_indicators();
                    }
                    None
                }
                Action::Right => {
                    if self.current_page + 1 < self.pages {
                        self.items = self.get_posts_with_retry(self.current_page + 1);
                        self.selected_index = 0;
                        self.update_indicators();
                    }
                    None
                }
                Action::Enter => {
                    let item_value = self.items[self.selected_index].value.clone();
                    return Some(EventResult::ChangePage(Page::Article(item_value)));
                }
                Action::Esc | Action::Sigint => {
                    return Some(EventResult::Quite);
                }
                _ => None,
            };
        }
        None
    }

    fn cursor_position(&self, parent_details: Option<ViewDetails>) -> Option<(u32, u32)> {
        let parent_details = parent_details.unwrap();
        Some((
            self.selected_index as u32 + parent_details.row + 1 + self.offset,
            parent_details.col,
        ))
    }
}
