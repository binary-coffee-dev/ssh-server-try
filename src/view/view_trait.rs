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

pub trait ViewTrait: ViewTraitClone + Send {
    fn draw(&self, screen: &mut Vec<String>, parent_details: Option<ViewDetails>);
    fn redimension(&mut self, width: u32, height: u32);
}

impl Clone for Box<dyn ViewTrait> {
    fn clone(&self) -> Box<dyn ViewTrait> {
        self.clone_box()
    }
}
