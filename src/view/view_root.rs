use crate::view::actions::Action;
use crate::view::view_article::ViewArticle;
use crate::view::view_details::ViewDetails;
use crate::view::view_list::ViewList;
use crate::view::view_trait::{EventResult, Page, PostOperation, ViewTrait};

#[derive(Clone)]
pub struct ViewRoot {
    pub details: ViewDetails,
    pub current_view: Box<dyn ViewTrait>,
}

impl ViewRoot {
    pub fn new() -> Self {
        ViewRoot {
            current_view: Box::new(ViewList::new(0, 0, 80, 24)),
            details: ViewDetails {
                width: 80,
                height: 24,
                row: 0,
                col: 0,
                focus: false,
                can_focus: false,
            },
        }
    }

    pub fn change_page(&mut self, page: Page) {
        match page {
            Page::Article(name) => {
                self.current_view = Box::new(ViewArticle::new(
                    self.details.row,
                    self.details.col,
                    self.details.width,
                    self.details.height,
                    name.clone(),
                ));
            }
            Page::List => {
                self.current_view = Box::new(ViewList::new(0, 0, 80, 24));
            }
        }
    }
}

impl ViewTrait for ViewRoot {
    fn draw(&mut self, screen: &mut Vec<String>, _parent_details: Option<ViewDetails>) {
        self.current_view.draw(screen, Some(self.details.clone()));
    }

    fn event(&mut self, action: &Action) -> Option<EventResult> {
        match self.current_view.event(action) {
            Some(EventResult::ChangePage(page)) => {
                self.change_page(page);
            }
            _ => {}
        }

        None
    }

    fn cursor_position(&self, _parent_details: Option<ViewDetails>) -> Option<(u32, u32)> {
        self.current_view
            .cursor_position(Some(self.details.clone()))
    }

    fn post_operations(&mut self, _parent_details: Option<ViewDetails>) -> Vec<PostOperation> {
        self.current_view
            .post_operations(Some(self.details.clone()))
    }

    fn redimension(&mut self, width: u32, height: u32) {
        self.details.width = width;
        self.details.height = height;

        self.current_view.redimension(width, height);
    }
}
