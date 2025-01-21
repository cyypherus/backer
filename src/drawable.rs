use crate::{models::Area, traits::Drawable};
use std::fmt;

type DrawFn<'nodes, State> = Box<dyn Fn(Area, &mut State) + 'nodes>;

pub(crate) enum SomeDrawable<'nodes, State> {
    Fn(DrawFn<'nodes, State>),
    Object(Box<dyn Drawable<'nodes, State> + 'nodes>),
}

impl<State> SomeDrawable<'_, State> {
    fn draw(&mut self, area: Area, state: &mut State, visible: bool) {
        match self {
            SomeDrawable::Fn(closure) => closure(area, state),
            SomeDrawable::Object(object) => object.draw(area, state, visible),
        }
    }
}

pub(crate) struct DrawableNode<'nodes, State> {
    pub(crate) area: Area,
    pub(crate) drawable: SomeDrawable<'nodes, State>,
}

impl<State> DrawableNode<'_, State> {
    pub(crate) fn draw(&mut self, area: Area, state: &mut State, contextual_visibility: bool) {
        if area.width >= 0. && area.height >= 0. {
            self.drawable.draw(area, state, contextual_visibility);
        }
    }
}

impl<State> fmt::Debug for DrawableNode<'_, State> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Drawable")
            .field("area", &self.area)
            .field("draw", &"<function>")
            .finish()
    }
}
