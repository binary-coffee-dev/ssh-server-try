use crate::view::actions::Action;
use crate::view::view_details::ViewDetails;
use crate::view::view_list::ViewList;
use crate::view::view_trait::ViewTrait;

#[derive(Clone)]
pub struct ViewRoot {
    pub details: ViewDetails,
    pub children: Vec<Box<dyn ViewTrait>>,
    pub current_view: Box<dyn ViewTrait>,
}

impl ViewRoot {
    pub fn new() -> Self {
        let mut initial_view = Box::new(ViewList::new(0, 0, 80, 24));
        initial_view.details.focus = true;
        ViewRoot {
            children: vec![
                initial_view.clone(),
            ],
            current_view: initial_view,
            details: ViewDetails {
                width: 80,
                height: 24,
                row: 0,
                col: 0,
                focus: false,
                can_focus: false,
            },
        }
    }
}

impl ViewTrait for ViewRoot {
    fn draw(&self, screen: &mut Vec<String>, _parent_details: Option<ViewDetails>) {
        for child in &self.children {
            child.draw(screen, Some(self.details.clone()));
        }
    }

    fn event(&mut self, action: &Action) {
        self.current_view.event(action);
    }

    fn cursor_position(&self, _parent_details: Option<ViewDetails>) -> Option<(u32, u32)> {
        self.current_view.cursor_position(Some(self.details.clone()))
    }

    fn redimension(&mut self, width: u32, height: u32) {
        self.details.width = width;
        self.details.height = height;

        for child in &mut self.children {
            child.redimension(width, height);
        }
    }
}
