#![warn(missing_docs)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

/*!
A library for straight-forward UI layout.

Dependency free & framework-agnostic. Backer can be used in an index-based layout approach or with inline drawing code.

_This library **only** implements layout & could be integrated with a range of UI crates._

# Quick Start
See [`Layout`] for setup.

See [`crate::nodes`] for available layout components.
*/

mod api;
mod debug;
mod passes;
mod tests;
mod tree;
mod types;

pub use types::{Align, Area, Layout};

/// All available node constructors
pub mod nodes {
    pub use crate::api::{
        multi_draw, column, column_aligned, column_spaced, column_spaced_aligned, draw, empty,
        row, row_aligned, row_spaced, row_spaced_aligned, space, stack, stack_aligned,
    };
}
