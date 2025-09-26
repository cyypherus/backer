use crate::{
    passes::{collect, perform_layout_passes},
    types::*,
};
use std::ops::RangeBounds;

impl<T> Layout<T> {
    pub fn draw(&mut self, available_area: Area) -> Vec<T> {
        perform_layout_passes(self, available_area);
        collect(self)
    }
}

pub fn draw<A>(data: impl FnOnce(Area) -> A + 'static) -> Layout<A> {
    Layout {
        layout: LayoutType::Draw(Some(Box::new(data))),
        constraints: Default::default(),
        dynamic_constraints: DynamicConstraints::default(),
        children: Vec::new(),
        resolved: None,
        allocated: None,
    }
}

pub fn column<A>(elements: Vec<Layout<A>>) -> Layout<A> {
    Layout {
        layout: LayoutType::Column {
            spacing: 0.,
            x_align: None,
            y_align: None,
        },
        constraints: Default::default(),
        dynamic_constraints: DynamicConstraints::default(),
        children: elements,
        resolved: None,
        allocated: None,
    }
}

pub fn column_spaced<A>(spacing: f32, elements: Vec<Layout<A>>) -> Layout<A> {
    Layout {
        layout: LayoutType::Column {
            spacing,
            x_align: None,
            y_align: None,
        },
        constraints: Default::default(),
        dynamic_constraints: DynamicConstraints::default(),
        children: elements,
        resolved: None,
        allocated: None,
    }
}

pub fn column_aligned<A>(align: Align, elements: Vec<Layout<A>>) -> Layout<A> {
    let (x_align, y_align) = align.axis_aligns();
    Layout {
        layout: LayoutType::Column {
            spacing: 0.,
            x_align,
            y_align,
        },
        constraints: Default::default(),
        dynamic_constraints: DynamicConstraints::default(),
        children: elements,
        resolved: None,
        allocated: None,
    }
}

pub fn column_spaced_aligned<A>(spacing: f32, align: Align, elements: Vec<Layout<A>>) -> Layout<A> {
    let (x_align, y_align) = align.axis_aligns();
    Layout {
        layout: LayoutType::Column {
            spacing,
            x_align,
            y_align,
        },
        constraints: Default::default(),
        dynamic_constraints: DynamicConstraints::default(),
        children: elements,
        resolved: None,
        allocated: None,
    }
}

pub fn row<A>(elements: Vec<Layout<A>>) -> Layout<A> {
    Layout {
        layout: LayoutType::Row {
            spacing: 0.0,
            x_align: None,
            y_align: None,
        },
        constraints: Default::default(),
        dynamic_constraints: DynamicConstraints::default(),
        children: elements,
        resolved: None,
        allocated: None,
    }
}

pub fn row_spaced<A>(spacing: f32, elements: Vec<Layout<A>>) -> Layout<A> {
    Layout {
        layout: LayoutType::Row {
            spacing,
            x_align: None,
            y_align: None,
        },
        constraints: Default::default(),
        dynamic_constraints: DynamicConstraints::default(),
        children: elements,
        resolved: None,
        allocated: None,
    }
}

pub fn row_aligned<A>(align: Align, elements: Vec<Layout<A>>) -> Layout<A> {
    let (x_align, y_align) = align.axis_aligns();
    Layout {
        layout: LayoutType::Row {
            spacing: 0.0,
            x_align,
            y_align,
        },
        constraints: Default::default(),
        dynamic_constraints: DynamicConstraints::default(),
        children: elements,
        resolved: None,
        allocated: None,
    }
}

pub fn row_spaced_aligned<A>(spacing: f32, align: Align, elements: Vec<Layout<A>>) -> Layout<A> {
    let (x_align, y_align) = align.axis_aligns();
    Layout {
        layout: LayoutType::Row {
            spacing,
            x_align,
            y_align,
        },
        constraints: Default::default(),
        dynamic_constraints: DynamicConstraints::default(),
        children: elements,
        resolved: None,
        allocated: None,
    }
}

pub fn stack<A>(elements: Vec<Layout<A>>) -> Layout<A> {
    Layout {
        layout: LayoutType::Stack {
            x_align: None,
            y_align: None,
        },
        constraints: Default::default(),
        dynamic_constraints: DynamicConstraints::default(),
        children: elements,
        resolved: None,
        allocated: None,
    }
}

pub fn stack_aligned<A>(align: Align, elements: Vec<Layout<A>>) -> Layout<A> {
    let (x_align, y_align) = align.axis_aligns();
    Layout {
        layout: LayoutType::Stack { x_align, y_align },
        constraints: Default::default(),
        dynamic_constraints: DynamicConstraints::default(),
        children: elements,
        resolved: None,
        allocated: None,
    }
}

pub fn space<A>() -> Layout<A> {
    Layout {
        layout: LayoutType::Space,
        constraints: Default::default(),
        dynamic_constraints: DynamicConstraints::default(),
        children: Vec::new(),
        resolved: None,
        allocated: None,
    }
}

pub fn empty<A>() -> Layout<A> {
    Layout {
        layout: LayoutType::Empty,
        constraints: Default::default(),
        dynamic_constraints: DynamicConstraints::default(),
        children: Vec::new(),
        resolved: None,
        allocated: None,
    }
}

pub fn area_reader<A>(func: impl Fn(Area) -> Layout<A> + 'static) -> Layout<A> {
    Layout {
        layout: LayoutType::AreaReader {
            func: Some(Box::new(func)),
        },
        constraints: Default::default(),
        dynamic_constraints: DynamicConstraints::default(),
        children: Vec::new(),
        resolved: None,
        allocated: None,
    }
}

impl<A> Layout<A> {
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

    pub fn pad(self, amount: f32) -> Layout<A> {
        Layout {
            layout: LayoutType::Padding {
                leading: amount,
                trailing: amount,
                top: amount,
                bottom: amount,
            },
            constraints: Default::default(),
            dynamic_constraints: DynamicConstraints::default(),
            children: vec![self],
            resolved: None,
            allocated: None,
        }
    }

    pub fn pad_x(self, amount: f32) -> Self {
        Layout {
            layout: LayoutType::Padding {
                leading: amount,
                trailing: amount,
                top: 0.,
                bottom: 0.,
            },
            constraints: Default::default(),
            dynamic_constraints: DynamicConstraints::default(),
            children: vec![self],
            resolved: None,
            allocated: None,
        }
    }

    pub fn pad_y(self, amount: f32) -> Self {
        Layout {
            layout: LayoutType::Padding {
                leading: 0.,
                trailing: 0.,
                top: amount,
                bottom: amount,
            },
            constraints: Default::default(),
            dynamic_constraints: DynamicConstraints::default(),
            children: vec![self],
            resolved: None,
            allocated: None,
        }
    }

    pub fn pad_top(self, amount: f32) -> Self {
        Layout {
            layout: LayoutType::Padding {
                leading: 0.,
                trailing: 0.,
                top: amount,
                bottom: 0.,
            },
            constraints: Default::default(),
            dynamic_constraints: DynamicConstraints::default(),
            children: vec![self],
            resolved: None,
            allocated: None,
        }
    }

    pub fn pad_bottom(self, amount: f32) -> Self {
        Layout {
            layout: LayoutType::Padding {
                leading: 0.,
                trailing: 0.,
                top: 0.,
                bottom: amount,
            },
            constraints: Default::default(),
            dynamic_constraints: DynamicConstraints::default(),
            children: vec![self],
            resolved: None,
            allocated: None,
        }
    }

    pub fn pad_leading(self, amount: f32) -> Self {
        Layout {
            layout: LayoutType::Padding {
                leading: amount,
                trailing: 0.,
                top: 0.,
                bottom: 0.,
            },
            constraints: Default::default(),
            dynamic_constraints: DynamicConstraints::default(),
            children: vec![self],
            resolved: None,
            allocated: None,
        }
    }

    pub fn pad_trailing(self, amount: f32) -> Self {
        Layout {
            layout: LayoutType::Padding {
                leading: 0.,
                trailing: amount,
                top: 0.,
                bottom: 0.,
            },
            constraints: Default::default(),
            dynamic_constraints: DynamicConstraints::default(),
            children: vec![self],
            resolved: None,
            allocated: None,
        }
    }

    pub fn offset(self, x: f32, y: f32) -> Layout<A> {
        Layout {
            layout: LayoutType::Offset { x, y },
            constraints: Default::default(),
            dynamic_constraints: DynamicConstraints::default(),
            children: vec![self],
            resolved: None,
            allocated: None,
        }
    }

    pub fn offset_x(self, x: f32) -> Layout<A> {
        Layout {
            layout: LayoutType::Offset { x, y: 0. },
            constraints: Default::default(),
            dynamic_constraints: DynamicConstraints::default(),
            children: vec![self],
            resolved: None,
            allocated: None,
        }
    }

    pub fn offset_y(self, y: f32) -> Layout<A> {
        Layout {
            layout: LayoutType::Offset { x: 0., y },
            constraints: Default::default(),
            dynamic_constraints: DynamicConstraints::default(),
            children: vec![self],
            resolved: None,
            allocated: None,
        }
    }

    pub fn attach_under(self, node: Layout<A>) -> Layout<A> {
        Layout {
            layout: LayoutType::Coupled { over: false },
            constraints: Default::default(),
            dynamic_constraints: DynamicConstraints::default(),
            children: vec![self, node],
            resolved: None,
            allocated: None,
        }
    }

    pub fn attach_over(self, node: Layout<A>) -> Layout<A> {
        Layout {
            layout: LayoutType::Coupled { over: true },
            constraints: Default::default(),
            dynamic_constraints: DynamicConstraints::default(),
            children: vec![node, self],
            resolved: None,
            allocated: None,
        }
    }

    pub fn dynamic_width(mut self, f: impl Fn(f32) -> f32 + 'static) -> Layout<A> {
        self.dynamic_constraints.width = Some(Box::new(f));
        self
    }

    pub fn dynamic_height(mut self, f: impl Fn(f32) -> f32 + 'static) -> Layout<A> {
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
