//! Area allocation phase
//!
//! Takes ConstrainedTree and produces LayoutedTree by allocating actual screen areas
//! to each node in a top-down traversal.

use crate::functional_layout::types::*;

/// Main function: allocate areas for entire tree
pub fn allocate_areas<A>(tree: ConstrainedTree<A>, area: Area) -> LaidOutTree<A> {
    // match &tree.layout {
    //     LayoutType::Leaf => todo!(),
    //     LayoutType::Row { spacing } => todo!(),
    //     LayoutType::Column { spacing } => todo!(),
    //     LayoutType::Stack => todo!(),
    // }
    // Allocate areas to children (pre-order traversal)
    // let allocated_children: Vec<LayoutedTree<A>> = tree
    //     .children
    //     .into_iter()
    //     .zip(child_areas)
    //     .map(|(child, child_area)| allocate_areas(child, child_area))
    //     .collect();

    // LayoutedTree {
    //     data: tree.data,
    //     layout: tree.layout,
    //     constraints: tree.constraints,
    //     resolved: tree.resolved,
    //     area,
    //     children: allocated_children,
    // }
    todo!()
}
