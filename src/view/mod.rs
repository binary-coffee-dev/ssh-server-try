pub mod view_details;
pub mod view_trait;
pub mod view_root;
pub mod view_text;

use std::cmp::min;
use crate::view::view_details::ViewDetails;
use crate::view::view_trait::*;

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
    for line in screen_vec {
        screen.push_str(line);
    }
    screen
}

