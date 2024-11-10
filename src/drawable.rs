use crate::models::Area;
use std::fmt;

pub trait Drawable<State> {
    fn draw(&mut self, area: Area, state: &mut State);
}

impl<State, F: Fn(Area, &mut State)> Drawable<State> for F {
    fn draw(&mut self, area: Area, state: &mut State) {
        self(area, state)
    }
}

pub(crate) struct DrawableNode<State> {
    pub(crate) area: Area,
    pub(crate) drawable: Box<dyn Drawable<State>>,
}

impl<State> DrawableNode<State> {
    pub(crate) fn draw(&mut self, area: Area, state: &mut State) {
        if area.width > 0. && area.height > 0. {
            self.drawable.draw(area, state);
        }
    }
}

impl<State> fmt::Debug for DrawableNode<State> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Drawable")
            .field("area", &self.area)
            .field("draw", &"<function>")
            .finish()
    }
}
