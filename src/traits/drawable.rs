use crate::models::Area;

/// An object which can be drawn
///
/// See `nodes::draw_object`
pub trait Drawable<T, U> {
    /// Called with the laid-out position for this node
    fn draw(&mut self, area: Area, t: &mut T, u: &mut U, visible: bool);
}
