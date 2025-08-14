use crate::{models::Area, traits::Drawable};
use std::fmt;

type DrawFn<'nodes, T, U> = Box<dyn Fn(Area, &mut T, U) + 'nodes>;

pub(crate) enum SomeDrawable<'nodes, T, U> {
    Fn(DrawFn<'nodes, T, U>),
    Object(Box<dyn Drawable<T, U> + 'nodes>),
}

impl<T, Lens: Copy> SomeDrawable<'_, T, Lens> {
    fn draw(&mut self, area: Area, t: &mut T, lens: Lens, visible: bool) {
        match self {
            SomeDrawable::Fn(closure) => {
                if visible {
                    closure(area, t, lens)
                }
            }
            SomeDrawable::Object(object) => object.draw(area, t, lens, visible),
        }
    }
}

pub(crate) struct DrawableNode<'nodes, T, U> {
    pub(crate) area: Area,
    pub(crate) drawable: SomeDrawable<'nodes, T, U>,
}

impl<T, Lens: Copy> DrawableNode<'_, T, Lens> {
    pub(crate) fn draw(&mut self, area: Area, t: &mut T, lens: Lens, contextual_visibility: bool) {
        if area.width >= 0. && area.height >= 0. {
            self.drawable.draw(area, t, lens, contextual_visibility);
        }
    }
}

impl<T, Lens: Copy> fmt::Debug for DrawableNode<'_, T, Lens> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Drawable")
            .field("area", &self.area)
            .field("draw", &"<function>")
            .finish()
    }
}
