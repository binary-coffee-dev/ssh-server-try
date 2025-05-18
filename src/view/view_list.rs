use crate::view::actions::Action;
use crate::view::view_details::ViewDetails;
use crate::view::view_text::ViewText;
use crate::view::view_trait::ViewTrait;

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
            items: vec![
                Box::new(ViewText::new("BinaryCoffee".to_string(), 0, 0)),
                Box::new(ViewText::new(
                    "This is a test asdf asdf asdf asdf asdf asdfasd fasd fsa".to_string(),
                    1,
                    0,
                )),
                Box::new(ViewText::new("Another line".to_string(), 2, 0)),
            ],
        }
    }
}

impl ViewTrait for ViewList {
    fn draw(&self, screen: &mut Vec<String>, _parent_details: Option<ViewDetails>) {
        for child in &self.items {
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
                    if self.selected_index < self.items.len() - 1 {
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
