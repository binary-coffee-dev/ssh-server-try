use crate::view::actions::Action;
use crate::view::view_details::ViewDetails;
use crate::view::view_list_item::ViewListItem;
use crate::view::view_trait::{EventResult, Page, ViewTrait};

#[derive(Clone)]
pub struct ViewArticle {
    pub details: ViewDetails,
    pub name: String,
}
impl ViewArticle {
    pub fn new(row: u32, col: u32, w: u32, h: u32, name: String) -> Self {
        ViewArticle {
            details: ViewDetails {
                width: w,
                height: h,
                row,
                col,
                focus: false,
                can_focus: false,
            },
            name,
        }
    }
}

impl ViewTrait for ViewArticle {
    fn draw(&mut self, _screen: &mut Vec<String>, _parent_details: Option<ViewDetails>) {
        ViewListItem::new(self.name.clone(), 0, 0, self.name.clone())
            .draw(_screen, Some(self.details.clone()));
    }

    fn event(&mut self, action: &Action) -> Option<EventResult> {
        match action {
            Action::Esc => {
                return Some(EventResult::ChangePage(Page::List));
            }
            _ => return None,
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
