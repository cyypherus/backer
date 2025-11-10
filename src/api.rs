use crate::{
    passes::{collect, perform_layout_passes},
    types::*,
};
use std::ops::RangeBounds;

impl<T, C> Layout<T, C> {
    pub fn draw_with(&mut self, available_area: Area, context: &mut C) -> Vec<T> {
        perform_layout_passes(self, available_area, context);
        collect(self, context)
    }
}

impl<T> Layout<T, ()> {
    pub fn draw(&mut self, available_area: Area) -> Vec<T> {
        perform_layout_passes(self, available_area, &mut ());
        collect(self, &mut ())
    }
}

struct NodeBuilder<A, C> {
    layout: LayoutType<A, C>,
    constraints: Constraints,
    dynamic_constraints: DynamicConstraints<C>,
    children: Vec<Layout<A, C>>,
}

impl<A, C> NodeBuilder<A, C> {
    fn new(layout: LayoutType<A, C>) -> Self {
        Self {
            layout,
            constraints: Constraints::default(),
            dynamic_constraints: DynamicConstraints::default(),
            children: Vec::new(),
        }
    }

    fn children(mut self, children: Vec<Layout<A, C>>) -> Self {
        self.children = children
            .into_iter()
            .filter(|child| !matches!(child.layout, LayoutType::Empty))
            .collect();
        self
    }

    fn child(self, child: Layout<A, C>) -> Self {
        self.children(vec![child])
    }

    fn build(self) -> Layout<A, C> {
        Layout {
            layout: self.layout,
            constraints: self.constraints,
            dynamic_constraints: self.dynamic_constraints,
            resolved: None,
            allocated: None,
            children: self.children,
        }
    }
}

pub fn draw<A, C>(data: impl FnOnce(Area, &mut C) -> A + 'static) -> Layout<A, C> {
    NodeBuilder::new(LayoutType::Draw(Some(Box::new(data)))).build()
}

pub fn column<A, C>(elements: Vec<Layout<A, C>>) -> Layout<A, C> {
    NodeBuilder::new(LayoutType::Column {
        spacing: 0.,
        x_align: None,
        y_align: None,
    })
    .children(elements)
    .build()
}

pub fn column_spaced<A, C>(spacing: f32, elements: Vec<Layout<A, C>>) -> Layout<A, C> {
    NodeBuilder::new(LayoutType::Column {
        spacing,
        x_align: None,
        y_align: None,
    })
    .children(elements)
    .build()
}

pub fn column_aligned<A, C>(align: Align, elements: Vec<Layout<A, C>>) -> Layout<A, C> {
    let (x_align, y_align) = align.axis_aligns();
    NodeBuilder::new(LayoutType::Column {
        spacing: 0.,
        x_align,
        y_align,
    })
    .children(elements)
    .build()
}

pub fn column_spaced_aligned<A, C>(
    spacing: f32,
    align: Align,
    elements: Vec<Layout<A, C>>,
) -> Layout<A, C> {
    let (x_align, y_align) = align.axis_aligns();
    NodeBuilder::new(LayoutType::Column {
        spacing,
        x_align,
        y_align,
    })
    .children(elements)
    .build()
}

pub fn row<A, C>(elements: Vec<Layout<A, C>>) -> Layout<A, C> {
    NodeBuilder::new(LayoutType::Row {
        spacing: 0.0,
        x_align: None,
        y_align: None,
    })
    .children(elements)
    .build()
}

pub fn row_spaced<A, C>(spacing: f32, elements: Vec<Layout<A, C>>) -> Layout<A, C> {
    NodeBuilder::new(LayoutType::Row {
        spacing,
        x_align: None,
        y_align: None,
    })
    .children(elements)
    .build()
}

pub fn row_aligned<A, C>(align: Align, elements: Vec<Layout<A, C>>) -> Layout<A, C> {
    let (x_align, y_align) = align.axis_aligns();
    NodeBuilder::new(LayoutType::Row {
        spacing: 0.0,
        x_align,
        y_align,
    })
    .children(elements)
    .build()
}

pub fn row_spaced_aligned<A, C>(
    spacing: f32,
    align: Align,
    elements: Vec<Layout<A, C>>,
) -> Layout<A, C> {
    let (x_align, y_align) = align.axis_aligns();
    NodeBuilder::new(LayoutType::Row {
        spacing,
        x_align,
        y_align,
    })
    .children(elements)
    .build()
}

pub fn stack<A, C>(elements: Vec<Layout<A, C>>) -> Layout<A, C> {
    NodeBuilder::new(LayoutType::Stack {
        x_align: None,
        y_align: None,
    })
    .children(elements)
    .build()
}

pub fn stack_aligned<A, C>(align: Align, elements: Vec<Layout<A, C>>) -> Layout<A, C> {
    let (x_align, y_align) = align.axis_aligns();
    NodeBuilder::new(LayoutType::Stack { x_align, y_align })
        .children(elements)
        .build()
}

pub fn space<A, C>() -> Layout<A, C> {
    NodeBuilder::new(LayoutType::Space).build()
}

pub fn empty<A, C>() -> Layout<A, C> {
    NodeBuilder::new(LayoutType::Empty).build()
}

pub fn area_reader<A, C>(func: impl Fn(Area, &mut C) -> Layout<A, C> + 'static) -> Layout<A, C> {
    NodeBuilder::new(LayoutType::AreaReader {
        func: Some(Box::new(func)),
    })
    .build()
}

impl<A, C> Layout<A, C> {
    pub fn pad(self, amount: f32) -> Layout<A, C> {
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

    pub fn offset(self, x: f32, y: f32) -> Layout<A, C> {
        NodeBuilder::new(LayoutType::Offset { x, y })
            .child(self)
            .build()
    }

    pub fn offset_x(self, x: f32) -> Layout<A, C> {
        NodeBuilder::new(LayoutType::Offset { x, y: 0. })
            .child(self)
            .build()
    }

    pub fn offset_y(self, y: f32) -> Layout<A, C> {
        NodeBuilder::new(LayoutType::Offset { x: 0., y })
            .child(self)
            .build()
    }

    pub fn attach_under(self, node: Layout<A, C>) -> Layout<A, C> {
        NodeBuilder::new(LayoutType::Coupled { over: false })
            .children(vec![node, self])
            .build()
    }

    pub fn attach_over(self, node: Layout<A, C>) -> Layout<A, C> {
        NodeBuilder::new(LayoutType::Coupled { over: true })
            .children(vec![self, node])
            .build()
    }
}

impl<A, C> Layout<A, C> {
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

    pub fn dynamic_width(mut self, f: impl Fn(f32, &mut C) -> f32 + 'static) -> Layout<A, C> {
        self.dynamic_constraints.width = Some(Box::new(f));
        self
    }

    pub fn dynamic_height(mut self, f: impl Fn(f32, &mut C) -> f32 + 'static) -> Layout<A, C> {
        self.dynamic_constraints.height = Some(Box::new(f));
        self
    }

    pub fn aspect_width(mut self, ratio: f32) -> Self {
        self.dynamic_constraints.width = Some(Box::new(move |height, _| height * ratio));
        self
    }

    pub fn aspect_height(mut self, ratio: f32) -> Self {
        self.dynamic_constraints.height = Some(Box::new(move |width, _| width / ratio));
        self
    }
}
