use crate::view::view_details::ViewDetails;
use crate::view::view_text::{TextFormat, ViewText};
use crate::view::view_trait::ViewTrait;

#[derive(Clone)]
pub struct ViewFooter {
    pub details: ViewDetails,
    pub children: Vec<Box<ViewText>>,
}

impl ViewFooter {
    pub fn new(row: u32, w: u32) -> Self {
        ViewFooter {
            details: ViewDetails {
                width: w,
                height: 1,
                row,
                col: 0,
                focus: false,
                can_focus: false,
            },
            children: vec![
                Box::new(ViewText::new(
                    TextFormat::PlainText("↑ (k)".to_string()),
                    0,
                    0,
                    5,
                    1,
                )),
                Box::new(ViewText::new(
                    TextFormat::PlainText("↓ (j)".to_string()),
                    0,
                    0,
                    5,
                    1,
                )),
                Box::new(ViewText::new(
                    TextFormat::PlainText("Quit (C+d)".to_string()),
                    0,
                    0,
                    10,
                    1,
                )),
                Box::new(ViewText::new(
                    TextFormat::PlainText("Open (enter)".to_string()),
                    0,
                    0,
                    12,
                    1,
                )),
            ],
        }
    }
}

impl ViewTrait for ViewFooter {
    fn draw(&mut self, screen: &mut Vec<String>, _parent_details: Option<ViewDetails>) {
        // let mut line: Vec<char> = screen[self.details.row as usize].chars().collect();
        // for i in 0..self.details.width {
            // line[i as usize] = '#';
        // }
        // screen[self.details.row as usize] = line.into_iter().collect();

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
            child.details.col = width - (child_displacement + 2 * (child_amount as u32 - 1));
            println!("xxx: {}, {}", child_displacement, child.details.width);
            child_displacement -= child.details.width;
            child_amount -= 1;
        }
    }
}
