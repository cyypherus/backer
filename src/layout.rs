use crate::{
    constraints::SizeConstraints, drawable::DrawableNode, models::*, node_cache::NodeCache,
    traits::NodeTrait, Node,
};
use core::f32;
use std::{fmt::Debug, rc::Rc};

/**
The root object used to store & calculate a layout

# Quick Start

```rust

use backer::*;
use backer::models::*;
use backer::nodes::*;

let layout = Layout::new(my_layout_fn);

// UI libraries generally will expose methods to get the available screen size
// In a real implementation this should use the real screen size!
let available_area = Area {
        x: 0.,
        y: 0.,
        width: 100.,
        height: 100.,
    };
let mut my_state = MyState {};

let layout = Layout::new(my_layout_fn);
// Perform layout & draw all of your drawable nodes.
layout.draw(available_area, &mut my_state);

fn my_layout_fn(state: &mut MyState) -> Node<MyState> {
    // Your layout here
    row(vec![
        space(),
    ])
}
struct MyState {}
```
 */
pub struct Layout<State> {
    tree: LayoutFn<State>,
}

pub type LayoutFn<State> = Box<dyn Fn(&mut State) -> Node<State>>;

impl<State> Layout<State> {
    /// Creates a new [`Layout<State>`].
    pub fn new(tree: impl Fn(&mut State) -> Node<State> + 'static) -> Self {
        Self {
            tree: Box::new(tree),
        }
    }
}

impl<State> Layout<State> {
    /// Calculates layout and draws all draw nodes in the tree
    pub fn draw(&self, area: Area, state: &mut State) {
        let mut layout = (self.tree)(state);
        let constraints = layout.inner.constraints(area, state);
        layout.inner.layout(
            area.constrained(&constraints, XAlign::Center, YAlign::Center),
            None,
            None,
            state,
        );
        layout.inner.draw(state);
    }
}

type AreaReaderFn<State> = Rc<dyn Fn(Area, &mut State) -> Node<State>>;

pub(crate) enum NodeValue<State> {
    Padding {
        amounts: Padding,
        element: Box<NodeCache<State>>,
    },
    Column {
        elements: Vec<NodeCache<State>>,
        spacing: f32,
        align: Option<YAlign>,
        off_axis_align: Option<XAlign>,
    },
    Row {
        elements: Vec<NodeCache<State>>,
        spacing: f32,
        align: Option<XAlign>,
        off_axis_align: Option<YAlign>,
    },
    Stack {
        elements: Vec<NodeCache<State>>,
        x_align: Option<XAlign>,
        y_align: Option<YAlign>,
    },
    Group(Vec<NodeCache<State>>),
    Offset {
        offset_x: f32,
        offset_y: f32,
        element: Box<NodeCache<State>>,
    },
    Draw(DrawableNode<State>),
    Explicit {
        options: Size<State>,
        element: Box<NodeCache<State>>,
    },
    Empty,
    Space,
    AreaReader {
        read: AreaReaderFn<State>,
    },
    Coupled {
        over: bool,
        element: Box<NodeCache<State>>,
        coupled: Box<NodeCache<State>>,
    },
}

impl<State> NodeValue<State> {
    pub(crate) fn draw(&mut self, state: &mut State) {
        match self {
            NodeValue::Draw(drawable) => drawable.draw(drawable.area, state),
            NodeValue::Padding { element, .. }
            | NodeValue::Explicit { element, .. }
            | NodeValue::Offset { element, .. } => {
                element.draw(state);
            }
            NodeValue::Stack { elements, .. } => {
                elements.iter_mut().for_each(|el| el.draw(state));
            }
            NodeValue::Column { elements, .. } | NodeValue::Row { elements, .. } => {
                elements.iter_mut().rev().for_each(|el| el.draw(state));
            }
            NodeValue::Space => (),
            NodeValue::Coupled {
                element,
                coupled,
                over,
            } => {
                if *over {
                    element.draw(state);
                    coupled.draw(state);
                } else {
                    coupled.draw(state);
                    element.draw(state);
                }
            }
            NodeValue::Group(_) | NodeValue::Empty | NodeValue::AreaReader { .. } => {
                unreachable!()
            }
        }
    }

    pub(crate) fn contextual_aligns(&self) -> (Option<XAlign>, Option<YAlign>) {
        if let NodeValue::Column {
            align: y,
            off_axis_align: x,
            ..
        }
        | NodeValue::Row {
            align: x,
            off_axis_align: y,
            ..
        } = self
        {
            (*x, *y)
        } else {
            (None, None)
        }
    }

    pub(crate) fn allocate_area(
        &mut self,
        available_area: Area,
        contextual_x_align: Option<XAlign>,
        contextual_y_align: Option<YAlign>,
        state: &mut State,
    ) -> Vec<Area> {
        match self {
            NodeValue::Padding { amounts, .. } => vec![Area {
                x: available_area.x + amounts.leading,
                y: available_area.y + amounts.top,
                width: (available_area.width - amounts.trailing - amounts.leading).max(0.),
                height: (available_area.height - amounts.bottom - amounts.top).max(0.),
            }],
            NodeValue::Column {
                elements,
                spacing,
                align,
                off_axis_align,
            } => layout_axis(
                elements,
                spacing,
                available_area,
                Orientation::Vertical,
                off_axis_align.unwrap_or(XAlign::Center),
                align.unwrap_or(YAlign::Center),
                state,
                true,
            ),
            NodeValue::Row {
                elements,
                spacing,
                align,
                off_axis_align,
            } => layout_axis(
                elements,
                spacing,
                available_area,
                Orientation::Horizontal,
                align.unwrap_or(XAlign::Center),
                off_axis_align.unwrap_or(YAlign::Center),
                state,
                true,
            ),
            NodeValue::Stack {
                elements,
                x_align,
                y_align,
            } => elements
                .iter_mut()
                .map(|child| {
                    available_area.constrained(
                        &child.constraints(available_area, state),
                        x_align.unwrap_or(XAlign::Center),
                        y_align.unwrap_or(YAlign::Center),
                    )
                })
                .collect(),
            NodeValue::Explicit { options, .. } => {
                vec![available_area.constrained(
                    &SizeConstraints::from_size(options.clone(), available_area, state),
                    contextual_x_align.unwrap_or(XAlign::Center),
                    contextual_y_align.unwrap_or(YAlign::Center),
                )]
            }
            NodeValue::Offset {
                offset_x, offset_y, ..
            } => vec![Area {
                x: available_area.x + *offset_x,
                y: available_area.y + *offset_y,
                width: available_area.width,
                height: available_area.height,
            }],
            NodeValue::Draw(_)
            | NodeValue::Space
            | NodeValue::AreaReader { .. }
            | NodeValue::Coupled { .. } => {
                vec![available_area]
            }
            NodeValue::Group(_) | NodeValue::Empty => unreachable!(),
        }
    }

    pub(crate) fn layout(
        &mut self,
        available_area: Area,
        contextual_x_align: Option<XAlign>,
        contextual_y_align: Option<YAlign>,
        state: &mut State,
    ) {
        let contextual_aligns = self.contextual_aligns();

        let allocated = self.allocate_area(
            available_area,
            contextual_aligns.0.or(contextual_x_align),
            contextual_aligns.1.or(contextual_y_align),
            state,
        );

        match self {
            NodeValue::Column {
                elements,
                align: y_align,
                off_axis_align: x_align,
                ..
            }
            | NodeValue::Row {
                elements,
                align: x_align,
                off_axis_align: y_align,
                ..
            } => {
                elements
                    .iter_mut()
                    .zip(allocated)
                    .for_each(|(el, allocation)| el.layout(allocation, *x_align, *y_align, state));
            }
            NodeValue::Stack { elements, .. } => {
                elements
                    .iter_mut()
                    .zip(allocated)
                    .for_each(|(el, allocation)| el.layout(allocation, None, None, state));
            }
            NodeValue::Padding { element, .. }
            | NodeValue::Explicit { element, .. }
            | NodeValue::Offset { element, .. } => {
                element.layout(allocated[0], None, None, state);
            }
            NodeValue::Draw(drawable) => {
                drawable.area = allocated[0];
                drawable.area.width = drawable.area.width.max(0.);
                drawable.area.height = drawable.area.height.max(0.);
            }
            NodeValue::Space => (),
            NodeValue::AreaReader { read } => {
                *self = read(allocated[0], state).inner;
                self.layout(allocated[0], None, None, state);
            }
            NodeValue::Coupled {
                element, coupled, ..
            } => {
                element.layout(allocated[0], None, None, state);
                coupled.layout(allocated[0], None, None, state);
            }
            NodeValue::Group(_) | NodeValue::Empty => unreachable!(),
        }
    }
}

impl Area {
    fn constrained(
        self,
        constraints: &SizeConstraints,
        contextual_x_align: XAlign,
        contextual_y_align: YAlign,
    ) -> Self {
        let mut width = match (
            constraints.width.get_lower(),
            if constraints.expand_x {
                None
            } else {
                constraints.width.get_upper()
            },
        ) {
            (None, None) => self.width,
            (None, Some(upper)) => self.width.min(upper),
            (Some(lower), None) => self.width.max(lower),
            (Some(lower), Some(upper)) => self.width.clamp(lower, upper.max(lower)),
        };
        let mut height = match (
            constraints.height.get_lower(),
            if constraints.expand_y {
                None
            } else {
                constraints.height.get_upper()
            },
        ) {
            (None, None) => self.height,
            (None, Some(upper)) => self.height.min(upper),
            (Some(lower), None) => self.height.max(lower),
            (Some(lower), Some(upper)) => self.height.clamp(lower, upper.max(lower)),
        };
        if let Some(aspect) = constraints.aspect {
            width = (height * aspect).min(width);
            height = (width / aspect).min(height);
        }
        let x = match constraints.x_align.unwrap_or(contextual_x_align) {
            XAlign::Leading => self.x,
            XAlign::Trailing => self.x + (self.width - width),
            XAlign::Center => self.x + (self.width * 0.5) - (width * 0.5),
        };
        let y = match constraints.y_align.unwrap_or(contextual_y_align) {
            YAlign::Top => self.y,
            YAlign::Bottom => self.y + (self.height - height),
            YAlign::Center => self.y + (self.height * 0.5) - (height * 0.5),
        };
        Area {
            x,
            y,
            width,
            height,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum Orientation {
    Horizontal,
    Vertical,
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn layout_axis<State>(
    elements: &mut [NodeCache<State>],
    spacing: &f32,
    available_area: Area,
    orientation: Orientation,
    x_align: XAlign,
    y_align: YAlign,
    state: &mut State,
    check: bool,
) -> Vec<Area> {
    let sizes: Vec<SizeConstraints> = elements
        .iter_mut()
        .map(|element| element.constraints(available_area, state))
        .collect();
    let element_count = elements.len();

    let total_spacing = *spacing * (element_count as i32 - 1).max(0) as f32;
    let available_size = match orientation {
        Orientation::Horizontal => available_area.width,
        Orientation::Vertical => available_area.height,
    } - total_spacing;

    let default_size = available_size / element_count as f32;

    let mut pool = 0.;
    let mut final_sizes = vec![None; element_count];
    let mut room_to_grow = vec![0.0; element_count];
    let mut room_to_shrink = vec![0.0; element_count];

    for (i, size_constraint) in sizes.iter().enumerate() {
        let constraint = match orientation {
            Orientation::Horizontal => size_constraint.width,
            Orientation::Vertical => size_constraint.height,
        };
        let mut final_size = Option::<f32>::None;
        let mut lower = constraint.get_lower();
        let mut upper = constraint.get_upper();

        if let Some(aspect) = size_constraint.aspect {
            match orientation {
                Orientation::Horizontal => {
                    let value = sizes[i].height.clamping(available_area.height) * aspect;
                    lower = Some(value);
                    upper = Some(value);
                }
                Orientation::Vertical => {
                    let value = sizes[i].width.clamping(available_area.width) / aspect;
                    lower = Some(value);
                    upper = Some(value);
                }
            }
        }

        if let Some(lower) = lower {
            if default_size < lower {
                pool += default_size - lower;
                final_size = Some(lower);
            }
        }
        if let Some(upper) = upper {
            if default_size > upper {
                pool += default_size - upper;
                final_size = Some(upper);
            }
        }

        if let Some(lower) = lower {
            if default_size >= lower {
                room_to_shrink[i] = -(final_size.unwrap_or(default_size) - lower);
            }
        } else {
            // Effectively, this means the element can shrink to 0
            room_to_shrink[i] = -default_size;
        }

        if let Some(upper) = upper {
            if default_size <= upper {
                room_to_grow[i] = -(final_size.unwrap_or(default_size) - upper);
            }
        } else {
            // Effectively, this means the element can expand any amount
            room_to_grow[i] = default_size * 10.;
        }

        final_sizes[i] = final_size.unwrap_or(default_size).into();
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
            // We need to use more room
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
                        *size += distribution_amount
                    }
                }
            });
        } else if !pool_empty && pool.is_sign_negative() && can_accommodate(&room_to_shrink) {
            // We need to use less room
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
                        *size += distribution_amount
                    }
                }
            });
        } else {
            break;
        }
    }

    let mut current_pos = match orientation {
        Orientation::Horizontal => match x_align {
            XAlign::Leading => available_area.x,
            XAlign::Center => available_area.x + (pool * 0.5),
            XAlign::Trailing => available_area.x + pool,
        },
        Orientation::Vertical => match y_align {
            YAlign::Top => available_area.y,
            YAlign::Center => available_area.y + (pool * 0.5),
            YAlign::Bottom => available_area.y + pool,
        },
    };

    let mut areas = Vec::<Area>::new();
    for (i, child) in elements.iter_mut().enumerate() {
        let child_size = final_sizes[i].unwrap();

        let area = match orientation {
            Orientation::Horizontal => Area {
                x: current_pos,
                y: available_area.y,
                width: child_size,
                height: available_area.height,
            },
            Orientation::Vertical => Area {
                x: available_area.x,
                y: current_pos,
                width: available_area.width,
                height: child_size,
            },
        }
        .constrained(&sizes[i], x_align, y_align);

        if !check {
            child.layout(area, Some(x_align), Some(y_align), state);
        } else {
            areas.push(area);
        }

        current_pos += child_size + *spacing;
    }
    areas
}
