use crate::{layout::NodeValue, models::*, node_cache::NodeCache, Node};
use std::{ops::RangeBounds, rc::Rc};

impl<'nodes, State> Node<'nodes, State> {
    /// Constrains the node's height as a function of available width.
    ///
    /// Generally you should prefer size constraints, aspect ratio constraints or area readers over dynamic height.
    ///
    /// **This is primarily for UI elements such as text** where node height must depend on available width & scaling is
    /// not a simple option.
    pub fn dynamic_height(self, f: impl Fn(f32, &mut State) -> f32 + 'static) -> Self {
        self.wrap_or_update_explicit(Size {
            dynamic_height: Some(Rc::new(f)),
            ..Default::default()
        })
    }
    /// Constrains the node's width as a function of available height.
    ///
    /// Generally you should prefer size constraints, aspect ratio constraints or area readers over dynamic height.
    ///
    /// **This is primarily for UI elements such as text** where node width must depend on available height & scaling is
    /// not a simple option.
    pub fn dynamic_width(self, f: impl Fn(f32, &mut State) -> f32 + 'static) -> Self {
        self.wrap_or_update_explicit(Size {
            dynamic_width: Some(Rc::new(f)),
            ..Default::default()
        })
    }
}

impl<'nodes, State> Node<'nodes, State> {
    /// Adds padding to the node along the leading edge
    pub fn pad_leading(self, amount: f32) -> Node<'nodes, State> {
        Node {
            inner: NodeValue::Padding {
                amounts: Padding {
                    leading: amount,
                    trailing: 0.,
                    top: 0.,
                    bottom: 0.,
                },
                element: Box::new(NodeCache::new(self.inner)),
            },
        }
    }
    /// Adds horizontal padding to the node (leading & trailing)
    pub fn pad_x(self, amount: f32) -> Node<'nodes, State> {
        Node {
            inner: NodeValue::Padding {
                amounts: Padding {
                    leading: amount,
                    trailing: amount,
                    top: 0.,
                    bottom: 0.,
                },
                element: Box::new(NodeCache::new(self.inner)),
            },
        }
    }
    /// Adds padding to the node along the trailing edge
    pub fn pad_trailing(self, amount: f32) -> Node<'nodes, State> {
        Node {
            inner: NodeValue::Padding {
                amounts: Padding {
                    leading: 0.,
                    trailing: amount,
                    top: 0.,
                    bottom: 0.,
                },
                element: Box::new(NodeCache::new(self.inner)),
            },
        }
    }
    /// Adds padding to the node along the top edge
    pub fn pad_top(self, amount: f32) -> Node<'nodes, State> {
        Node {
            inner: NodeValue::Padding {
                amounts: Padding {
                    leading: 0.,
                    trailing: 0.,
                    top: amount,
                    bottom: 0.,
                },
                element: Box::new(NodeCache::new(self.inner)),
            },
        }
    }

    /// Adds vertical padding to the node (top & bottom)
    pub fn pad_y(self, amount: f32) -> Node<'nodes, State> {
        Node {
            inner: NodeValue::Padding {
                amounts: Padding {
                    leading: 0.,
                    trailing: 0.,
                    top: amount,
                    bottom: amount,
                },
                element: Box::new(NodeCache::new(self.inner)),
            },
        }
    }
    /// Adds padding to the node along the bottom edge
    pub fn pad_bottom(self, amount: f32) -> Node<'nodes, State> {
        Node {
            inner: NodeValue::Padding {
                amounts: Padding {
                    leading: 0.,
                    trailing: 0.,
                    top: 0.,
                    bottom: amount,
                },
                element: Box::new(NodeCache::new(self.inner)),
            },
        }
    }
    /// Adds padding to the node on all sides
    pub fn pad(self, amount: f32) -> Node<'nodes, State> {
        Node {
            inner: NodeValue::Padding {
                amounts: Padding {
                    leading: amount,
                    trailing: amount,
                    top: amount,
                    bottom: amount,
                },
                element: Box::new(NodeCache::new(self.inner)),
            },
        }
    }
    /// Offsets the node along the x axis.
    /// This is an absolute offset that simply shifts nodes away from their calculated position
    /// This won't impact layout besides child nodes also being offset
    pub fn offset_x(self, amount: f32) -> Node<'nodes, State> {
        Node {
            inner: NodeValue::Offset {
                offset_x: amount,
                offset_y: 0.,
                element: Box::new(NodeCache::new(self.inner)),
            },
        }
    }
    /// Offsets the node along the y axis.
    /// This is an absolute offset that simply shifts nodes away from their calculated position
    /// This won't impact layout besides child nodes also being offset
    pub fn offset_y(self, amount: f32) -> Node<'nodes, State> {
        Node {
            inner: NodeValue::Offset {
                offset_x: 0.,
                offset_y: amount,
                element: Box::new(NodeCache::new(self.inner)),
            },
        }
    }
    /// Offsets the node along the x & y axis.
    /// This is an absolute offset that simply shifts nodes away from their calculated position
    /// This won't impact layout besides child nodes also being offset
    pub fn offset(self, offset_x: f32, offset_y: f32) -> Node<'nodes, State> {
        Node {
            inner: NodeValue::Offset {
                offset_x,
                offset_y,
                element: Box::new(NodeCache::new(self.inner)),
            },
        }
    }
    /// Specifies an explicit width for a node
    pub fn width(self, width: f32) -> Self {
        self.wrap_or_update_explicit(Size {
            width_min: Some(width),
            width_max: Some(width),
            expand_x: false,
            ..Default::default()
        })
    }
    /// Specifies an explicit height for a node
    pub fn height(self, height: f32) -> Self {
        self.wrap_or_update_explicit(Size {
            height_min: Some(height),
            height_max: Some(height),
            expand_y: false,
            ..Default::default()
        })
    }
    /// Specifies bounds on a node's height
    pub fn height_range<R>(self, range: R) -> Self
    where
        R: RangeBounds<f32>,
    {
        self.wrap_or_update_explicit(Size {
            height_min: match range.start_bound() {
                std::ops::Bound::Included(bound) => Some(*bound),
                std::ops::Bound::Excluded(bound) => Some(*bound),
                std::ops::Bound::Unbounded => None,
            },
            height_max: match range.end_bound() {
                std::ops::Bound::Included(bound) => Some(*bound),
                std::ops::Bound::Excluded(bound) => Some(*bound),
                std::ops::Bound::Unbounded => None,
            },
            expand_y: false,
            ..Default::default()
        })
    }
    /// Specifies bounds on a node's width
    pub fn width_range<R>(self, range: R) -> Self
    where
        R: RangeBounds<f32>,
    {
        self.wrap_or_update_explicit(Size {
            width_min: match range.start_bound() {
                std::ops::Bound::Included(bound) => Some(*bound),
                std::ops::Bound::Excluded(bound) => Some(*bound),
                std::ops::Bound::Unbounded => None,
            },
            width_max: match range.end_bound() {
                std::ops::Bound::Included(bound) => Some(*bound),
                std::ops::Bound::Excluded(bound) => Some(*bound),
                std::ops::Bound::Unbounded => None,
            },
            expand_x: false,
            ..Default::default()
        })
    }
    /// Specifies an alignment along the x and/or y axis.
    ///
    /// If this seems to not have any effect - make sure your constraints create
    /// enough space for the node to adhere to the desired alignment.
    ///
    /// The child (the node to be aligned) must be constrained to be smaller than it's container, &
    /// the container must be constrained to be larger than the child -
    /// otherwise there is no wiggle room for alignment to take effect.
    ///
    /// If you don't want a container to hug / shrink-wrap it's contents use [`Node::expand()`]
    pub fn align(self, align: Align) -> Self {
        let (x, y) = align.axis_aligns();
        match (x, y) {
            (None, None) => self,
            (None, Some(y)) => self.y_align(y),
            (Some(x), None) => self.x_align(x),
            (Some(x), Some(y)) => self.x_align(x).y_align(y),
        }
    }
    /// Applies an alignment to all the contents of a container / sequence node
    ///
    /// This modifier must be used immediately after the container / sequence node to take effect:
    ///
    /// ```rust
    /// use backer::models::*;
    /// use backer::nodes::*;
    ///
    /// row::<(), ()>(vec![
    ///     draw(|a, _| {
    ///         assert_eq!(a, Area::new(60., 0., 10., 100.));
    ///     })
    ///     .width(10.),
    ///     draw(|a, _| {
    ///         assert_eq!(a, Area::new(70., 0., 30., 100.));
    ///     })
    ///     .width(30.),
    /// ])
    /// .align_contents(Align::Trailing)
    /// .expand();
    /// ```
    pub fn align_contents(self, align: Align) -> Self {
        let (x, y) = align.axis_aligns();
        match (x, y) {
            (None, None) => self,
            (None, Some(y)) => self.y_align_contents(y),
            (Some(x), None) => self.x_align_contents(x),
            (Some(x), Some(y)) => self.x_align_contents(x).y_align_contents(y),
        }
    }
    fn x_align_contents(mut self, align: XAlign) -> Self {
        match self.inner {
            NodeValue::Column {
                off_axis_align: ref mut col_align,
                ..
            } => *col_align = Some(align),
            NodeValue::Row {
                align: ref mut row_align,
                ..
            } => *row_align = Some(align),
            NodeValue::Stack {
                ref mut x_align, ..
            } => *x_align = Some(align),
            _ => (),
        }
        self
    }
    fn y_align_contents(mut self, align: YAlign) -> Self {
        match self.inner {
            NodeValue::Row {
                off_axis_align: ref mut row_align,
                ..
            } => *row_align = Some(align),
            NodeValue::Column {
                align: ref mut col_align,
                ..
            } => *col_align = Some(align),
            NodeValue::Stack {
                ref mut y_align, ..
            } => *y_align = Some(align),
            _ => (),
        }
        self
    }
    fn x_align(self, align: XAlign) -> Self {
        self.wrap_or_update_explicit(Size {
            x_align: Some(align),
            ..Default::default()
        })
    }
    fn y_align(self, align: YAlign) -> Self {
        self.wrap_or_update_explicit(Size {
            y_align: Some(align),
            ..Default::default()
        })
    }
    /// Constrains the node's height to `ratio` of width
    pub fn aspect(self, ratio: f32) -> Self {
        self.wrap_or_update_explicit(Size {
            aspect: Some(ratio),
            ..Default::default()
        })
    }
    /// Expands the node along both axes, ignoring child sizes.
    ///
    /// Prevents containers from hugging / shrink-wrapping their contents.
    /// This is mutually exclusive with explicit size constraints.
    pub fn expand(self) -> Self {
        self.wrap_or_update_explicit(Size {
            expand_x: true,
            expand_y: true,
            ..Default::default()
        })
    }
    /// Expands the node along the y axis, ignoring child sizes.
    ///
    /// Prevents containers from hugging / shrink-wrapping their contents.
    /// This is mutually exclusive with explicit height constraints.
    pub fn expand_y(self) -> Self {
        self.wrap_or_update_explicit(Size {
            expand_y: true,
            ..Default::default()
        })
    }
    /// Expands the node along the x axis, ignoring child sizes.
    ///
    /// Prevents containers from hugging / shrink-wrapping their contents.
    /// This is mutually exclusive with explicit width constraints.
    pub fn expand_x(self) -> Self {
        self.wrap_or_update_explicit(Size {
            expand_x: true,
            ..Default::default()
        })
    }
    /// Attaches `node` over this node as an overlay
    ///
    /// The area available to the attached node is the size of the node it's attached to.
    /// Useful for adding an unconstrained node as an ornament, background, or overlay to a constrained node.
    pub fn attach_over(self, node: Self) -> Self {
        Node {
            inner: NodeValue::Coupled {
                over: true,
                element: Box::new(NodeCache::new(self.inner)),
                coupled: Box::new(NodeCache::new(node.inner)),
            },
        }
    }
    /// Attaches `node` under this node as a background
    ///
    /// The area available to the attached node is the size of the node it's attached to.
    /// Useful for adding an unconstrained node as an ornament, background, or overlay to a constrained node.
    pub fn attach_under(self, node: Self) -> Self {
        Node {
            inner: NodeValue::Coupled {
                over: false,
                element: Box::new(NodeCache::new(self.inner)),
                coupled: Box::new(NodeCache::new(node.inner)),
            },
        }
    }
    /// When used alongside a `TransitionDrawable` object, this interpolates transition for this node & it's children or in other words animated removal / addition.
    #[cfg(feature = "transitions")]
    pub fn visible(self, visibility: bool) -> Self {
        Node {
            inner: NodeValue::Visibility {
                visible: visibility,
                element: Box::new(NodeCache::new(self.inner)),
            },
        }
    }
    fn wrap_or_update_explicit(mut self, size: Size<State>) -> Self {
        match self.inner {
            NodeValue::Explicit {
                ref mut options,
                element: _,
            } => {
                let width_update = size.width_min.or(size.width_max).is_some();
                let height_update = size.height_min.or(size.height_max).is_some();
                *options = Size {
                    width_min: if width_update {
                        size.width_min
                    } else {
                        options.width_min
                    },
                    width_max: if width_update {
                        size.width_max
                    } else {
                        options.width_max
                    },
                    height_min: if height_update {
                        size.height_min
                    } else {
                        options.height_min
                    },
                    height_max: if height_update {
                        size.height_min
                    } else {
                        options.height_min
                    },
                    x_align: size.x_align.or(options.x_align),
                    y_align: size.y_align.or(options.y_align),
                    aspect: size.aspect.or(options.aspect),
                    dynamic_height: size.dynamic_height.or(options.dynamic_height.clone()),
                    dynamic_width: size.dynamic_width.or(options.dynamic_width.clone()),
                    expand_x: size.expand_x || options.expand_x,
                    expand_y: size.expand_y || options.expand_y,
                };
            }
            _ => {
                return Node {
                    inner: NodeValue::Explicit {
                        options: size,
                        element: Box::new(NodeCache::new(self.inner)),
                    },
                };
            }
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::models::*;
    use crate::nodes::*;

    #[test]
    fn test_explicit_wrap_valid() {
        let c = space::<()>()
            .width(10.)
            .width_range(5.0..)
            .inner
            .constraints(Area::zero(), &mut ())
            .unwrap();
        assert!(c.width.get_upper().is_none());
        assert_eq!(c.width.get_lower().unwrap(), 5.);
    }
}
