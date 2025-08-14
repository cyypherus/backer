use core::fmt;
use std::fmt::{Debug, Formatter};

use crate::{layout::NodeValue, models::Area};

/// A layout tree node. Use methods in [`crate::nodes`] to create nodes.
pub struct Node<'nodes, T, U> {
    pub(crate) inner: NodeValue<'nodes, T, U>,
}

impl<T, U> Debug for Node<'_, T, U> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("NodeWith")
            .field("inner", &self.inner)
            .finish()
    }
}

impl<'nodes, T, U> Node<'nodes, T, U> {
    /// Returns the minimum height of the node based on the contents and constraints of the node & the available area.
    pub fn min_height(&mut self, available_area: Area, t: &mut T, u: &mut U) -> Option<f32> {
        self.inner
            .constraints(available_area, t, u)
            .and_then(|c| c.height.get_lower())
    }

    /// Returns the minimum width of the node based on the contents and constraints of the node & the available area.
    pub fn min_width(&mut self, available_area: Area, state: &mut T, u: &mut U) -> Option<f32> {
        self.inner
            .constraints(available_area, state, u)
            .and_then(|c| c.width.get_lower())
    }
}
