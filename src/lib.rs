// #![warn(missing_docs)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

/*!
A library for straight-forward UI layout.

Dependency free & framework-agnostic. Backer can be used in an index-based layout approach or with inline drawing code.

_This library **only** implements layout & could be integrated with a range of UI crates._

# Quick Start
See [`Layout`] for setup.

See [`Node`] for layout customization.
*/

// mod constraints;
// mod debug;
// mod drawable;
// mod layout;
// pub use layout::Layout;
// mod modifiers;
// mod node;
// pub use node::Node;
// mod node_cache;
// mod scoper;
// mod subtree;
// mod tests;

mod mvp;

pub use mvp::{Layout, Node};

pub mod models {
    pub use crate::mvp::{Align, Area, Size};
}
pub mod nodes {
    pub use crate::mvp::{
        area_reader, column, column_aligned, column_spaced, column_spaced_aligned, draw, dynamic,
        empty, intermediate, row, row_aligned, row_spaced, row_spaced_aligned, space, stack,
        stack_aligned,
    };
}
pub mod node {
    pub use crate::mvp::Node;
}

// /// Traits for layout definitions
// pub mod traits;

// /// Structs involved in layout definitions
// pub mod models;

// /// Layout core node construction
// pub mod nodes;
