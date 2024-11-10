use core::fmt;
use std::fmt::{Debug, Formatter};

use crate::layout::NodeValue;

/// A layout tree node. Use methods in [`crate::nodes`] to create nodes.
pub struct Node<State> {
    pub(crate) inner: NodeValue<State>,
}

impl<State> Debug for Node<State> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("NodeWith")
            .field("inner", &self.inner)
            .finish()
    }
}
