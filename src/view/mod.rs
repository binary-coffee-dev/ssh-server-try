use crate::view::view_trait::PostOperation;

pub mod actions;
mod api_client;
pub mod view_article;
pub mod view_details;
pub mod view_list;
pub mod view_list_item;
pub mod view_root;
pub mod view_trait;

// ANSI escape codes BEGIN
#[macro_export]
macro_rules! move_cursor {
    ($a:expr, $b:expr) => {
        format!("\x1B[{};{}H", $a, $b)
    };
    () => {
        format!("\x1B[1;1H")
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
#[macro_export]
macro_rules! underline_begin {
    () => {
        format!("\x1b[4m")
    };
}
#[macro_export]
macro_rules! underline_end {
    () => {
        format!("\x1b[24m")
    };
}
// ANSI escape codes END

pub fn to_screen_text(screen_vec: &Vec<String>, operations: Vec<PostOperation>) -> String {
    let mut screen = String::new();
    let mut row = 0;
    for line in screen_vec {
        let mut col = 0;
        let mut operation_started = false;
        for c in line.chars() {
            // apply operations
            for op in &operations {
                match op {
                    PostOperation::Underline(r, c, c_e) => {
                        if *r == row && *c == col {
                            screen.push_str(underline_begin!().as_str());
                            operation_started = true;
                        } else if operation_started && *r == row && *c_e == col {
                            screen.push_str(underline_end!().as_str());
                            operation_started = false;
                        }
                    }
                }
            }
            
            screen.push(c);
            col += 1;
        }
        if operation_started {
            // check if the operation is Underline
            screen.push_str(underline_end!().as_str());
        }
        
        row += 1;
    }
    screen
}
