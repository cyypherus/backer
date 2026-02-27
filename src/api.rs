use crate::{
    passes::{collect, perform_layout_passes},
    types::*,
};
use std::ops::RangeBounds;

macro_rules! container_doc {
    () => {
        r#"
Container nodes, by default, will only take up enough space to fit their contents.

If you want the container to take up as much space as is available you can use an `expand` modifier,
or add an unconstrained node to it's contents.

Unconstrained nodes can be conceptualized as "pushing" outwards & expanding their container,
or pushing against other unconstrained nodes with equal force.
        "#
    };
}

impl<A, S> Layout<A, S> {
    /// Performs layout passes & collect all the draw values from draw nodes.
    pub fn draw(&mut self, available_area: Area, state: &mut S) -> Vec<A> {
        perform_layout_passes(self, available_area, state);
        collect(self, state)
    }

    pub fn min_height(&mut self, available_area: Area, state: &mut S) -> Option<f32> {
        perform_layout_passes(self, available_area, state);
        self.constraints().height.lower
    }

    pub fn min_width(&mut self, available_area: Area, state: &mut S) -> Option<f32> {
        perform_layout_passes(self, available_area, state);
        self.constraints().width.lower
    }
}

struct NodeBuilder<A, S = ()> {
    layout: LayoutType<A, S>,
    constraints: Constraints,
    dynamic_constraints: DynamicConstraints<S>,
    children: Vec<Layout<A, S>>,
}

impl<A, S> NodeBuilder<A, S> {
    fn new(layout: LayoutType<A, S>) -> Self {
        Self {
            layout,
            constraints: Constraints::default(),
            dynamic_constraints: DynamicConstraints::default(),
            children: Vec::new(),
        }
    }

    fn children(mut self, children: Vec<Layout<A, S>>) -> Self {
        self.children = children
            .into_iter()
            .filter(|child| !matches!(child.layout, LayoutType::Empty))
            .collect();
        self
    }

    fn child(self, child: Layout<A, S>) -> Self {
        self.children(vec![child])
    }

    fn build(self) -> Layout<A, S> {
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

/// Creates a node that can be drawn.
///
/// This node produces a value from a laid-out `Area` that will be collected when calling `Layout::draw`
pub fn draw<A, S>(data: impl FnOnce(Area, &mut S) -> A + 'static) -> Layout<A, S> {
    NodeBuilder::new(LayoutType::Draw(Some(Box::new(data)))).build()
}

/// Creates a vertical sequence of elements
///
#[doc = container_doc!()]
pub fn column<A, S>(elements: Vec<Layout<A, S>>) -> Layout<A, S> {
    NodeBuilder::new(LayoutType::Column {
        spacing: 0.,
        x_align: None,
        y_align: None,
    })
    .children(elements)
    .build()
}

/// Creates a vertical sequence of elements with the specified spacing between each element.
///
#[doc = container_doc!()]
pub fn column_spaced<A, S>(spacing: f32, elements: Vec<Layout<A, S>>) -> Layout<A, S> {
    NodeBuilder::new(LayoutType::Column {
        spacing,
        x_align: None,
        y_align: None,
    })
    .children(elements)
    .build()
}

/// Creates a vertical sequence of elements with the specified alignment applied to each immediate child.
///
#[doc = container_doc!()]
pub fn column_aligned<A, S>(align: Align, elements: Vec<Layout<A, S>>) -> Layout<A, S> {
    let (x_align, y_align) = align.axis_aligns();
    NodeBuilder::new(LayoutType::Column {
        spacing: 0.,
        x_align,
        y_align,
    })
    .children(elements)
    .build()
}

/// Creates a horizontal sequence of elements with the specified spacing between each element and the specified alignment applied to each immediate child.
///
#[doc = container_doc!()]
pub fn column_spaced_aligned<A, S>(
    spacing: f32,
    align: Align,
    elements: Vec<Layout<A, S>>,
) -> Layout<A, S> {
    let (x_align, y_align) = align.axis_aligns();
    NodeBuilder::new(LayoutType::Column {
        spacing,
        x_align,
        y_align,
    })
    .children(elements)
    .build()
}

/// Creates a horizontal sequence of elements
///
#[doc = container_doc!()]
pub fn row<A, S>(elements: Vec<Layout<A, S>>) -> Layout<A, S> {
    NodeBuilder::new(LayoutType::Row {
        spacing: 0.0,
        x_align: None,
        y_align: None,
    })
    .children(elements)
    .build()
}

/// Creates a horizontal sequence of elements with the specified spacing between each element.
///
#[doc = container_doc!()]
pub fn row_spaced<A, S>(spacing: f32, elements: Vec<Layout<A, S>>) -> Layout<A, S> {
    NodeBuilder::new(LayoutType::Row {
        spacing,
        x_align: None,
        y_align: None,
    })
    .children(elements)
    .build()
}

/// Creates a horizontal sequence of elements with the specified alignment applied to each immediate child.
///
#[doc = container_doc!()]
pub fn row_aligned<A, S>(align: Align, elements: Vec<Layout<A, S>>) -> Layout<A, S> {
    let (x_align, y_align) = align.axis_aligns();
    NodeBuilder::new(LayoutType::Row {
        spacing: 0.0,
        x_align,
        y_align,
    })
    .children(elements)
    .build()
}

/// Creates a horizontal sequence of elements with the specified spacing between each element and the specified alignment applied to each immediate child.
///
#[doc = container_doc!()]
pub fn row_spaced_aligned<A, S>(
    spacing: f32,
    align: Align,
    elements: Vec<Layout<A, S>>,
) -> Layout<A, S> {
    let (x_align, y_align) = align.axis_aligns();
    NodeBuilder::new(LayoutType::Row {
        spacing,
        x_align,
        y_align,
    })
    .children(elements)
    .build()
}

/// Creates a sequence of elements to be laid out on top of each other.
///
#[doc = container_doc!()]
pub fn stack<A, S>(elements: Vec<Layout<A, S>>) -> Layout<A, S> {
    NodeBuilder::new(LayoutType::Stack {
        x_align: None,
        y_align: None,
    })
    .children(elements)
    .build()
}

/// Creates a sequence of elements to be laid out on top of each other with the specified alignment applied to each immediate child.
///
#[doc = container_doc!()]
pub fn stack_aligned<A, S>(align: Align, elements: Vec<Layout<A, S>>) -> Layout<A, S> {
    let (x_align, y_align) = align.axis_aligns();
    NodeBuilder::new(LayoutType::Stack { x_align, y_align })
        .children(elements)
        .build()
}

/// Creates an empty space which is laid out the same as any other node.
///
/// To add spacing between each item in a row or column you can also use
/// [`row_spaced`] & [`column_spaced`]
pub fn space<A, S>() -> Layout<A, S> {
    NodeBuilder::new(LayoutType::Space).build()
}

/// Nothing! This will not have any impact on layout - useful for conditionally
/// adding elements to a layout in the case where nothing should be added.
pub fn empty<A, S>() -> Layout<A, S> {
    NodeBuilder::new(LayoutType::Empty).build()
}

/// Returns nodes based on available area
///
/// This node comes with caveats! Constraints within an area reader **cannot** expand the area reader itself.
/// If it could - it would require the resolution of a cyclical dependency.
pub fn area_reader<A, S>(func: impl Fn(Area, &mut S) -> Layout<A, S> + 'static) -> Layout<A, S> {
    NodeBuilder::new(LayoutType::AreaReader {
        func: Some(Box::new(func)),
    })
    .build()
}

impl<A, S> Layout<A, S> {
    /// Adds padding to the node on all edges
    pub fn pad(self, amount: f32) -> Layout<A, S> {
        NodeBuilder::new(LayoutType::Padding {
            leading: amount,
            trailing: amount,
            top: amount,
            bottom: amount,
        })
        .child(self)
        .build()
    }

    /// Adds horizontal padding to the node (leading & trailing)
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

    /// Adds vertical padding to the node (top & bottom)
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

    /// Adds padding to the node along the top edge
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

    /// Adds padding to the node along the bottom edge
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

    /// Adds padding to the node along the leading edge
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

    /// Adds padding to the node along the trailing edge
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

    /// Offsets the node along the x & y axis.
    /// This is an absolute offset that simply shifts nodes away from their calculated position
    /// This won't impact layout besides child nodes also being offset
    pub fn offset(self, x: f32, y: f32) -> Layout<A, S> {
        NodeBuilder::new(LayoutType::Offset { x, y })
            .child(self)
            .build()
    }

    /// Offsets the node along the x axis.
    /// This is an absolute offset that simply shifts nodes away from their calculated position
    /// This won't impact layout besides child nodes also being offset
    pub fn offset_x(self, x: f32) -> Layout<A, S> {
        NodeBuilder::new(LayoutType::Offset { x, y: 0. })
            .child(self)
            .build()
    }

    /// Offsets the node along the y axis.
    /// This is an absolute offset that simply shifts nodes away from their calculated position
    /// This won't impact layout besides child nodes also being offset
    pub fn offset_y(self, y: f32) -> Layout<A, S> {
        NodeBuilder::new(LayoutType::Offset { x: 0., y })
            .child(self)
            .build()
    }

    /// Attaches `node` under this node as a background
    ///
    /// The area available to the attached node is the size of the node it's attached to.
    /// Useful for adding an unconstrained node as an ornament, background, or overlay to a constrained node.
    pub fn attach_under(self, node: Layout<A, S>) -> Layout<A, S> {
        NodeBuilder::new(LayoutType::Coupled { over: false })
            .children(vec![node, self])
            .build()
    }

    /// Attaches `node` over this node as an overlay
    ///
    /// The area available to the attached node is the size of the node it's attached to.
    /// Useful for adding an unconstrained node as an ornament, background, or overlay to a constrained node.
    pub fn attach_over(self, node: Layout<A, S>) -> Layout<A, S> {
        NodeBuilder::new(LayoutType::Coupled { over: true })
            .children(vec![self, node])
            .build()
    }
}

impl<A, S> Layout<A, S> {
    /// Specifies the z layer of the node.
    /// Layers are global, & all children of a node will inherit their parent's layer.
    pub fn layer(mut self, layer: i32) -> Layout<A, S> {
        self.layer = Some(layer);
        self
    }

    /// Specifies an explicit width for a node
    pub fn width(mut self, width: f32) -> Self {
        self.constraints.width.lower = Some(width);
        self.constraints.width.upper = Some(width);
        self
    }

    /// Specifies an explicit height for a node
    pub fn height(mut self, height: f32) -> Self {
        self.constraints.height.lower = Some(height);
        self.constraints.height.upper = Some(height);
        self
    }

    /// Expands the node along the x axis, ignoring child sizes.
    ///
    /// Prevents containers from hugging / shrink-wrapping their contents.
    /// This is mutually exclusive with explicit width constraints.
    pub fn expand_x(mut self) -> Self {
        self.constraints.expand_x = true;
        self
    }

    /// Expands the node along the y axis, ignoring child sizes.
    ///
    /// Prevents containers from hugging / shrink-wrapping their contents.
    /// This is mutually exclusive with explicit height constraints.
    pub fn expand_y(mut self) -> Self {
        self.constraints.expand_y = true;
        self
    }

    /// Expands the node along both axes, ignoring child sizes.
    ///
    /// Prevents containers from hugging / shrink-wrapping their contents.
    /// This is mutually exclusive with explicit size constraints.
    pub fn expand(self) -> Self {
        self.expand_x().expand_y()
    }

    /// Specifies bounds on a node's width
    pub fn width_range<R>(mut self, range: R) -> Self
    where
        R: RangeBounds<f32>,
    {
        let (width_min, width_max) = Self::extract_bounds(range);
        self.constraints.width.lower = width_min;
        self.constraints.width.upper = width_max;
        self
    }

    /// Specifies bounds on a node's height
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

    /// Specifies an alignment along the x and/or y axis.
    ///
    /// The alignment of a node defines how it will be placed when there is less *or* more space available than it requires along a given axis.
    /// If this function doesn't seem to have any effect on the layout ensure that one of the mentioned conditions is true.
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

    /// Constrains the node's width as a function of available height.
    ///
    /// Generally you should prefer simple size constraints whenever possible.
    ///
    /// **This is primarily for UI elements such as text** where node width must depend on available height.
    pub fn dynamic_width(mut self, f: impl Fn(f32, &mut S) -> f32 + 'static) -> Layout<A, S> {
        self.dynamic_constraints.width = Some(Box::new(f));
        self
    }
    /// Constrains the node's height as a function of available width.
    ///
    /// Generally you should prefer simple size constraints whenever possible.
    ///
    /// **This is primarily for UI elements such as text** where node height must depend on available width.
    pub fn dynamic_height(mut self, f: impl Fn(f32, &mut S) -> f32 + 'static) -> Layout<A, S> {
        self.dynamic_constraints.height = Some(Box::new(f));
        self
    }

    /// Constrains the node's width to a multiple of it's height
    pub fn aspect_width(mut self, ratio: f32) -> Self {
        self.dynamic_constraints.width = Some(Box::new(move |height, _| height * ratio));
        self
    }

    /// Constrains the node's height to a multiple of it's width
    pub fn aspect_height(mut self, ratio: f32) -> Self {
        self.dynamic_constraints.height = Some(Box::new(move |width, _| width / ratio));
        self
    }
}
