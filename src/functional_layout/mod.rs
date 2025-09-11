//! Functional Layout System
//!
//! A pure functional approach to UI layout with clear phase separation:
//!
//! ## Three-Phase Pipeline
//!
//! 1. **Input Phase**: Build `InputTree<T>` with layout structure and constraints
//! 2. **Constraint Resolution**: `InputTree<T>` → `ConstrainedTree<T>` (bottom-up catamorphism)
//! 3. **Area Allocation**: `ConstrainedTree<T>` → `LayoutedTree<T>` (top-down distribution)
//!
//! ## Usage
//!
//! ```rust
//! use functional_layout::*;
//!
//! // Build input tree
//! let tree = row_spaced("container", 10.0, vec![
//!     leaf("button1").fixed_size(80.0, 30.0),
//!     leaf("button2").width(100.0, None).height(30.0, None),
//! ]);
//!
//! // Complete layout pipeline
//! let layouted = layout(tree, Area::new(0.0, 0.0, 300.0, 100.0));
//!
//! // Or step by step
//! let constrained = resolve(tree);
//! let layouted = allocate(constrained, Area::new(0.0, 0.0, 300.0, 100.0));
//! ```
//!
//! ## Key Features
//!
//! - **Pure functional**: No mutation during layout passes
//! - **Type-safe phases**: Each phase has its own tree type
//! - **Catamorphic constraint resolution**: Bottom-up fold combining child constraints
//! - **Functor instances**: Use `.map()` to transform data at any phase
//! - **Fluent API**: Chain constraint modifiers like `.width(100, None).expand_x()`

pub mod api;
pub mod area_allocation;
pub mod constraint_resolution;
pub mod types;

// Re-export core types
pub use types::{Area, Constraints, LayoutType};
pub use types::{ConstrainedTree, InputTree, LaidOutTree};

// Re-export main API functions
pub use api::{allocate, column, column_spaced, layout, leaf, resolve, row, row_spaced, stack};

// Re-export phase functions for advanced usage
pub use area_allocation::allocate_areas;
pub use constraint_resolution::resolve_constraints;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complete_pipeline() {
        let input = row_spaced(
            "container",
            10.0,
            vec![
                leaf("button1").width(80.0).height(30.0),
                leaf("button2").width(100.0).height(30.0),
            ],
        );

        let layouted = layout(input, Area::new(0.0, 0.0, 300.0, 50.0));

        assert_eq!(layouted.data, "container");
        assert_eq!(layouted.children.len(), 2);
        assert_eq!(layouted.children[0].data, "button1");
        assert_eq!(layouted.children[1].data, "button2");
    }

    #[test]
    fn test_phase_separation() {
        let input = column(
            "root",
            vec![leaf("item1").height(20.0), leaf("item2").height(30.0)],
        );

        // Phase 1: Constraint resolution
        let constrained = resolve(input);
        assert_eq!(constrained.constraints.min_height, Some(50.0));

        // Phase 2: Area allocation
        let layouted = allocate(constrained, Area::new(0.0, 0.0, 100.0, 60.0));
        assert_eq!(layouted.area.height, 60.0);
        assert_eq!(layouted.children[0].area.height, 30.0); // 60 / 2
        assert_eq!(layouted.children[1].area.height, 30.0);
    }

    #[test]
    fn test_constraint_resolution_catamorphism() {
        let input = row(
            "container",
            vec![leaf("a").width(50.0), leaf("b").width(30.0)],
        );

        // Use catamorphism directly for custom processing
        let total_min_width = input.cata(
            &|_data, layout, constraints: Constraints, children: Vec<f32>| match layout {
                LayoutType::Row { spacing } => {
                    children.iter().sum::<f32>() + spacing * (children.len() as f32 - 1.0).max(0.0)
                }
                LayoutType::Leaf => constraints.min_width.unwrap_or(0.0),
                _ => children.iter().fold(0.0f32, |acc, &w| acc.max(w)),
            },
        );

        assert_eq!(total_min_width, 80.0); // 50 + 30
    }
}
