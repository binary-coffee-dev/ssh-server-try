use std::cmp::min;

// ANSI escape codes BEGIN
#[macro_export]
macro_rules! move_cursor {
    ($a:expr, $b:expr) => {
        format!("\x1B[{};{}H", $a, $b)
    };
    () => {
        format!("\x1B[0;0H")
    };
}
#[macro_export]
macro_rules! clear_screen {
    () => {
        format!("\x1B[2J")
    };
}
#[macro_export]
macro_rules! enter_alt_screen {
    () => {
        format!("\x1B[?1049h")
    };
}
#[macro_export]
macro_rules! exit_alt_screen {
    () => {
        format!("\x1B[?1049l")
    };
}
// ANSI escape codes END

pub fn to_screen_text(screen_vec: &Vec<String>) -> String {
    let mut screen = String::new();
    for i in 0..screen_vec.len() {
        screen.push_str(&screen_vec[i]);
        // if i < screen_vec.len() - 1 {
        //     screen.push('\n');
        // }
    }
    screen
}

pub trait ViewTraitClone {
    fn clone_box(&self) -> Box<dyn ViewTrait>;
}

impl<T> ViewTraitClone for T
where
    T: 'static + ViewTrait + Clone,
{
    fn clone_box(&self) -> Box<dyn ViewTrait> {
        Box::new(self.clone())
    }
}

pub trait ViewTrait: ViewTraitClone + Send {
    fn draw(&self, screen: &mut Vec<String>, parent_details: Option<ViewDetails>);
    fn redimension(&mut self, width: u32, height: u32);
}

impl Clone for Box<dyn ViewTrait> {
    fn clone(&self) -> Box<dyn ViewTrait> {
        self.clone_box()
    }
}

#[derive(Clone)]
pub struct ViewDetails {
    pub width: u32,
    pub height: u32,
    pub row: u32,
    pub col: u32,
}

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
                Box::new(ViewText::new("This is a test asdf asdf asdf asdf asdf asdfasd fasd fsa".to_string(), 1, 0)),
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
            },
            text,
        }
    }
}

impl ViewTrait for ViewText {
    fn draw(&self, screen: &mut Vec<String>, parent_details: Option<ViewDetails>) {
        let row = self.details.row as usize + parent_details.clone().map_or(0, |d| d.row as usize);
        let col = self.details.col as usize + parent_details.clone().map_or(0, |d| d.col as usize);

        let prow = parent_details.clone().map_or(self.details.row as usize, |d| d.row as usize);
        let pcol = parent_details.clone().map_or(self.details.col as usize, |d| d.col as usize);
        let pw = parent_details.clone().map_or(self.details.width as usize, |d| d.width as usize);
        let ph = parent_details.clone().map_or(self.details.height as usize, |d| d.height as usize);

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
