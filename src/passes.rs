use std::collections::HashMap;

use crate::{
    Layout,
    tree::TreeNode,
    types::{Area, AxisConstraint, Constraints, DrawFn, LayoutType, XAlign, YAlign},
};

pub(crate) fn perform_layout_passes<'a, D, S>(
    tree: &mut Layout<'a, D, S>,
    available_area: Area,
    state: &mut S,
) {
    for _ in 0..2 {
        tree.allocated = Some(available_area);
        resolve(tree, state);
        allocate(tree, available_area, state);
    }
}

pub(crate) fn resolve<'a, D, S>(input: &mut Layout<'a, D, S>, state: &mut S) {
    input.traverse_bottom_up(|node| {
        let self_constraints = Constraints {
            width: node
                .allocated
                .and_then(|area| {
                    node.dynamic_constraints.width.as_ref().map(|f| {
                        let w = f(area.height, state);
                        AxisConstraint::new(Some(w), Some(w))
                    })
                })
                .unwrap_or(AxisConstraint::new(
                    node.constraints.width.lower,
                    node.constraints.width.upper,
                )),
            height: node
                .allocated
                .and_then(|area| {
                    node.dynamic_constraints.height.as_ref().map(|f| {
                        let h = f(area.width, state);
                        AxisConstraint::new(Some(h), Some(h))
                    })
                })
                .unwrap_or(AxisConstraint::new(
                    node.constraints.height.lower,
                    node.constraints.height.upper,
                )),
            expand_x: node.constraints.expand_x,
            expand_y: node.constraints.expand_y,
            transparent_x: node.constraints.transparent_x,
            transparent_y: node.constraints.transparent_y,
            x_align: node.constraints.x_align,
            y_align: node.constraints.y_align,
        };
        node.resolved = Some(match node.layout {
            LayoutType::Column { spacing, .. } => {
                self_constraints.combine_parent_child(node.children.iter().fold(
                    Option::<Constraints>::None,
                    |current: Option<Constraints>, child: &Layout<'a, D, S>| {
                        let mut masked = child.constraints();
                        if child.constraints.transparent_x {
                            masked.width = AxisConstraint::none();
                        }
                        if child.constraints.transparent_y {
                            masked.height = AxisConstraint::none();
                        }
                        if child.constraints.transparent_x && child.constraints.transparent_y {
                            return current;
                        }
                        if let Some(current) = current {
                            Some(Constraints {
                                width: current.width.combine_adjacent_priority(masked.width),
                                height: current.height.combine_sum(masked.height, spacing),
                                ..Default::default()
                            })
                        } else {
                            Some(masked)
                        }
                    },
                ))
            }
            LayoutType::Row { spacing, .. } => {
                self_constraints.combine_parent_child(node.children.iter().fold(
                    Option::<Constraints>::None,
                    |current: Option<Constraints>, child: &Layout<'a, D, S>| {
                        let mut masked = child.constraints();
                        if child.constraints.transparent_x {
                            masked.width = AxisConstraint::none();
                        }
                        if child.constraints.transparent_y {
                            masked.height = AxisConstraint::none();
                        }
                        if child.constraints.transparent_x && child.constraints.transparent_y {
                            return current;
                        }
                        if let Some(current) = current {
                            Some(Constraints {
                                width: current.width.combine_sum(masked.width, spacing),
                                height: current.height.combine_adjacent_priority(masked.height),
                                ..Default::default()
                            })
                        } else {
                            Some(masked)
                        }
                    },
                ))
            }
            LayoutType::Stack { .. } => {
                self_constraints.combine_parent_child(node.children.iter().fold(
                    Option::<Constraints>::None,
                    |current: Option<Constraints>, child: &Layout<'a, D, S>| {
                        let mut masked = child.constraints();
                        if child.constraints.transparent_x {
                            masked.width = AxisConstraint::none();
                        }
                        if child.constraints.transparent_y {
                            masked.height = AxisConstraint::none();
                        }
                        if child.constraints.transparent_x && child.constraints.transparent_y {
                            return current;
                        }
                        if let Some(current) = current {
                            Some(Constraints {
                                width: current.width.combine_adjacent_priority(masked.width),
                                height: current.height.combine_adjacent_priority(masked.height),
                                ..Default::default()
                            })
                        } else {
                            Some(masked)
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
                            .constraints()
                            .width
                            .lower
                            .map(|lower| lower + leading + trailing),
                        child
                            .constraints()
                            .width
                            .upper
                            .map(|upper| upper + leading + trailing),
                    ),
                    height: AxisConstraint::new(
                        child
                            .constraints()
                            .height
                            .lower
                            .map(|lower| lower + top + bottom),
                        child
                            .constraints()
                            .height
                            .upper
                            .map(|upper| upper + top + bottom),
                    ),
                    ..Default::default()
                }
            })),
            LayoutType::Offset { .. } | LayoutType::Space => self_constraints
                .combine_parent_child(node.children.first().map(|child| child.constraints())),
            LayoutType::Draw(_) | LayoutType::Empty => self_constraints,
        });
    })
}

pub(crate) fn allocate<'a, D, S>(
    constrained: &mut Layout<'a, D, S>,
    available_area: Area,
    state: &mut S,
) {
    constrained.traverse_top_down(|node| {
        let available_area =
            node.allocated
                .unwrap_or(available_area)
                .constrained(&node.resolved, None, None);

        match node.layout {
            LayoutType::Draw(_) => {
                node.allocated = Some(available_area);
            }
            LayoutType::Column {
                spacing,
                x_align,
                y_align,
            } => node.layout_axis(spacing, available_area, true, x_align, y_align, state),
            LayoutType::Row {
                spacing,
                x_align,
                y_align,
            } => node.layout_axis(spacing, available_area, false, x_align, y_align, state),
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
                    let child_area = Area {
                        x: available_area.x + leading,
                        y: available_area.y + top,
                        width: (available_area.width - leading - trailing).max(0.0),
                        height: (available_area.height - top - bottom).max(0.0),
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

            LayoutType::Space | LayoutType::Empty => (),
        }
    })
}

impl<'a, D, S> Layout<'a, D, S> {
    fn layout_axis(
        &mut self,
        spacing: f32,
        available_area: Area,
        is_vertical: bool,
        x_align: Option<XAlign>,
        y_align: Option<YAlign>,
        state: &mut S,
    ) {
        if self.children.is_empty() {
            return;
        }

        let element_count = self.children.len();

        let non_transparent_count = self
            .children
            .iter()
            .filter(|c| {
                if is_vertical {
                    !c.constraints.transparent_y
                } else {
                    !c.constraints.transparent_x
                }
            })
            .count();
        let filtered_element_count = non_transparent_count;
        let total_spacing = spacing * (non_transparent_count as i32 - 1).max(0) as f32;
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
            let transparent_on_main = if is_vertical {
                child.constraints.transparent_y
            } else {
                child.constraints.transparent_x
            };
            if transparent_on_main {
                final_sizes[i] = Some(0.0);
                continue;
            }
            let mut lower = if is_vertical {
                child
                    .dynamic_constraints
                    .height
                    .as_ref()
                    .map(|f| f(available_area.width, state))
                    .or(child.constraints.height.lower)
            } else {
                child
                    .dynamic_constraints
                    .width
                    .as_ref()
                    .map(|f| f(available_area.height, state))
                    .or(child.constraints.width.lower)
            };
            let mut upper = if is_vertical {
                child
                    .dynamic_constraints
                    .height
                    .as_ref()
                    .map(|f| f(available_area.width, state))
                    .or(child.constraints.height.upper)
            } else {
                child
                    .dynamic_constraints
                    .width
                    .as_ref()
                    .map(|f| f(available_area.height, state))
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
            let transparent_on_main = if is_vertical {
                child.constraints.transparent_y
            } else {
                child.constraints.transparent_x
            };
            if transparent_on_main {
                let final_area = if is_vertical {
                    Area {
                        x: available_area.x,
                        y: current_pos,
                        width: available_area.width,
                        height: 0.0,
                    }
                } else {
                    Area {
                        x: current_pos,
                        y: available_area.y,
                        width: 0.0,
                        height: available_area.height,
                    }
                };
                child.allocated = Some(final_area);
                continue;
            }

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

pub(crate) fn collect<'a, D, S>(constrained: &mut Layout<'a, D, S>, state: &mut S) -> Vec<D> {
    let mut layers = HashMap::<i32, Vec<(Option<Area>, DrawFn<'a, D, S>)>>::new();
    let mut stack = vec![(constrained, 0)];
    while let Some((node, mut contextual_layer)) = stack.pop() {
        if let Some(node_layer) = node.layer {
            contextual_layer = node_layer
        }
        if let LayoutType::Draw(draw_fn) = &mut node.layout
            && let Some(draw_fn) = draw_fn.take()
        {
            layers
                .entry(contextual_layer)
                .or_default()
                .push((node.allocated, draw_fn));
        }

        let children = node.children_mut();
        for child in children.rev() {
            stack.push((child, contextual_layer));
        }
    }
    let mut keys: Vec<i32> = layers.keys().cloned().collect();
    keys.sort();
    let mut result = Vec::<D>::new();
    for key in keys {
        for (area, func) in layers.remove(&key).unwrap_or_default() {
            let Some(area) = area else { continue };
            result.extend(func(area, state));
        }
    }
    result
}
