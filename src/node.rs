use core::fmt;
use std::fmt::{Debug, Formatter};

use crate::layout::NodeValue;

/// A layout tree node. Use methods in [`crate::nodes`] to create nodes.
pub struct Node<'nodes, State> {
    pub(crate) inner: NodeValue<'nodes, State>,
}

impl<'nodes, State> Debug for Node<'nodes, State> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("NodeWith")
            .field("inner", &self.inner)
            .finish()
    }
}
