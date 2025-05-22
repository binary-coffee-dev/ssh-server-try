use crate::view::view_details::ViewDetails;
use crate::view::view_text::ViewText;
use crate::view::view_trait::ViewType::Footer;
use crate::view::view_trait::{ViewTrait, ViewType};

#[derive(Clone)]
pub struct ViewFooter {
    pub details: ViewDetails,
    pub children: Vec<Box<ViewText>>,
}

impl ViewFooter {
    pub fn new(row: u32, w: u32, indications: Vec<Box<ViewText>>) -> Self {
        ViewFooter {
            details: ViewDetails {
                width: w,
                height: 1,
                row,
                col: 0,
                focus: false,
                can_focus: false,
            },
            children: indications,
        }
    }
}

impl ViewTrait for ViewFooter {
    fn draw(&mut self, screen: &mut Vec<String>, _parent_details: Option<ViewDetails>) {
        for child in &mut self.children {
            child.draw(screen, Some(self.details.clone()));
        }
    }

    fn redimension(&mut self, width: u32, height: u32) {
        self.details.width = width;
        self.details.row = height - 1;

        let mut child_amount = self.children.len();
        let mut child_displacement = 0;
        for child in &self.children {
            child_displacement += child.details.width;
        }
        for child in &mut self.children {
            if width  < child_displacement + 2 * (child_amount as u32 - 1) {
                break;
            }
            child.details.col = width - (child_displacement + 2 * (child_amount as u32 - 1));
            child_displacement -= child.details.width;
            child_amount -= 1;
        }
    }

    fn view_type(&self) -> ViewType {
        Footer
    }
}
