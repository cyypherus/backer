use crate::functional_layout::{
    InputTree,
    tree::IntoTreeTrait,
    types::{Area, AxisConstraint, ConstrainedTree, Constraints, LaidOutTree, LayoutType},
};

pub(crate) fn layout<A>(input: InputTree<A>, available_area: Area) -> LaidOutTree<A> {
    let constrained = resolve(input);
    todo!()
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
    // constrained.into_fold_top_down(available_area, |area, parent| {})
    todo!()
}
