use crate::functional_layout::area_allocation::allocate_areas;
use crate::functional_layout::constraint_resolution::resolve_constraints;
use crate::functional_layout::types::*;

pub fn leaf<A>(data: A) -> InputTree<A> {
    InputTree {
        data,
        layout: LayoutType::Leaf,
        constraints: Default::default(),
        children: Vec::new(),
    }
}

pub fn row<A>(data: A, children: Vec<InputTree<A>>) -> InputTree<A> {
    InputTree {
        data,
        layout: LayoutType::Row { spacing: 0.0 },
        constraints: Default::default(),
        children,
    }
}

pub fn row_spaced<A>(data: A, spacing: f32, children: Vec<InputTree<A>>) -> InputTree<A> {
    InputTree {
        data,
        layout: LayoutType::Row { spacing },
        constraints: Default::default(),
        children,
    }
}

pub fn column<A>(data: A, children: Vec<InputTree<A>>) -> InputTree<A> {
    InputTree {
        data,
        layout: LayoutType::Column { spacing: 0.0 },
        constraints: Default::default(),
        children,
    }
}

pub fn column_spaced<A>(data: A, spacing: f32, children: Vec<InputTree<A>>) -> InputTree<A> {
    InputTree {
        data,
        layout: LayoutType::Column { spacing },
        constraints: Default::default(),
        children,
    }
}

pub fn stack<A>(data: A, children: Vec<InputTree<A>>) -> InputTree<A> {
    InputTree {
        data,
        layout: LayoutType::Stack,
        constraints: Default::default(),
        children,
    }
}

impl<A> InputTree<A> {
    pub fn width(mut self, width: f32) -> Self {
        self.constraints.min_width = Some(width);
        self.constraints.max_width = Some(width);
        self
    }

    pub fn height(mut self, height: f32) -> Self {
        self.constraints.min_height = Some(height);
        self.constraints.max_height = Some(height);
        self
    }

    pub fn expand_x(mut self) -> Self {
        self.constraints.expand_x = true;
        self
    }

    pub fn expand_y(mut self) -> Self {
        self.constraints.expand_y = true;
        self
    }

    pub fn expand(self) -> Self {
        self.expand_x().expand_y()
    }
}

impl<A> InputTree<A> {
    pub fn map<B, F>(self, f: F) -> InputTree<B>
    where
        F: Fn(A) -> B + Clone,
    {
        InputTree {
            data: f(self.data),
            layout: self.layout,
            constraints: self.constraints,
            children: self
                .children
                .into_iter()
                .map(|c| c.map(f.clone()))
                .collect(),
        }
    }
}

// impl<A> ConstrainedTree<A> {
//     /// Transform the data in this tree using the provided function
//     pub fn map<B, F>(self, f: F) -> ConstrainedTree<B>
//     where
//         F: Fn(A) -> B + Clone,
//     {
//         ConstrainedTree {
//             data: f(self.data),
//             layout: self.layout,
//             constraints: self.constraints,
//             resolved: self.resolved,
//             children: self
//                 .children
//                 .into_iter()
//                 .map(|c| c.map(f.clone()))
//                 .collect(),
//         }
//     }
// }

// impl<A> LaidOutTree<A> {
//     /// Transform the data in this tree using the provided function
//     pub fn map<B, F>(self, f: F) -> LaidOutTree<B>
//     where
//         F: Fn(A) -> B + Clone,
//     {
//         LaidOutTree {
//             data: f(self.data),
//             layout: self.layout,
//             constraints: self.constraints,
//             resolved: self.resolved,
//             area: self.area,
//             children: self
//                 .children
//                 .into_iter()
//                 .map(|c| c.map(f.clone()))
//                 .collect(),
//         }
//     }
// }

// =============================================================================
// COMPLETE LAYOUT PIPELINE
// =============================================================================

/// Complete layout process: InputTree -> ConstrainedTree -> LayoutedTree
pub fn layout<A>(input: InputTree<A>, available_area: Area) -> LaidOutTree<A> {
    let constrained = resolve_constraints(input);
    allocate_areas(constrained, available_area)
}

/// Just the constraint resolution phase
pub fn resolve<A>(input: InputTree<A>) -> ConstrainedTree<A> {
    resolve_constraints(input)
}

/// Just the area allocation phase
pub fn allocate<A>(constrained: ConstrainedTree<A>, available_area: Area) -> LaidOutTree<A> {
    allocate_areas(constrained, available_area)
}
