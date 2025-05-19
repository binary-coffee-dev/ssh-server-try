use crate::view::view_details::ViewDetails;
use crate::view::view_trait::ViewTrait;
use std::cmp::min;

#[derive(Clone)]
pub struct ViewText {
    pub details: ViewDetails,
    pub text: String,
}

impl ViewText {
    pub fn new(text: String, row: u32, col: u32) -> Self {
        ViewText {
            details: ViewDetails {
                width: text.chars().count() as u32,
                height: 1,
                row,
                col,
                focus: false,
                can_focus: false,
            },
            text,
        }
    }
}

impl ViewTrait for ViewText {
    fn draw(&mut self, screen: &mut Vec<String>, parent_details: Option<ViewDetails>) {
        let row = self.details.row as usize + parent_details.clone().map_or(0, |d| d.row as usize);
        let col = self.details.col as usize + parent_details.clone().map_or(0, |d| d.col as usize);

        let pcol = parent_details
            .clone()
            .map_or(self.details.col as usize, |d| d.col as usize);
        let pw = parent_details
            .clone()
            .map_or(self.details.width as usize, |d| d.width as usize);

        if row < screen.len() && col < screen[row].chars().count() {
            let mut line: Vec<char> = screen[row].chars().collect();
            let text: Vec<char> = self.text.chars().collect();
            let w = min(
                col + self.details.width as usize,
                min(pcol + pw, line.len()),
            );
            for i in 0..(w - col) {
                line[col + i] = text[i];
            }
            screen[row] = line.into_iter().collect();
        }
    }

    fn redimension(&mut self, width: u32, height: u32) {
        self.details.width = width;
        self.details.height = height;
    }
}
