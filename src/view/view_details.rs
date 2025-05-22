#[derive(Clone)]
pub struct ViewDetails {
    pub width: u32,
    pub height: u32,
    pub row: u32,
    pub col: u32,
    pub focus: bool,
    pub can_focus: bool,
}

impl ViewDetails {
    pub fn default() -> Self {
        ViewDetails {
            width: 0,
            height: 0,
            row: 0,
            col: 0,
            focus: false,
            can_focus: false,
        }
    }
}
