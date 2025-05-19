use crate::view::actions::Action;
use crate::view::api_client::get_posts;
use crate::view::view_details::ViewDetails;
use crate::view::view_text::ViewText;
use crate::view::view_trait::ViewTrait;
use std::thread;
use tokio::runtime::Runtime;

#[derive(Clone)]
pub struct ViewList {
    pub details: ViewDetails,
    pub items: Vec<Box<dyn ViewTrait>>,
    pub selected_index: usize,
}
impl ViewList {
    pub fn new(row: u32, col: u32, w: u32, h: u32) -> Self {
        ViewList {
            details: ViewDetails {
                width: w,
                height: h,
                row,
                col,
                focus: false,
                can_focus: true,
            },
            selected_index: 0,
            items: vec![],
        }
    }
}

impl ViewTrait for ViewList {
    fn draw(&mut self, screen: &mut Vec<String>, _parent_details: Option<ViewDetails>) {
        if self.items.is_empty() {
            // load the list of posts from the API
            let handle = thread::spawn(|| {
                let rt = Runtime::new().unwrap();
                rt.block_on(get_posts())
            });

            let result = handle.join().unwrap().unwrap();
            let mut count = 0;
            for post in result["data"]["posts"]["data"].as_array().unwrap() {
                self.items.push(Box::new(ViewText::new(
                    post["attributes"]["title"].as_str().unwrap().to_string(),
                    count,
                    0,
                )));
                println!("{:?}", post["attributes"]["title"].as_str());
                count += 1;
            }
        }

        for child in &mut self.items {
            child.draw(screen, Some(self.details.clone()));
        }
    }

    fn event(&mut self, action: &Action) {
        if self.details.focus {
            match action {
                Action::Up => {
                    if (self.selected_index as u32) > 0 {
                        self.selected_index = self.selected_index - 1;
                    }
                }
                Action::Down => {
                    if self.items.len() > 0 && self.selected_index < self.items.len() - 1 {
                        self.selected_index = self.selected_index + 1;
                    }
                }
                _ => {}
            }
        }
    }

    fn cursor_position(&self, parent_details: Option<ViewDetails>) -> Option<(u32, u32)> {
        let parent_details = parent_details.unwrap();
        Some((
            self.selected_index as u32 + parent_details.row + 1,
            parent_details.col,
        ))
    }

    fn redimension(&mut self, width: u32, height: u32) {
        self.details.width = width;
        self.details.height = height;
    }
}
