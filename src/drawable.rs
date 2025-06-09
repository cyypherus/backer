use crate::{models::Area, traits::Drawable};
use std::fmt;

type DrawFn<'nodes, T, U> = Box<dyn Fn(Area, &mut T, &mut U) + 'nodes>;

pub(crate) enum SomeDrawable<'nodes, T, U> {
    Fn(DrawFn<'nodes, T, U>),
    Object(Box<dyn Drawable<T, U> + 'nodes>),
}

impl<T, U> SomeDrawable<'_, T, U> {
    fn draw(&mut self, area: Area, t: &mut T, u: &mut U, visible: bool) {
        match self {
            SomeDrawable::Fn(closure) => {
                if visible {
                    closure(area, t, u)
                }
            }
            SomeDrawable::Object(object) => object.draw(area, t, u, visible),
        }
    }
}

pub(crate) struct DrawableNode<'nodes, T, U> {
    pub(crate) area: Area,
    pub(crate) drawable: SomeDrawable<'nodes, T, U>,
}

impl<T, U> DrawableNode<'_, T, U> {
    pub(crate) fn draw(&mut self, area: Area, t: &mut T, u: &mut U, contextual_visibility: bool) {
        if area.width >= 0. && area.height >= 0. {
            self.drawable.draw(area, t, u, contextual_visibility);
        }
    }
}

impl<T, U> fmt::Debug for DrawableNode<'_, T, U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Drawable")
            .field("area", &self.area)
            .field("draw", &"<function>")
            .finish()
    }
}
