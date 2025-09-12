use crate::functional_layout::{
    InputTree,
    tree::IntoTreeTrait,
    types::{Area, AxisConstraint, Constraints, LayoutType},
};

pub(crate) fn layout<A>(input: InputTree<A>, available_area: Area) -> InputTree<A> {
    let constrained = resolve(input);
    todo!()
    // allocate_areas(constrained, available_area)
}

pub(crate) fn resolve<A>(input: InputTree<A>) -> InputTree<A> {
    input.into_fold_bottom_up::<InputTree<A>, _>(|mut node| {
        let self_constraints = node.constraints;
        node.resolved = Some(match node.layout {
            LayoutType::Column { spacing, .. } => {
                self_constraints.combine_parent_child(node.children.iter().fold(
                    Option::<Constraints>::None,
                    |current: Option<Constraints>, child_constrained: &InputTree<A>| {
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
                ))
            }
            LayoutType::Row { spacing, .. } => {
                self_constraints.combine_parent_child(node.children.iter().fold(
                    Option::<Constraints>::None,
                    |current: Option<Constraints>, child_constrained: &InputTree<A>| {
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
                ))
            }
            LayoutType::Stack { .. } => {
                self_constraints.combine_parent_child(node.children.iter().fold(
                    Option::<Constraints>::None,
                    |current: Option<Constraints>, child_constrained: &InputTree<A>| {
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
                ))
            }
            LayoutType::Padding {
                leading,
                trailing,
                top,
                bottom,
            } => self_constraints.combine_parent_child(node.children.first().map(|child| {
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
            })),
            LayoutType::Offset { .. } => self_constraints
                .combine_parent_child(node.children.first().map(|child| child.constraints)),
            LayoutType::Space => self_constraints
                .combine_parent_child(node.children.first().map(|child| child.constraints)),
            LayoutType::Coupled { over } => self_constraints.combine_parent_child(if over {
                node.children.first().map(|child| child.constraints)
            } else {
                node.children.get(1).map(|child| child.constraints)
            }),
            LayoutType::Draw(_) | LayoutType::Empty => self_constraints,
        });
        node
    })
}

pub(crate) fn allocate<A>(constrained: InputTree<A>, available_area: Area) -> InputTree<A> {
    // constrained.into_fold_top_down(available_area, |area, parent| {})
    todo!()
}
