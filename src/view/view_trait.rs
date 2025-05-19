use crate::view::actions::Action;
use crate::view::view_details::ViewDetails;

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

pub enum PostOperation {
    Underline(u32, u32, u32),
}

pub enum Page {
    List,
    Article(String),
}

pub enum EventResult {
    ChangePage(Page),
}

pub trait ViewTrait: ViewTraitClone + Send {
    fn draw(&mut self, screen: &mut Vec<String>, parent_details: Option<ViewDetails>);
    fn redimension(&mut self, _width: u32, _height: u32) {
    }
    fn post_operations(&mut self, _parent_details: Option<ViewDetails>) -> Vec<PostOperation> {
        vec![]
    }
    fn event(&mut self, _action: &Action) -> Option<EventResult> {
        None
    }
    fn cursor_position(&self, _parent_details: Option<ViewDetails>) -> Option<(u32, u32)> {
        None
    }
}

impl Clone for Box<dyn ViewTrait> {
    fn clone(&self) -> Box<dyn ViewTrait> {
        self.clone_box()
    }
}
