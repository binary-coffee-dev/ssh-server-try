#[derive(Debug)]
pub enum Action {
    Up,
    Down,
    Left,
    Right,

    Sigint,
    Eof,
}

pub fn map_key(key: &[u8]) -> Option<Action> {
    match key {
        [27, 91, 65] | [107] => Some(Action::Up),
        [27, 91, 66] | [106]  => Some(Action::Down),
        [27, 91, 68] | [104] => Some(Action::Left),
        [27, 91, 67] | [108] => Some(Action::Right),
        [3] => Some(Action::Sigint),
        [4] => Some(Action::Eof),
        _ => None,
    }
}
