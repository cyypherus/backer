use crate::models::Area;

pub trait Drawable<'nodes, State> {
    fn draw(&mut self, area: Area, state: &mut State, visible: bool);
}
