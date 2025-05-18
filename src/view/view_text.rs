use std::cmp::min;
use crate::view::view_details::ViewDetails;
use crate::view::view_trait::ViewTrait;

#[derive(Clone)]
pub struct ViewText {
    pub details: ViewDetails,
    pub text: String,
}

impl ViewText {
    pub fn new(text: String, row: u32, col: u32) -> Self {
        ViewText {
            details: ViewDetails {
                width: text.len() as u32,
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
    fn draw(&self, screen: &mut Vec<String>, parent_details: Option<ViewDetails>) {
        let row = self.details.row as usize + parent_details.clone().map_or(0, |d| d.row as usize);
        let col = self.details.col as usize + parent_details.clone().map_or(0, |d| d.col as usize);

        let prow = parent_details
            .clone()
            .map_or(self.details.row as usize, |d| d.row as usize);
        let pcol = parent_details
            .clone()
            .map_or(self.details.col as usize, |d| d.col as usize);
        let pw = parent_details
            .clone()
            .map_or(self.details.width as usize, |d| d.width as usize);
        let ph = parent_details
            .clone()
            .map_or(self.details.height as usize, |d| d.height as usize);

        if row < screen.len() && col < screen[row].len() {
            let line = &mut screen[row];
            let w = min(col + self.details.width as usize, pcol + pw as usize);
            line.replace_range(col..w as usize, &self.text[col..w as usize]);
        }
    }

    fn redimension(&mut self, width: u32, height: u32) {
        self.details.width = width;
        self.details.height = height;
    }
}
