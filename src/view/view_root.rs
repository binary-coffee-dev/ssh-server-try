use crate::view::view_details::ViewDetails;
use crate::view::view_list::ViewList;
use crate::view::view_trait::ViewTrait;
use crate::view::view_text::ViewText;

#[derive(Clone)]
pub struct ViewRoot {
    pub details: ViewDetails,
    pub children: Vec<Box<dyn ViewTrait>>,
}

impl ViewRoot {
    pub fn new() -> Self {
        ViewRoot {
            children: vec![
                Box::new(ViewList::new(0, 0, 80, 24)),
            ],
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
}

impl ViewTrait for ViewRoot {
    fn draw(&self, screen: &mut Vec<String>, parent_details: Option<ViewDetails>) {
        for child in &self.children {
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
}
