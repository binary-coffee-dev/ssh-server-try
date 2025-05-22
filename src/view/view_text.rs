use std::cmp::min;

use unicode_width::UnicodeWidthChar;

use crate::view::actions::Action;
use crate::view::view_details::ViewDetails;
use crate::view::view_trait::{EventResult, ViewTrait, ViewType};

#[derive(Clone)]
pub enum TextFormat {
    Markdown(String),
    PlainText(String),
}

struct MarkdownDecoder {
    text: Vec<char>,
    position: usize,
    lines: u32,
}

impl MarkdownDecoder {
    pub fn new(text: String) -> Self {
        MarkdownDecoder {
            text: text.chars().collect(),
            position: 0,
            lines: 1,
        }
    }

    pub fn decode(&mut self) -> String {
        let mut decoded_text = String::new();

        while let Some(c) = self.next_char() {
            match c {
                // '#' => {}
                // '*' => {}
                // '`' => {}
                _ => {
                    if c == '\n' {
                        self.lines += 1;
                    }
                    decoded_text.push(c);
                }
            }
        }

        decoded_text
    }

    fn next_char(&mut self) -> Option<char> {
        if self.position < self.text.len() {
            let c = self.text[self.position];
            self.position += 1;
            Some(c)
        } else {
            None
        }
    }
    //
    // fn current_char(&self) -> Option<char> {
    //     if self.position < self.text.len() {
    //         Some(self.text[self.position])
    //     } else {
    //         None
    //     }
    // }
}

#[derive(Clone)]
pub struct ViewText {
    pub details: ViewDetails,
    pub text: TextFormat,
    pub lines: u32,
    pub decoded_text: String,
    pub scroll_position: u32,
}

/// A view that displays text in a given area of the screen. This component support markdown format.
impl ViewText {
    pub fn new(text: TextFormat, row: u32, col: u32, w: u32, h: u32) -> Self {
        let mut instance = ViewText {
            details: ViewDetails {
                width: w,
                height: h,
                row,
                col,
                focus: false,
                can_focus: false,
            },
            text,
            lines: 0,
            decoded_text: String::new(),
            scroll_position: 0,
        };

        instance.decode_text();

        instance
    }

    pub fn decode_text(&mut self) {
        self.decoded_text = match &self.text {
            TextFormat::Markdown(text) => {
                let mut decoder = MarkdownDecoder::new(text.clone());
                let res = decoder.decode();
                self.lines = decoder.lines;
                res
            }
            TextFormat::PlainText(text) => {
                self.lines = text.lines().count() as u32;
                text.clone()
            }
        }
    }
}

impl ViewTrait for ViewText {
    fn draw(&mut self, screen: &mut Vec<String>, parent_details: Option<ViewDetails>) {
        let row_in_screen =
            self.details.row as usize + parent_details.clone().map_or(0, |d| d.row as usize);
        let col_in_screen =
            self.details.col as usize + parent_details.clone().map_or(0, |d| d.col as usize);

        let pcol = parent_details
            .clone()
            .map_or(col_in_screen, |d| d.col as usize);
        let prow = parent_details
            .clone()
            .map_or(row_in_screen as usize, |d| d.row as usize);
        let pw = parent_details
            .clone()
            .map_or(self.details.width as usize, |d| d.width as usize);
        let ph = parent_details
            .clone()
            .map_or(self.details.height as usize, |d| d.height as usize);

        let h = min(
            row_in_screen + self.details.height as usize,
            min(prow + ph, screen.len()),
        );

        let text: Vec<char> = self.decoded_text.chars().collect();
        let mut text_it = 0;

        // skip lines to scroll
        let mut scroll = self.scroll_position;
        while text_it < text.len() && scroll > 0 {
            if text[text_it] == '\n' {
                scroll -= 1;
            }
            text_it += 1;
        }

        for j in row_in_screen..h {
            if text_it >= text.len() {
                break;
            }

            if j < screen.len() && col_in_screen < screen[j].chars().count() {
                let mut line: Vec<char> = screen[j].chars().collect();
                let w = min(
                    col_in_screen + self.details.width as usize,
                    min(pcol + pw, line.len()),
                );

                let mut char_offset = 0;
                let mut i = 0;
                while i < w - col_in_screen - char_offset {
                    if text_it >= text.len() {
                        break;
                    }

                    if text[text_it] == '\n' {
                        text_it += 1;
                        break;
                    }

                    let cw = text[text_it].width().unwrap();
                    if cw > 0 {
                        char_offset += cw - 1;

                        line[col_in_screen + i] = text[text_it];
                        i += 1;
                    }
                    text_it += 1;
                }
                screen[j] = line.into_iter().collect();
            }
        }
    }

    fn event(&mut self, action: &Action) -> Option<EventResult> {
        match action {
            Action::Up => {
                if self.scroll_position > 0 {
                    self.scroll_position -= 1;
                }
                None
            }
            Action::Down => {
                if self.lines > 1 + self.details.height + self.scroll_position {
                    self.scroll_position += 1;
                }
                None
            }
            _ => None,
        }
    }

    fn redimension(&mut self, width: u32, height: u32) {
        self.details.width = width;
        self.details.height = height;
    }

    fn view_type(&self) -> ViewType {
        ViewType::Text
    }
}
