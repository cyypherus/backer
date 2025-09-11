use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct InputTree<A> {
    pub data: A,
    pub layout: LayoutType,
    pub constraints: Constraints,
    pub children: Vec<InputTree<A>>,
}

#[derive(Debug, Clone)]
pub struct ConstrainedTree<A> {
    pub data: A,
    pub layout: LayoutType,
    pub constraints: Constraints,
    pub children: Vec<ConstrainedTree<A>>,
}

#[derive(Debug, Clone)]
pub struct LaidOutTree<A> {
    pub data: A,
    pub layout: LayoutType,
    pub constraints: Constraints,
    pub resolved: Constraints,
    pub area: Area,
    pub children: Vec<LaidOutTree<A>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Constraints {
    pub min_width: Option<f32>,
    pub max_width: Option<f32>,
    pub min_height: Option<f32>,
    pub max_height: Option<f32>,
    pub expand_x: bool,
    pub expand_y: bool,
}

/// Allocated screen area
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Area {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Area {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

// =============================================================================
// LAYOUT TYPES
// =============================================================================

/// Layout algorithms for arranging children
#[derive(Debug, Clone, PartialEq)]
pub enum LayoutType {
    Leaf,
    Row { spacing: f32 },
    Column { spacing: f32 },
    Stack,
}
