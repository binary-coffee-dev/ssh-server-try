use crate::view::view_details::ViewDetails;
use crate::view::view_trait::ViewTrait;
use std::cmp::min;

#[derive(Clone)]
pub enum TextFormat {
    Markdown(String),
}

struct MarkdownDecoder {
    text: Vec<char>,
    position: usize,
}

impl MarkdownDecoder {
    pub fn new(text: String) -> Self {
        MarkdownDecoder {
            text: text.chars().collect(),
            position: 0,
        }
    }

    pub fn decode(&mut self) -> String {
        let mut decoded_text = String::new();
        let mut in_code_block = false;
        let mut in_list = false;

        while let Some(c) = self.next_char() {
            match c {
                // '#' => {}
                // '*' => {}
                // '`' => {}
                _ => {
                    if in_list && c == '\n' {
                        in_list = false;
                    }
                    decoded_text.push(c);
                }
            }
        }

        decoded_text
    }

    fn consume_header() -> String {
        "".to_string()
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
}

#[derive(Clone)]
pub struct ViewText {
    pub details: ViewDetails,
    pub text: TextFormat,
    pub decoded_text: String,
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
            decoded_text: String::new(),
        };

        instance.decode_text();

        instance
    }

    pub fn decode_text(&mut self) {
        self.decoded_text = match &self.text {
            TextFormat::Markdown(text) => {
                let mut decoder = MarkdownDecoder::new(text.clone());
                decoder.decode()
            }
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

        let text: Vec<char> = self.decoded_text.chars().collect();
        let mut text_it = 0;
        for j in 0..(h - row) {
            if text_it >= text.len() {
                break;
            }

            if j < screen.len() && col < screen[j].chars().count() {
                let mut line: Vec<char> = screen[j].chars().collect();
                let w = min(
                    col + self.details.width as usize,
                    min(pcol + pw, line.len()),
                );

                for i in 0..(w - col) {
                    if text_it >= text.len() {
                        break;
                    }

                    if text[text_it] == '\n' {
                        text_it += 1;
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
