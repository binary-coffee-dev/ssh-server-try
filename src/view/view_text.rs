use crate::view::view_details::ViewDetails;
use crate::view::view_trait::PostOperation::Underline;
use crate::view::view_trait::{PostOperation, ViewTrait};
use std::cmp::min;

#[derive(Clone)]
pub struct ViewText {
    pub details: ViewDetails,
    pub text: String,
}

impl ViewText {
    pub fn new(text: String, row: u32, col: u32, w: u32, h: u32) -> Self {
        ViewText {
            details: ViewDetails {
                width: w,
                height: h,
                row,
                col,
                focus: false,
                can_focus: false,
            },
            text: text.clone(),
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
        let ph = parent_details
            .clone()
            .map_or(self.details.height as usize, |d| d.height as usize);

        let h = min(
            row + self.details.height as usize,
            min(pcol + ph, screen.len()),
        );

        let text: Vec<char> = self.text.chars().collect();
        let mut text_it = 0;
        for j in 0..(h - row) {
            if (text_it >= text.len()) {
                break;
            }

            if j < screen.len() && col < screen[j].chars().count() {
                let mut line: Vec<char> = screen[j].chars().collect();
                let w = min(
                    col + self.details.width as usize,
                    min(pcol + pw, line.len()),
                );

                for i in 0..(w - col) {
                    if (text_it >= text.len()) {
                        break;
                    }

                    line[col + i] = text[text_it];
                    text_it += 1;
                }
                screen[j] = line.into_iter().collect();
            }
        }
    }
}
