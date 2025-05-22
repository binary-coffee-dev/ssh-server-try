use crate::view::view_details::ViewDetails;
use crate::view::view_list_item::ViewListItem;
use crate::view::view_trait::ViewType::Logo;
use crate::view::view_trait::{ViewTrait, ViewType};

#[derive(Clone)]
pub struct ViewLogo {
    pub details: ViewDetails,
    logo_text: Vec<Box<ViewListItem>>,
}

impl ViewLogo {
    pub fn new(row: u32, col: u32) -> Self {
        let binary_logo_text = r#"
______ _                        _____        __  __               _
| ___ (_)                      /  __ \      / _|/ _|             | |
| |_/ /_ _ __   __ _ _ __ _   _| /  \/ ___ | |_| |_ ___  ___   __| | _____   __
| ___ \ | '_ \ / _` | '__| | | | |    / _ \|  _|  _/ _ \/ _ \ / _` |/ _ \ \ / /
| |_/ / | | | | (_| | |  | |_| | \__/\ (_) | | | ||  __/  __/| (_| |  __/\ V /
\____/|_|_| |_|\__,_|_|   \__, |\____/\___/|_| |_| \___|\___(_)__,_|\___| \_/
                           __/ |
                          |___/"#
            .to_string();

        let mut logo_text: Vec<Box<ViewListItem>> = vec![];

        for (i, line) in binary_logo_text.lines().enumerate() {
            logo_text.push(Box::new(ViewListItem::new(
                line.to_string(),
                row + i as u32,
                col,
                "".to_string(),
            )));
        }

        ViewLogo {
            details: ViewDetails {
                width: 80,
                height: 11,
                row,
                col,
                focus: false,
                can_focus: false,
            },
            logo_text,
        }
    }
}

impl ViewTrait for ViewLogo {
    fn draw(&mut self, screen: &mut Vec<String>, _parent_details: Option<ViewDetails>) {
        for text in &mut self.logo_text {
            text.draw(screen, Some(self.details.clone()));
        }
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
