use crate::view::actions::Action;
use crate::view::api_client::get_post_by_name;
use crate::view::view_details::ViewDetails;
use crate::view::view_footer::ViewFooter;
use crate::view::view_text::{TextFormat, ViewText};
use crate::view::view_trait::{EventResult, Page, ViewTrait, ViewType};
use std::thread;
use tokio::runtime::Runtime;

#[derive(Clone)]
pub struct ViewArticle {
    pub details: ViewDetails,
    pub items: Vec<Box<dyn ViewTrait>>,
}
impl ViewArticle {
    pub fn new(row: u32, col: u32, w: u32, h: u32, name: String) -> Self {
        // get article from api
        let handle = thread::spawn(move || {
            let rt = Runtime::new().unwrap();
            rt.block_on(get_post_by_name(&name))
        });
        let result = handle.join().unwrap().unwrap();
        let text = result["data"]["postByName"]["data"]["attributes"]["body"]
            .as_str()
            .unwrap()
            .to_string();

        // let text = "somedasf asdf adsf asdf".to_string();
        ViewArticle {
            details: ViewDetails {
                width: w,
                height: h,
                row,
                col,
                focus: false,
                can_focus: false,
            },
            items: vec![
                Box::new(ViewText::new(
                    TextFormat::Markdown(text),
                    row,
                    col,
                    w,
                    h - 1,
                )),
                Box::new(ViewFooter::new(
                    h - 1,
                    w,
                    vec![
                        Box::new(ViewText::new(
                            TextFormat::PlainText("↑ (k)".to_string()),
                            0,
                            0,
                            5,
                            1,
                        )),
                        Box::new(ViewText::new(
                            TextFormat::PlainText("↓ (j)".to_string()),
                            0,
                            0,
                            5,
                            1,
                        )),
                        Box::new(ViewText::new(
                            TextFormat::PlainText("Quit (C+d)".to_string()),
                            0,
                            0,
                            10,
                            1,
                        )),
                        Box::new(ViewText::new(
                            TextFormat::PlainText("Back (C+c)".to_string()),
                            0,
                            0,
                            10,
                            1,
                        )),
                    ],
                )),
            ],
        }
    }
}

impl ViewTrait for ViewArticle {
    fn draw(&mut self, screen: &mut Vec<String>, _parent_details: Option<ViewDetails>) {
        for child in &mut self.items {
            child.draw(screen, Some(self.details.clone()));
        }
    }

    fn event(&mut self, action: &Action) -> Option<EventResult> {
        match action {
            Action::Esc | Action::Sigint => Some(EventResult::ChangePage(Page::List)),
            _ => {
                for child in &mut self.items {
                    if let Some(result) = child.event(action) {
                        return Some(result);
                    }
                }
                None
            }
        }
    }

    fn cursor_position(&self, _parent_details: Option<ViewDetails>) -> Option<(u32, u32)> {
        Some((self.details.height, self.details.width))
    }

    fn redimension(&mut self, width: u32, height: u32) {
        self.details.width = width;
        self.details.height = height;

        for child in &mut self.items {
            match child.view_type() {
                ViewType::Text => child.redimension(width, height - 1),
                _ => child.redimension(width, height),
            }
        }
    }
}
