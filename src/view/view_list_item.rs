use crate::view::view_details::ViewDetails;
use crate::view::view_trait::PostOperation::Underline;
use crate::view::view_trait::{PostOperation, ViewTrait, ViewType};
use std::cmp::min;

#[derive(Clone)]
pub struct ViewListItem {
    pub details: ViewDetails,
    pub text: String,
    pub col: u32,
    pub col_end: u32,
    pub value: String,
}

impl ViewListItem {
    pub fn new(text: String, row: u32, col: u32, value: String) -> Self {
        ViewListItem {
            details: ViewDetails {
                width: text.chars().count() as u32,
                height: 1,
                row,
                col,
                focus: false,
                can_focus: false,
            },
            text: text.clone(),
            col,
            col_end: col + text.chars().count() as u32,
            value,
        }
    }
}

impl ViewTrait for ViewListItem {
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
            self.col = col as u32;
            self.col_end = w as u32;
            for i in 0..(w - col) {
                line[col + i] = text[i];
            }
            screen[row] = line.into_iter().collect();
        }
    }

    fn post_operations(&mut self, _parent_details: Option<ViewDetails>) -> Vec<PostOperation> {
        vec![Underline(self.details.row, self.col, self.col_end)]
    }

    fn view_type(&self) -> ViewType {
        ViewType::ListItem
    }
}
