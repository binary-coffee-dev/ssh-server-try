use crate::view::actions::Action;
use crate::view::api_client::get_posts;
use crate::view::view_details::ViewDetails;
use crate::view::view_text::ViewText;
use crate::view::view_trait::ViewTrait;
use std::thread;
use tokio::runtime::Runtime;

#[derive(Clone)]
pub struct ViewArticle {
    pub details: ViewDetails,
}
impl ViewArticle {
    pub fn new(row: u32, col: u32, w: u32, h: u32) -> Self {
        ViewArticle {
            details: ViewDetails {
                width: w,
                height: h,
                row,
                col,
                focus: false,
                can_focus: false,
            },
        }
    }
}

impl ViewTrait for ViewArticle {
    fn draw(&mut self, _screen: &mut Vec<String>, _parent_details: Option<ViewDetails>) {
    }

    fn event(&mut self, _action: &Action) {
    }

    fn cursor_position(&self, parent_details: Option<ViewDetails>) -> Option<(u32, u32)> {
        None
    }

    fn redimension(&mut self, width: u32, height: u32) {
        self.details.width = width;
        self.details.height = height;
    }
}
