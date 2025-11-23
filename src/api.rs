use crate::{
    passes::{collect, perform_layout_passes},
    types::*,
};
use std::ops::RangeBounds;

impl<State, A> Layout<State, A> {
    pub fn draw(&mut self, available_area: Area, state: &mut State) -> Vec<A> {
        perform_layout_passes(self, available_area, state);
        collect(self, state)
    }
}

struct NodeBuilder<State, A> {
    layout: LayoutType<State, A>,
    constraints: Constraints,
    dynamic_constraints: DynamicConstraints,
    children: Vec<Layout<State, A>>,
}

impl<State, A> NodeBuilder<State, A> {
    fn new(layout: LayoutType<State, A>) -> Self {
        Self {
            layout,
            constraints: Constraints::default(),
            dynamic_constraints: DynamicConstraints::default(),
            children: Vec::new(),
        }
    }

    fn children(mut self, children: Vec<Layout<State, A>>) -> Self {
        self.children = children
            .into_iter()
            .filter(|child| !matches!(child.layout, LayoutType::Empty))
            .collect();
        self
    }

    fn child(self, child: Layout<State, A>) -> Self {
        self.children(vec![child])
    }

    fn build(self) -> Layout<State, A> {
        Layout {
            layout: self.layout,
            constraints: self.constraints,
            dynamic_constraints: self.dynamic_constraints,
            layer: None,
            resolved: None,
            allocated: None,
            children: self.children,
        }
    }
}

pub fn draw<State, A>(data: impl FnOnce(&mut State, Area) -> A + 'static) -> Layout<State, A> {
    NodeBuilder::new(LayoutType::Draw(Some(Box::new(data)))).build()
}

pub fn column<State, A>(elements: Vec<Layout<State, A>>) -> Layout<State, A> {
    NodeBuilder::new(LayoutType::Column {
        spacing: 0.,
        x_align: None,
        y_align: None,
    })
    .children(elements)
    .build()
}

pub fn column_spaced<State, A>(spacing: f32, elements: Vec<Layout<State, A>>) -> Layout<State, A> {
    NodeBuilder::new(LayoutType::Column {
        spacing,
        x_align: None,
        y_align: None,
    })
    .children(elements)
    .build()
}

pub fn column_aligned<State, A>(align: Align, elements: Vec<Layout<State, A>>) -> Layout<State, A> {
    let (x_align, y_align) = align.axis_aligns();
    NodeBuilder::new(LayoutType::Column {
        spacing: 0.,
        x_align,
        y_align,
    })
    .children(elements)
    .build()
}

pub fn column_spaced_aligned<State, A>(
    spacing: f32,
    align: Align,
    elements: Vec<Layout<State, A>>,
) -> Layout<State, A> {
    let (x_align, y_align) = align.axis_aligns();
    NodeBuilder::new(LayoutType::Column {
        spacing,
        x_align,
        y_align,
    })
    .children(elements)
    .build()
}

pub fn row<State, A>(elements: Vec<Layout<State, A>>) -> Layout<State, A> {
    NodeBuilder::new(LayoutType::Row {
        spacing: 0.0,
        x_align: None,
        y_align: None,
    })
    .children(elements)
    .build()
}

pub fn row_spaced<State, A>(spacing: f32, elements: Vec<Layout<State, A>>) -> Layout<State, A> {
    NodeBuilder::new(LayoutType::Row {
        spacing,
        x_align: None,
        y_align: None,
    })
    .children(elements)
    .build()
}

pub fn row_aligned<State, A>(align: Align, elements: Vec<Layout<State, A>>) -> Layout<State, A> {
    let (x_align, y_align) = align.axis_aligns();
    NodeBuilder::new(LayoutType::Row {
        spacing: 0.0,
        x_align,
        y_align,
    })
    .children(elements)
    .build()
}

pub fn row_spaced_aligned<State, A>(
    spacing: f32,
    align: Align,
    elements: Vec<Layout<State, A>>,
) -> Layout<State, A> {
    let (x_align, y_align) = align.axis_aligns();
    NodeBuilder::new(LayoutType::Row {
        spacing,
        x_align,
        y_align,
    })
    .children(elements)
    .build()
}

pub fn stack<State, A>(elements: Vec<Layout<State, A>>) -> Layout<State, A> {
    NodeBuilder::new(LayoutType::Stack {
        x_align: None,
        y_align: None,
    })
    .children(elements)
    .build()
}

pub fn stack_aligned<State, A>(align: Align, elements: Vec<Layout<State, A>>) -> Layout<State, A> {
    let (x_align, y_align) = align.axis_aligns();
    NodeBuilder::new(LayoutType::Stack { x_align, y_align })
        .children(elements)
        .build()
}

pub fn space<State, A>() -> Layout<State, A> {
    NodeBuilder::new(LayoutType::Space).build()
}

pub fn empty<State, A>() -> Layout<State, A> {
    NodeBuilder::new(LayoutType::Empty).build()
}

pub fn area_reader<State, A>(
    func: impl Fn(&mut State, Area) -> Layout<State, A> + 'static,
) -> Layout<State, A> {
    NodeBuilder::new(LayoutType::AreaReader {
        func: Some(Box::new(func)),
    })
    .build()
}

impl<State, A> Layout<State, A> {
    pub fn pad(self, amount: f32) -> Layout<State, A> {
        NodeBuilder::new(LayoutType::Padding {
            leading: amount,
            trailing: amount,
            top: amount,
            bottom: amount,
        })
        .child(self)
        .build()
    }

    pub fn pad_x(self, amount: f32) -> Self {
        NodeBuilder::new(LayoutType::Padding {
            leading: amount,
            trailing: amount,
            top: 0.,
            bottom: 0.,
        })
        .child(self)
        .build()
    }

    pub fn pad_y(self, amount: f32) -> Self {
        NodeBuilder::new(LayoutType::Padding {
            leading: 0.,
            trailing: 0.,
            top: amount,
            bottom: amount,
        })
        .child(self)
        .build()
    }

    pub fn pad_top(self, amount: f32) -> Self {
        NodeBuilder::new(LayoutType::Padding {
            leading: 0.,
            trailing: 0.,
            top: amount,
            bottom: 0.,
        })
        .child(self)
        .build()
    }

    pub fn pad_bottom(self, amount: f32) -> Self {
        NodeBuilder::new(LayoutType::Padding {
            leading: 0.,
            trailing: 0.,
            top: 0.,
            bottom: amount,
        })
        .child(self)
        .build()
    }

    pub fn pad_leading(self, amount: f32) -> Self {
        NodeBuilder::new(LayoutType::Padding {
            leading: amount,
            trailing: 0.,
            top: 0.,
            bottom: 0.,
        })
        .child(self)
        .build()
    }

    pub fn pad_trailing(self, amount: f32) -> Self {
        NodeBuilder::new(LayoutType::Padding {
            leading: 0.,
            trailing: amount,
            top: 0.,
            bottom: 0.,
        })
        .child(self)
        .build()
    }

    pub fn offset(self, x: f32, y: f32) -> Layout<State, A> {
        NodeBuilder::new(LayoutType::Offset { x, y })
            .child(self)
            .build()
    }

    pub fn offset_x(self, x: f32) -> Layout<State, A> {
        NodeBuilder::new(LayoutType::Offset { x, y: 0. })
            .child(self)
            .build()
    }

    pub fn offset_y(self, y: f32) -> Layout<State, A> {
        NodeBuilder::new(LayoutType::Offset { x: 0., y })
            .child(self)
            .build()
    }

    pub fn attach_under(self, node: Layout<State, A>) -> Layout<State, A> {
        NodeBuilder::new(LayoutType::Coupled { over: false })
            .children(vec![self, node])
            .build()
    }

    pub fn attach_over(self, node: Layout<State, A>) -> Layout<State, A> {
        NodeBuilder::new(LayoutType::Coupled { over: true })
            .children(vec![node, self])
            .build()
    }
}

impl<State, A> Layout<State, A> {
    pub fn layer(mut self, layer: i32) -> Layout<State, A> {
        self.layer = Some(layer);
        self
    }

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

    pub fn dynamic_width(mut self, f: impl Fn(f32) -> f32 + 'static) -> Layout<State, A> {
        self.dynamic_constraints.width = Some(Box::new(f));
        self
    }

    pub fn dynamic_height(mut self, f: impl Fn(f32) -> f32 + 'static) -> Layout<State, A> {
        self.dynamic_constraints.height = Some(Box::new(f));
        self
    }

    pub fn aspect_width(mut self, ratio: f32) -> Self {
        self.dynamic_constraints.width = Some(Box::new(move |height| height * ratio));
        self
    }

    pub fn aspect_height(mut self, ratio: f32) -> Self {
        self.dynamic_constraints.height = Some(Box::new(move |width| width / ratio));
        self
    }
}
