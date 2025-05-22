use crate::view::view_details::ViewDetails;
use crate::view::view_text::{TextFormat, ViewText};
use crate::view::view_trait::ViewType::Logo;
use crate::view::view_trait::{ViewTrait, ViewType};

#[derive(Clone)]
pub struct ViewLogo {
    pub details: ViewDetails,
    logo: ViewText,
}

impl ViewLogo {
    pub fn new(row: u32, col: u32) -> Self {
        let binary_logo_text = r#"______ _                        _____        __  __
| ___ (_)                      /  __ \      / _|/ _|
| |_/ /_ _ __   __ _ _ __ _   _| /  \/ ___ | |_| |_ ___  ___
| ___ \ | '_ \ / _` | '__| | | | |    / _ \|  _|  _/ _ \/ _ \
| |_/ / | | | | (_| | |  | |_| | \__/\ (_) | | | ||  __/  __/
\____/|_|_| |_|\__,_|_|   \__, |\____/\___/|_| |_| \___|\___|
                           __/ |
                          |___/"#
            .to_string();
        ViewLogo {
            details: ViewDetails {
                width: 62,
                height: 10,
                row,
                col,
                focus: false,
                can_focus: false,
            },
            logo: ViewText::new(TextFormat::PlainText(binary_logo_text), 0, 0, 62, 8),
        }
    }
}

impl ViewTrait for ViewLogo {
    fn draw(&mut self, screen: &mut Vec<String>, _parent_details: Option<ViewDetails>) {
        self.logo.draw(screen, Some(self.details.clone()));
    }

    fn redimension(&mut self, width: u32, _height: u32) {
        self.details.width = width;
    }

    fn view_type(&self) -> ViewType {
        Logo
    }

    fn get_details(&self) -> ViewDetails {
        self.details.clone()
    }
}
