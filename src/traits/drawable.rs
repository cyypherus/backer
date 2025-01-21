use crate::{constraints::SizeConstraints, models::Area};

pub trait Drawable<'nodes, State> {
    fn draw(&mut self, area: Area, state: &mut State, visible: bool);
    fn constraints(&mut self, available_area: Area, state: &mut State) -> Option<SizeConstraints>;
}
