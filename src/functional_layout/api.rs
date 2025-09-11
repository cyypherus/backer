use std::ops::RangeBounds;

use crate::functional_layout::area_allocation::allocate_areas;
use crate::functional_layout::tree::{IntoTreeTrait, TreeTrait};
use crate::functional_layout::types::*;

pub fn draw<A>(data: A) -> InputTree<A> {
    InputTree {
        layout: LayoutType::Draw(data),
        constraints: Default::default(),
        children: Vec::new(),
    }
}

pub fn column<A>(elements: Vec<InputTree<A>>) -> InputTree<A> {
    InputTree {
        layout: LayoutType::Column {
            spacing: 0.,
            x_align: None,
            y_align: None,
        },
        constraints: Default::default(),
        children: elements,
    }
}

pub fn column_spaced<A>(spacing: f32, elements: Vec<InputTree<A>>) -> InputTree<A> {
    InputTree {
        layout: LayoutType::Column {
            spacing,
            x_align: None,
            y_align: None,
        },
        constraints: Default::default(),
        children: elements,
    }
}

pub fn column_aligned<A>(align: Align, elements: Vec<InputTree<A>>) -> InputTree<A> {
    let (x_align, y_align) = align.axis_aligns();
    InputTree {
        layout: LayoutType::Column {
            spacing: 0.,
            x_align,
            y_align,
        },
        constraints: Default::default(),
        children: elements,
    }
}

pub fn column_spaced_aligned<A>(
    spacing: f32,
    align: Align,
    elements: Vec<InputTree<A>>,
) -> InputTree<A> {
    let (x_align, y_align) = align.axis_aligns();
    InputTree {
        layout: LayoutType::Column {
            spacing,
            x_align,
            y_align,
        },
        constraints: Default::default(),
        children: elements,
    }
}

pub fn row<A>(elements: Vec<InputTree<A>>) -> InputTree<A> {
    InputTree {
        layout: LayoutType::Row {
            spacing: 0.0,
            x_align: None,
            y_align: None,
        },
        constraints: Default::default(),
        children: elements,
    }
}

pub fn row_spaced<A>(spacing: f32, elements: Vec<InputTree<A>>) -> InputTree<A> {
    InputTree {
        layout: LayoutType::Row {
            spacing,
            x_align: None,
            y_align: None,
        },
        constraints: Default::default(),
        children: elements,
    }
}

pub fn row_aligned<A>(align: Align, elements: Vec<InputTree<A>>) -> InputTree<A> {
    let (x_align, y_align) = align.axis_aligns();
    InputTree {
        layout: LayoutType::Row {
            spacing: 0.0,
            x_align,
            y_align,
        },
        constraints: Default::default(),
        children: elements,
    }
}

pub fn row_spaced_aligned<A>(
    spacing: f32,
    align: Align,
    elements: Vec<InputTree<A>>,
) -> InputTree<A> {
    let (x_align, y_align) = align.axis_aligns();
    InputTree {
        layout: LayoutType::Row {
            spacing,
            x_align,
            y_align,
        },
        constraints: Default::default(),
        children: elements,
    }
}

pub fn stack<A>(elements: Vec<InputTree<A>>) -> InputTree<A> {
    InputTree {
        layout: LayoutType::Stack {
            x_align: None,
            y_align: None,
        },
        constraints: Default::default(),
        children: elements,
    }
}

pub fn stack_aligned<A>(align: Align, elements: Vec<InputTree<A>>) -> InputTree<A> {
    let (x_align, y_align) = align.axis_aligns();
    InputTree {
        layout: LayoutType::Stack { x_align, y_align },
        constraints: Default::default(),
        children: elements,
    }
}

pub fn space<A>() -> InputTree<A> {
    InputTree {
        layout: LayoutType::Space,
        constraints: Default::default(),
        children: Vec::new(),
    }
}

pub fn empty<A>() -> InputTree<A> {
    InputTree {
        layout: LayoutType::Empty,
        constraints: Default::default(),
        children: Vec::new(),
    }
}

// pub fn dynamic<A>(func: impl Fn(&mut A, &mut A) -> InputTree<A> + 'static) -> InputTree<A> {
//     Node::new(NodeType::Dynamic {
//         func: Box::new(func),
//         expanded: false,
//     })
// }

// pub fn area_reader<A>(
//     func: impl Fn(Area, &mut A, &mut A) -> InputTree<A> + 'static,
// ) -> InputTree<A> {
//     Node::new(NodeType::AreaReader {
//         func: Box::new(func),
//         expanded: false,
//     })
// }

impl<A> InputTree<A> {
    pub fn width(mut self, width: f32) -> Self {
        self.constraints.width.lower = Some(width);
        self.constraints.width.upper = Some(width);
        self
    }

    pub fn height(mut self, height: f32) -> Self {
        self.constraints.height.lower = Some(height);
        self.constraints.height.upper = Some(height);
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

    pub fn width_range<R>(mut self, range: R) -> Self
    where
        R: RangeBounds<f32>,
    {
        let (width_min, width_max) = Self::extract_bounds(range);
        self.constraints.width.lower = width_min;
        self.constraints.width.upper = width_max;
        self
    }

    pub fn height_range<R>(mut self, range: R) -> Self
    where
        R: RangeBounds<f32>,
    {
        let (height_min, height_max) = Self::extract_bounds(range);
        self.constraints.height.lower = height_min;
        self.constraints.height.upper = height_max;
        self
    }

    fn extract_bounds<R: RangeBounds<f32>>(range: R) -> (Option<f32>, Option<f32>) {
        let min = match range.start_bound() {
            std::ops::Bound::Included(bound) | std::ops::Bound::Excluded(bound) => Some(*bound),
            std::ops::Bound::Unbounded => None,
        };
        let max = match range.end_bound() {
            std::ops::Bound::Included(bound) | std::ops::Bound::Excluded(bound) => Some(*bound),
            std::ops::Bound::Unbounded => None,
        };
        (min, max)
    }

    pub fn align(mut self, align: Align) -> Self {
        let (x_align, y_align) = align.axis_aligns();
        if let Some(x) = x_align {
            self.constraints.x_align = Some(x);
        }
        if let Some(y) = y_align {
            self.constraints.y_align = Some(y);
        }
        self
    }

    pub fn pad(self, amount: f32) -> InputTree<A> {
        InputTree {
            layout: LayoutType::Padding {
                leading: amount,
                trailing: amount,
                top: amount,
                bottom: amount,
            },
            constraints: Default::default(),
            children: vec![self],
        }
    }

    pub fn pad_x(self, amount: f32) -> Self {
        InputTree {
            layout: LayoutType::Padding {
                leading: amount,
                trailing: amount,
                top: 0.,
                bottom: 0.,
            },
            constraints: Default::default(),
            children: vec![self],
        }
    }

    pub fn pad_y(self, amount: f32) -> Self {
        InputTree {
            layout: LayoutType::Padding {
                leading: 0.,
                trailing: 0.,
                top: amount,
                bottom: amount,
            },
            constraints: Default::default(),
            children: vec![self],
        }
    }

    pub fn pad_top(self, amount: f32) -> Self {
        InputTree {
            layout: LayoutType::Padding {
                leading: 0.,
                trailing: 0.,
                top: amount,
                bottom: 0.,
            },
            constraints: Default::default(),
            children: vec![self],
        }
    }

    pub fn pad_bottom(self, amount: f32) -> Self {
        InputTree {
            layout: LayoutType::Padding {
                leading: 0.,
                trailing: 0.,
                top: 0.,
                bottom: amount,
            },
            constraints: Default::default(),
            children: vec![self],
        }
    }

    pub fn pad_leading(self, amount: f32) -> Self {
        InputTree {
            layout: LayoutType::Padding {
                leading: amount,
                trailing: 0.,
                top: 0.,
                bottom: 0.,
            },
            constraints: Default::default(),
            children: vec![self],
        }
    }

    pub fn pad_trailing(self, amount: f32) -> Self {
        InputTree {
            layout: LayoutType::Padding {
                leading: 0.,
                trailing: amount,
                top: 0.,
                bottom: 0.,
            },
            constraints: Default::default(),
            children: vec![self],
        }
    }

    pub fn offset(self, x: f32, y: f32) -> InputTree<A> {
        InputTree {
            layout: LayoutType::Offset { x, y },
            constraints: Default::default(),
            children: vec![self],
        }
    }

    pub fn offset_x(self, x: f32) -> InputTree<A> {
        InputTree {
            layout: LayoutType::Offset { x, y: 0. },
            constraints: Default::default(),
            children: vec![self],
        }
    }

    pub fn offset_y(self, y: f32) -> InputTree<A> {
        InputTree {
            layout: LayoutType::Offset { x: 0., y },
            constraints: Default::default(),
            children: vec![self],
        }
    }

    pub fn attach_under(self, node: InputTree<A>) -> InputTree<A> {
        InputTree {
            layout: LayoutType::Coupled { over: false },
            constraints: Default::default(),
            children: vec![self, node],
        }
    }

    pub fn attach_over(self, node: InputTree<A>) -> InputTree<A> {
        InputTree {
            layout: LayoutType::Coupled { over: true },
            constraints: Default::default(),
            children: vec![node, self],
        }
    }
}

pub(crate) fn layout<A>(input: InputTree<A>, available_area: Area) -> LaidOutTree<A> {
    resolve(input);
    todo!()
    // let constrained = resolve_constraints(input);
    // allocate_areas(constrained, available_area)
}

pub(crate) fn resolve<A>(input: InputTree<A>) -> ConstrainedTree<A> {
    input.into_fold_bottom_up::<ConstrainedTree<A>, _>(|node, child_results| {
        let self_constraints = node.constraints;
        match node.layout {
            LayoutType::Draw(data) => ConstrainedTree {
                layout: LayoutType::Draw(data),
                constraints: self_constraints,
                children: Vec::new(),
            },
            LayoutType::Column {
                spacing,
                x_align,
                y_align,
            } => ConstrainedTree {
                layout: LayoutType::Column {
                    spacing,
                    x_align,
                    y_align,
                },
                constraints: self_constraints.combine_parent_child(child_results.iter().fold(
                    Option::<Constraints>::None,
                    |current: Option<Constraints>, child_constrained: &ConstrainedTree<A>| {
                        if let Some(current) = current {
                            Some(Constraints {
                                width: current
                                    .width
                                    .combine_adjacent_priority(child_constrained.constraints.width),
                                height: current
                                    .height
                                    .combine_sum(child_constrained.constraints.height, spacing),
                                ..Default::default()
                            })
                        } else {
                            Some(child_constrained.constraints)
                        }
                    },
                )),
                children: child_results,
            },
            LayoutType::Row {
                spacing,
                x_align,
                y_align,
            } => ConstrainedTree {
                layout: LayoutType::Row {
                    spacing,
                    x_align,
                    y_align,
                },
                constraints: self_constraints.combine_parent_child(child_results.iter().fold(
                    Option::<Constraints>::None,
                    |current: Option<Constraints>, child_constrained: &ConstrainedTree<A>| {
                        if let Some(current) = current {
                            Some(Constraints {
                                width: current
                                    .width
                                    .combine_sum(child_constrained.constraints.width, spacing),
                                height: current.height.combine_adjacent_priority(
                                    child_constrained.constraints.height,
                                ),
                                ..Default::default()
                            })
                        } else {
                            Some(child_constrained.constraints)
                        }
                    },
                )),
                children: child_results,
            },
            LayoutType::Stack { x_align, y_align } => ConstrainedTree {
                layout: LayoutType::Stack { x_align, y_align },
                constraints: self_constraints.combine_parent_child(child_results.iter().fold(
                    Option::<Constraints>::None,
                    |current: Option<Constraints>, child_constrained: &ConstrainedTree<A>| {
                        if let Some(current) = current {
                            Some(Constraints {
                                width: current
                                    .width
                                    .combine_adjacent_priority(child_constrained.constraints.width),
                                height: current.height.combine_adjacent_priority(
                                    child_constrained.constraints.height,
                                ),
                                ..Default::default()
                            })
                        } else {
                            Some(child_constrained.constraints)
                        }
                    },
                )),
                children: child_results,
            },
            LayoutType::Padding {
                leading,
                trailing,
                top,
                bottom,
            } => ConstrainedTree {
                layout: LayoutType::Padding {
                    leading,
                    trailing,
                    top,
                    bottom,
                },
                constraints: self_constraints.combine_parent_child(child_results.first().map(
                    |child| {
                        Constraints {
                            width: AxisConstraint::new(
                                child
                                    .constraints
                                    .width
                                    .lower
                                    .map(|lower| lower + leading + trailing),
                                child
                                    .constraints
                                    .width
                                    .upper
                                    .map(|upper| upper + leading + trailing),
                            ),
                            height: AxisConstraint::new(
                                child
                                    .constraints
                                    .height
                                    .lower
                                    .map(|lower| lower + top + bottom),
                                child
                                    .constraints
                                    .height
                                    .upper
                                    .map(|upper| upper + top + bottom),
                            ),
                            ..Default::default()
                        }
                    },
                )),
                children: child_results,
            },
            LayoutType::Offset { x, y } => ConstrainedTree {
                layout: LayoutType::Offset { x, y },
                constraints: self_constraints
                    .combine_parent_child(child_results.first().map(|child| child.constraints)),
                children: child_results,
            },
            LayoutType::Space => ConstrainedTree {
                layout: LayoutType::Space,
                constraints: self_constraints
                    .combine_parent_child(child_results.first().map(|child| child.constraints)),
                children: child_results,
            },
            LayoutType::Empty => ConstrainedTree {
                layout: LayoutType::Empty,
                constraints: self_constraints,
                children: child_results,
            },
            LayoutType::Coupled { over } => ConstrainedTree {
                layout: LayoutType::Coupled { over },
                constraints: self_constraints.combine_parent_child(if over {
                    child_results.first().map(|child| child.constraints)
                } else {
                    child_results.get(1).map(|child| child.constraints)
                }),
                children: child_results,
            },
        }
    })
}

pub(crate) fn allocate<A>(constrained: ConstrainedTree<A>, available_area: Area) -> LaidOutTree<A> {
    allocate_areas(constrained, available_area)
}
