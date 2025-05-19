use crate::view::actions::Action;
use crate::view::view_details::ViewDetails;
use crate::view::view_list_item::ViewListItem;
use crate::view::view_text::ViewText;
use crate::view::view_trait::{EventResult, Page, ViewTrait};

#[derive(Clone)]
pub struct ViewArticle {
    pub details: ViewDetails,
    pub name: String,
    pub items: Vec<Box<dyn ViewTrait>>,
}
impl ViewArticle {
    pub fn new(row: u32, col: u32, w: u32, h: u32, name: String) -> Self {
        let text = "somedasf asdf adsf asdf".to_string();
        ViewArticle {
            details: ViewDetails {
                width: w,
                height: h,
                row,
                col,
                focus: false,
                can_focus: false,
            },
            items: vec![Box::new(ViewText::new(text, row, col, w, h))],
            name,
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
            Action::Esc => Some(EventResult::ChangePage(Page::List)),
            _ => None,
        }
    }

    fn cursor_position(&self, _parent_details: Option<ViewDetails>) -> Option<(u32, u32)> {
        None
    }

    fn redimension(&mut self, width: u32, height: u32) {
        self.details.width = width;
        self.details.height = height;
    }
}
