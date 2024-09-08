use crate::{layout::Node, layout::NodeValue, models::*};

impl<U> Node<U> {
    /// Adds padding to the node along the leading edge
    pub fn pad_leading(self, amount: f32) -> Node<U> {
        Node {
            inner: NodeValue::Padding {
                amounts: Padding {
                    leading: amount,
                    trailing: 0.,
                    top: 0.,
                    bottom: 0.,
                },
                element: Box::new(self.inner),
            },
        }
    }
    /// Adds horizontal padding to the node (leading & trailing)
    pub fn pad_x(self, amount: f32) -> Node<U> {
        Node {
            inner: NodeValue::Padding {
                amounts: Padding {
                    leading: amount,
                    trailing: amount,
                    top: 0.,
                    bottom: 0.,
                },
                element: Box::new(self.inner),
            },
        }
    }
    /// Adds padding to the node along the trailing edge
    pub fn pad_trailing(self, amount: f32) -> Node<U> {
        Node {
            inner: NodeValue::Padding {
                amounts: Padding {
                    leading: 0.,
                    trailing: amount,
                    top: 0.,
                    bottom: 0.,
                },
                element: Box::new(self.inner),
            },
        }
    }
    /// Adds padding to the node along the top edge
    pub fn pad_top(self, amount: f32) -> Node<U> {
        Node {
            inner: NodeValue::Padding {
                amounts: Padding {
                    leading: 0.,
                    trailing: 0.,
                    top: amount,
                    bottom: 0.,
                },
                element: Box::new(self.inner),
            },
        }
    }

    /// Adds vertical padding to the node (top & bottom)
    pub fn pad_y(self, amount: f32) -> Node<U> {
        Node {
            inner: NodeValue::Padding {
                amounts: Padding {
                    leading: 0.,
                    trailing: 0.,
                    top: amount,
                    bottom: amount,
                },
                element: Box::new(self.inner),
            },
        }
    }
    /// Adds padding to the node along the bottom edge
    pub fn pad_bottom(self, amount: f32) -> Node<U> {
        Node {
            inner: NodeValue::Padding {
                amounts: Padding {
                    leading: 0.,
                    trailing: 0.,
                    top: 0.,
                    bottom: amount,
                },
                element: Box::new(self.inner),
            },
        }
    }
    /// Adds padding to the node on all sides
    pub fn pad(self, amount: f32) -> Node<U> {
        Node {
            inner: NodeValue::Padding {
                amounts: Padding {
                    leading: amount,
                    trailing: amount,
                    top: amount,
                    bottom: amount,
                },
                element: Box::new(self.inner),
            },
        }
    }
    /// Offsets the node along the x axis.
    /// This is an absolute offset that simply shifts nodes away from their calculated position
    /// This won't impact layout besides child nodes also being offset
    pub fn offset_x(self, amount: f32) -> Node<U> {
        Node {
            inner: NodeValue::Offset {
                offset_x: amount,
                offset_y: 0.,
                element: Box::new(self.inner),
            },
        }
    }
    /// Offsets the node along the y axis.
    /// This is an absolute offset that simply shifts nodes away from their calculated position
    /// This won't impact layout besides child nodes also being offset
    pub fn offset_y(self, amount: f32) -> Node<U> {
        Node {
            inner: NodeValue::Offset {
                offset_x: 0.,
                offset_y: amount,
                element: Box::new(self.inner),
            },
        }
    }
    /// Offsets the node along the x & y axis.
    /// This is an absolute offset that simply shifts nodes away from their calculated position
    /// This won't impact layout besides child nodes also being offset
    pub fn offset(self, offset_x: f32, offset_y: f32) -> Node<U> {
        Node {
            inner: NodeValue::Offset {
                offset_x,
                offset_y,
                element: Box::new(self.inner),
            },
        }
    }
    /// Specifies an explicit width for a node
    pub fn width(self, width: f32) -> Self {
        self.wrap_or_update_explicit(|options| {
            options.width = width.into();
            options.x_relative = false;
        })
    }
    /// Specifies an explicit height for a node
    pub fn height(self, height: f32) -> Self {
        self.wrap_or_update_explicit(|options| {
            options.height = height.into();
            options.y_relative = false;
        })
    }
    /// Specifies an explicit width for a node as a fraction of the available width
    pub fn relative_width(self, ratio: f32) -> Self {
        self.wrap_or_update_explicit(|options| {
            options.width = ratio.into();
            options.x_relative = true;
        })
    }
    /// Specifies an explicit height for a node as a fraction of the available height
    pub fn relative_height(self, ratio: f32) -> Self {
        self.wrap_or_update_explicit(|options| {
            options.height = ratio.into();
            options.y_relative = true;
        })
    }
    /// Specifies a lower bound on a node's width
    ///
    /// When used inside a [crate::nodes::row] this will not impact row layout.
    /// If you'd like to impact row layout use [Node::width] or [Node::relative_width]
    pub fn min_width(self, width: f32) -> Self {
        self.wrap_or_update_explicit(|options| {
            options.width_min = width.into();
        })
    }
    /// Specifies a lower bound on a node's height
    ///
    /// When used inside a [crate::nodes::column] this will not impact column layout.
    /// If you'd like to impact column layout use [Node::height] or [Node::relative_height]
    pub fn min_height(self, height: f32) -> Self {
        self.wrap_or_update_explicit(|options| {
            options.height_min = height.into();
        })
    }
    /// Specifies an upper bound on a node's width
    ///
    /// When used inside a [crate::nodes::row] this will not impact row layout.
    /// If you'd like to impact row layout use [Node::width] or [Node::relative_width]
    pub fn max_width(self, width: f32) -> Self {
        self.wrap_or_update_explicit(|options| {
            options.width_max = width.into();
        })
    }
    /// Specifies an upper bound on a node's height
    ///
    /// When used inside a [crate::nodes::column] this will not impact column layout.
    /// If you'd like to impact column layout use [Node::height] or [Node::relative_height]
    pub fn max_height(self, height: f32) -> Self {
        self.wrap_or_update_explicit(|options| {
            options.height_max = height.into();
        })
    }
    /// Specifies an alignment along the x axis.
    ///
    /// This will only have an effect if the node is constrained to be smaller than the area that is available
    /// Otherwise, there's no wiggle room!
    pub fn x_align(self, align: XAlign) -> Self {
        self.wrap_or_update_explicit(|options| {
            options.x_align = align;
        })
    }
    /// Specifies an alignment along the y axis.
    ///
    /// This will only have an effect if the node is constrained to be smaller than the area that is available.
    /// Otherwise, there's no wiggle room!
    pub fn y_align(self, align: YAlign) -> Self {
        self.wrap_or_update_explicit(|options| {
            options.y_align = align;
        })
    }
    /// Specifies an alignment along both the x & y axis.
    ///
    /// This will only have an effect if the node is constrained along the axis to be smaller than the area that is available.
    /// Otherwise, there's no wiggle room!
    pub fn align(self, align: Align) -> Self {
        self.wrap_or_update_explicit(|options| match align {
            Align::TopLeading => {
                options.y_align = YAlign::Top;
                options.x_align = XAlign::Leading;
            }
            Align::TopCenter => {
                options.y_align = YAlign::Top;
                options.x_align = XAlign::Center;
            }
            Align::TopTrailing => {
                options.y_align = YAlign::Top;
                options.x_align = XAlign::Trailing;
            }
            Align::CenterTrailing => {
                options.y_align = YAlign::Center;
                options.x_align = XAlign::Trailing;
            }
            Align::BottomTrailing => {
                options.y_align = YAlign::Bottom;
                options.x_align = XAlign::Trailing;
            }
            Align::BottomCenter => {
                options.y_align = YAlign::Bottom;
                options.x_align = XAlign::Center;
            }
            Align::BottomLeading => {
                options.y_align = YAlign::Bottom;
                options.x_align = XAlign::Leading;
            }
            Align::CenterLeading => {
                options.y_align = YAlign::Center;
                options.x_align = XAlign::Leading;
            }
            Align::CenterCenter => {
                options.y_align = YAlign::Center;
                options.x_align = XAlign::Center;
            }
        })
    }

    fn wrap_or_update_explicit(mut self, update: impl Fn(&mut Size)) -> Self {
        if let NodeValue::Explicit {
            ref mut options, ..
        } = self.inner
        {
            update(options);
            self
        } else {
            let mut options = Size::new();
            update(&mut options);
            Node {
                inner: NodeValue::Explicit {
                    options,
                    element: Box::new(self.inner),
                },
            }
        }
    }
}
