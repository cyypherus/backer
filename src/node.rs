use core::fmt;
use std::fmt::{Debug, Formatter};

use crate::{
    layout::NodeValue,
    models::{Area, XAlign, YAlign},
};

/// A layout tree node. Use methods in [`crate::nodes`] to create nodes.
pub struct Node<'nodes, State> {
    pub(crate) inner: NodeValue<'nodes, State>,
}

impl<State> Node<'_, State> {
    pub fn draw(&mut self, area: Area, state: &mut State) {
        let constraints = self.inner.constraints(area, state);
        self.inner.layout(
            area.constrained(
                &constraints.unwrap_or_default(),
                XAlign::Center,
                YAlign::Center,
            ),
            None,
            None,
            state,
        );
        self.inner.draw(state, true);
    }
}

impl<State> Debug for Node<'_, State> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("NodeWith")
            .field("inner", &self.inner)
            .finish()
    }
}
