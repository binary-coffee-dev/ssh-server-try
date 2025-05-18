use crate::view::view_details::ViewDetails;
use crate::view::view_text::ViewText;
use crate::view::view_trait::ViewTrait;

#[derive(Clone)]
pub struct ViewList {
    pub details: ViewDetails,
    pub children: Vec<Box<dyn ViewTrait>>,
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
            children: vec![
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
        for child in &self.children {
            child.draw(screen, Some(self.details.clone()));
        }
    }

    fn redimension(&mut self, width: u32, height: u32) {
        self.details.width = width;
        self.details.height = height;
    }
}
