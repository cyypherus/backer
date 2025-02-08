use core::fmt;
use std::fmt::{Debug, Formatter};

use crate::{layout::NodeValue, models::Area};

/// A layout tree node. Use methods in [`crate::nodes`] to create nodes.
pub struct Node<'nodes, State> {
    pub(crate) inner: NodeValue<'nodes, State>,
}

impl<'nodes, State> Node<'nodes, State> {
    pub fn min_height(&mut self, available_area: Area, state: &mut State) -> Option<f32> {
        if let Some(constraint) = self.inner.constraints(available_area, state) {
            constraint.height.get_lower()
        } else {
            None
        }
    }
}

impl<State> Debug for Node<'_, State> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("NodeWith")
            .field("inner", &self.inner)
            .finish()
    }
}
