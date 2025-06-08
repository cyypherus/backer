use core::fmt;
use std::fmt::{Debug, Formatter};

use crate::{layout::NodeValue, models::Area};

/// A layout tree node. Use methods in [`crate::nodes`] to create nodes.
pub struct Node<'nodes, State> {
    pub(crate) inner: NodeValue<'nodes, State>,
}

impl<State> Debug for Node<'_, State> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("NodeWith")
            .field("inner", &self.inner)
            .finish()
    }
}

impl<'nodes, State> Node<'nodes, State> {
    /// Returns the minimum size of the node based on the contents and constraints of the node & the available area.
    pub fn min_height(&mut self, available_area: Area, state: &mut State) -> Option<f32> {
        self.inner
            .constraints(available_area, state)
            .map(|c| c.height.get_lower())?
    }
    /// Returns the minimum size of the node based on the contents and constraints of the node & the available area.
    pub fn min_width(&mut self, available_area: Area, state: &mut State) -> Option<f32> {
        self.inner
            .constraints(available_area, state)
            .map(|c| c.width.get_lower())?
    }
}
