use crate::functional_layout::{
    Layout,
    tree::IntoTreeTrait,
    types::{Area, AxisConstraint, Constraints, LayoutType, XAlign, YAlign},
};

pub(crate) fn resolve<A>(input: Layout<A>) -> Layout<A> {
    input.into_fold_bottom_up(|mut node| {
        let self_constraints = node.constraints;
        node.resolved = Some(match node.layout {
            LayoutType::Column { spacing, .. } => {
                self_constraints.combine_parent_child(node.children.iter().fold(
                    Option::<Constraints>::None,
                    |current: Option<Constraints>, child_constrained: &Layout<A>| {
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
                    |current: Option<Constraints>, child_constrained: &Layout<A>| {
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
                    |current: Option<Constraints>, child_constrained: &Layout<A>| {
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

pub(crate) fn allocate<A>(constrained: Layout<A>, available_area: Area) -> Layout<A> {
    constrained.into_fold_top_down(|mut node| {
        let available_area = node.allocated.unwrap_or(available_area);
        match node.layout {
            LayoutType::Draw(_) => {
                node.allocated = Some(available_area);
            }
            LayoutType::Column {
                spacing,
                x_align,
                y_align,
            } => node.layout_axis(spacing, available_area, true, x_align, y_align),
            LayoutType::Row {
                spacing,
                x_align,
                y_align,
            } => node.layout_axis(spacing, available_area, false, x_align, y_align),
            LayoutType::Stack { x_align, y_align } => {
                node.children.iter_mut().for_each(|child| {
                    child.allocated =
                        Some(available_area.constrained(&child.resolved, x_align, y_align));
                });
            }
            LayoutType::Padding {
                leading,
                trailing,
                top,
                bottom,
            } => {
                if let Some(child) = node.children.first_mut() {
                    let constrained_area = available_area.constrained(&child.resolved, None, None);
                    let child_area = Area {
                        x: constrained_area.x + leading,
                        y: constrained_area.y + top,
                        width: (constrained_area.width - leading - trailing).max(0.0),
                        height: (constrained_area.height - top - bottom).max(0.0),
                    };
                    child.allocated = Some(child_area);
                }
            }
            LayoutType::Offset { x, y } => {
                if let Some(child) = node.children.first_mut() {
                    let constrained_area = available_area.constrained(&node.resolved, None, None);
                    let child_area = Area {
                        x: constrained_area.x + x,
                        y: constrained_area.y + y,
                        width: constrained_area.width,
                        height: constrained_area.height,
                    };
                    child.allocated = Some(child_area);
                }
            }
            LayoutType::Coupled { over } => {
                let element = if over { 0 } else { 1 };
                let coupled = if over { 1 } else { 0 };
                let constrained_area =
                    available_area.constrained(&node.children[element].resolved, None, None);
                node.children[element].allocated = Some(constrained_area);
                node.children[coupled].allocated = Some(constrained_area);
            }
            LayoutType::Space | LayoutType::Empty => (),
        }
        node
    })
}

impl<T> Layout<T> {
    fn layout_axis(
        &mut self,
        spacing: f32,
        available_area: Area,
        is_vertical: bool,
        x_align: Option<XAlign>,
        y_align: Option<YAlign>,
    ) {
        if self.children.is_empty() {
            return;
        }

        let element_count = self.children.len();

        let filtered_element_count = element_count;
        let total_spacing = spacing * (element_count as i32 - 1).max(0) as f32;
        let available_size = if is_vertical {
            available_area.height
        } else {
            available_area.width
        } - total_spacing;

        let default_size = available_size / filtered_element_count as f32;

        let mut pool = 0.0;
        let mut final_sizes = vec![None; element_count];
        let mut room_to_grow = vec![0.0; element_count];
        let mut room_to_shrink = vec![0.0; element_count];

        for (i, child) in self.children.iter_mut().enumerate() {
            let mut lower = if is_vertical {
                child
                    .dynamic_constraints
                    .height
                    .as_ref()
                    .map(|f| f(available_area.width))
                    .or(child.constraints.height.lower)
            } else {
                child
                    .dynamic_constraints
                    .width
                    .map(|f| f(available_area.height))
                    .or(child.constraints.width.lower)
            };
            let mut upper = if is_vertical {
                child
                    .dynamic_constraints
                    .height
                    .as_ref()
                    .map(|f| f(available_area.width))
                    .or(child.constraints.height.upper)
            } else {
                child
                    .dynamic_constraints
                    .width
                    .as_ref()
                    .map(|f| f(available_area.height))
                    .or(child.constraints.width.upper)
            };

            if lower.is_none() && upper.is_none() {
                let child_constraints = &child.constraints;

                let is_expanded = if let Some(size_constraints) = child.resolved {
                    if is_vertical {
                        child_constraints.expand_y || size_constraints.should_expand_y()
                    } else {
                        child_constraints.expand_x || size_constraints.should_expand_x()
                    }
                } else if is_vertical {
                    child_constraints.expand_y || child_constraints.height.upper.is_none()
                } else {
                    child_constraints.expand_x || child_constraints.width.upper.is_none()
                };

                if !is_expanded && let Some(size_constraints) = child.resolved {
                    let intrinsic_size = if is_vertical {
                        size_constraints.height.lower.unwrap_or(0.0)
                    } else {
                        size_constraints.width.lower.unwrap_or(0.0)
                    };

                    if intrinsic_size > 0.0 {
                        lower = Some(intrinsic_size);
                        upper = Some(intrinsic_size);
                    }
                }
            }

            let mut final_size = None;

            if let Some(lower) = lower
                && default_size < lower
            {
                pool += default_size - lower;
                final_size = Some(lower);
            }
            if let Some(upper) = upper
                && default_size > upper
            {
                pool += default_size - upper;
                final_size = Some(upper);
            }

            if let Some(lower) = lower {
                if default_size >= lower {
                    room_to_shrink[i] = -(final_size.unwrap_or(default_size) - lower);
                }
            } else {
                room_to_shrink[i] = -default_size;
            }

            if let Some(upper) = upper {
                if default_size <= upper {
                    room_to_grow[i] = -(final_size.unwrap_or(default_size) - upper);
                }
            } else {
                room_to_grow[i] = default_size * 10.0;
            }

            final_sizes[i] = Some(final_size.unwrap_or(default_size));
        }

        fn can_accommodate(room: &[f32]) -> bool {
            room.iter().filter(|r| r.abs() > 0.).count() as f32 > 0.
        }

        let limit = 5;
        let mut i = 0;
        loop {
            if i > limit {
                break;
            }
            i += 1;
            let pool_empty = pool.abs() < 0.1;
            if !pool_empty && pool.is_sign_positive() && can_accommodate(&room_to_grow) {
                let mut enumerated_room: Vec<(usize, f32)> = room_to_grow
                    .iter()
                    .enumerate()
                    .map(|(i, v)| (i, *v))
                    .filter(|(_, v)| *v != 0.)
                    .collect();
                enumerated_room.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
                let distribution_candidates = room_to_grow
                    .iter()
                    .filter(|r| r.abs() > 0. && r.is_sign_positive())
                    .count() as f32;
                let distribution_amount =
                    (pool / distribution_candidates).min(enumerated_room.first().unwrap().1);
                pool -= distribution_amount * distribution_candidates;
                enumerated_room.iter().for_each(|&(i, _)| {
                    if room_to_grow[i].abs() > 0. && room_to_grow[i].is_sign_positive() {
                        room_to_grow[i] -= distribution_amount;
                        if let Some(size) = &mut final_sizes[i] {
                            *size += distribution_amount;
                        }
                    }
                });
            } else if !pool_empty && pool.is_sign_negative() && can_accommodate(&room_to_shrink) {
                let mut enumerated_room: Vec<(usize, f32)> = room_to_shrink
                    .iter()
                    .enumerate()
                    .map(|(i, v)| (i, *v))
                    .filter(|(_, v)| *v != 0.)
                    .collect();
                enumerated_room.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap().reverse());
                let distribution_candidates = room_to_shrink
                    .iter()
                    .filter(|r| r.abs() > 0. && r.is_sign_negative())
                    .count() as f32;
                let distribution_amount =
                    (pool / distribution_candidates).max(enumerated_room.first().unwrap().1);
                pool -= distribution_amount * distribution_candidates;
                enumerated_room.iter().for_each(|&(i, _)| {
                    if room_to_shrink[i].abs() > 0. && room_to_shrink[i].is_sign_negative() {
                        room_to_shrink[i] -= distribution_amount;
                        if let Some(size) = &mut final_sizes[i] {
                            *size += distribution_amount;
                        }
                    }
                });
            } else {
                break;
            }
        }

        let mut current_pos = if is_vertical {
            match y_align.unwrap_or(YAlign::Center) {
                YAlign::Top => available_area.y,
                YAlign::Center => available_area.y + (pool * 0.5),
                YAlign::Bottom => available_area.y + pool,
            }
        } else {
            match x_align.unwrap_or(XAlign::Center) {
                XAlign::Leading => available_area.x,
                XAlign::Center => available_area.x + (pool * 0.5),
                XAlign::Trailing => available_area.x + pool,
            }
        };

        for (i, child) in self.children.iter_mut().enumerate() {
            let child_size = final_sizes[i].unwrap_or(if filtered_element_count > 1 {
                0.0
            } else if is_vertical {
                available_area.height
            } else {
                available_area.width
            });

            let final_area = if is_vertical {
                Area {
                    x: available_area.x,
                    y: current_pos,
                    width: available_area.width,
                    height: child_size,
                }
            } else {
                Area {
                    x: current_pos,
                    y: available_area.y,
                    width: child_size,
                    height: available_area.height,
                }
            }
            .constrained(&child.resolved, x_align, y_align);

            child.allocated = Some(final_area);

            current_pos += child_size + spacing;
        }
    }
}

fn expand_area_reader_nodes<A>(tree: Layout<A>) -> Layout<A> {
    // let mut stack = vec![from];

    // while let Some(node_id) = stack.pop() {
    //     let area = self.tree.get_node(node_id).area;
    //     let expansion_result = if let NodeType::AreaReader {
    //         func,
    //         expanded: expanded @ false,
    //     } = &mut self.tree.get_node_mut(node_id).node_type
    //     {
    //         if let Some(area) = area {
    //             *expanded = true;
    //             let construction_node = func(area, state, ui_state);
    //             let new_child = self.tree.add_child(node_id, construction_node);
    //             self.layout_and_expand(new_child, area, state, ui_state);
    //             Some(new_child)
    //         } else {
    //             eprintln!("Unexpected area reader expansion without area {:?}", self);
    //             None
    //         }
    //     } else {
    //         None
    //     };

    //     if let Some(computed_id) = expansion_result {
    //         stack.push(computed_id);
    //     } else {
    //         let children = self.tree.get_node(node_id).children().clone();
    //         for &child_id in children.iter().rev() {
    //             stack.push(child_id);
    //         }
    //     }
    // }
    todo!()
}

pub(crate) fn collect<A>(constrained: Layout<A>) -> Vec<A> {
    constrained.cata(|node, child_results| {
        if let LayoutType::Draw(value) = node.layout {
            vec![value]
        } else {
            child_results.into_iter().flatten().collect()
        }
    })
}
