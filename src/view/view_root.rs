use crate::view::view_details::ViewDetails;
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
                Box::new(ViewText::new("BinaryCoffee".to_string(), 0, 0)),
                Box::new(ViewText::new(
                    "This is a test asdf asdf asdf asdf asdf asdfasd fasd fsa".to_string(),
                    1,
                    0,
                )),
                Box::new(ViewText::new("Another line".to_string(), 2, 0)),
            ],
            details: ViewDetails {
                width: 80,
                height: 24,
                row: 0,
                col: 0,
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
    }
}
