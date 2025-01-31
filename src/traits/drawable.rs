use crate::models::Area;

/// An object which can be drawn
///
/// See `nodes::draw_object`
pub trait Drawable<'nodes, State> {
    /// Called with the laid-out position for this node
    fn draw(&mut self, area: Area, state: &mut State, visible: bool);
}
