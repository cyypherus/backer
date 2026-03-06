use std::fmt::Debug;
use std::rc::Rc;

use crate::tree::TreeNode;

pub(crate) type DrawFn<'a, D, S> = Box<dyn FnOnce(Area, &mut S) -> Vec<D> + 'a>;
pub(crate) type DimensionFn<'a, S> = Option<Box<dyn Fn(f32, &mut S) -> f32 + 'a>>;

/// The tree structure which represents a layout.
///
/// Use methods in [`crate::nodes`] to create nodes.
///
/// Call `Layout::draw` to produce laid-out values to be rendered
///
/// # Example
///
/// ```
/// use backer::nodes::*;
/// use backer::{Layout, Area};
///
/// enum View {
///     Button(Button),
///     Label(Label),
/// }
/// struct Button { area: Area }
/// struct Label { area: Area, text: &'static str }
///
/// let mut layout = column(vec![
///     draw(|area, _: &mut ()| vec![View::Button(Button { area })]).width(100.0).height(50.0),
///     draw(|area, _: &mut ()| vec![View::Label(Label { area, text: "Hello world!" })]).width(100.0).height(50.0),
/// ]);
///
/// let views = layout.draw(Area::new(0.0, 0.0, 200.0, 150.0), &mut ());
/// // views is now a Vec of Button values with their laid-out areas
/// ```
pub struct Layout<'a, D, S = ()> {
    pub(crate) layout: LayoutType<'a, D, S>,
    pub(crate) constraints: Constraints,
    pub(crate) dynamic_constraints: DynamicConstraints<'a, S>,
    pub(crate) layer: Option<i32>,
    pub(crate) resolved: Option<Constraints>,
    pub(crate) allocated: Option<Area>,
    pub(crate) children: Vec<Layout<'a, D, S>>,
}

impl<'a, D, S> Layout<'a, D, S> {
    pub(crate) fn constraints(&self) -> Constraints {
        self.resolved.unwrap_or(self.constraints)
    }
}

impl<'a, D: 'a, S: 'a> Layout<'a, D, S> {
    /// Transforms the draw output of every node in the tree from type `D` to type `B`.
    pub fn map<B: 'a>(self, f: impl Fn(D) -> B + 'a) -> Layout<'a, B, S> {
        self.map_rc(Rc::new(f))
    }

    fn map_rc<B: 'a>(self, f: Rc<dyn Fn(D) -> B + 'a>) -> Layout<'a, B, S> {
        Layout {
            layout: match self.layout {
                LayoutType::Draw(Some(draw_fn)) => {
                    let f = f.clone();
                    LayoutType::Draw(Some(Box::new(move |area, s| {
                        draw_fn(area, s).into_iter().map(|draw| f(draw)).collect()
                    })))
                }
                LayoutType::Draw(None) => LayoutType::Draw(None),
                LayoutType::Column {
                    spacing,
                    x_align,
                    y_align,
                } => LayoutType::Column {
                    spacing,
                    x_align,
                    y_align,
                },
                LayoutType::Row {
                    spacing,
                    x_align,
                    y_align,
                } => LayoutType::Row {
                    spacing,
                    x_align,
                    y_align,
                },
                LayoutType::Stack { x_align, y_align } => LayoutType::Stack { x_align, y_align },
                LayoutType::Padding {
                    leading,
                    trailing,
                    top,
                    bottom,
                } => LayoutType::Padding {
                    leading,
                    trailing,
                    top,
                    bottom,
                },
                LayoutType::Offset { x, y } => LayoutType::Offset { x, y },
                LayoutType::Space => LayoutType::Space,
                LayoutType::Empty => LayoutType::Empty,
            },
            constraints: self.constraints,
            dynamic_constraints: self.dynamic_constraints,
            layer: self.layer,
            resolved: self.resolved,
            allocated: self.allocated,
            children: self
                .children
                .into_iter()
                .map(|c| c.map_rc(f.clone()))
                .collect(),
        }
    }
}

impl<'a, D, S> TreeNode for Layout<'a, D, S> {
    fn children_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Self> {
        self.children.iter_mut()
    }
}

pub(crate) struct DynamicConstraints<'a, S = ()> {
    pub(crate) width: DimensionFn<'a, S>,
    pub(crate) height: DimensionFn<'a, S>,
}

impl<'a, S> Default for DynamicConstraints<'a, S> {
    fn default() -> Self {
        Self {
            width: None,
            height: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Constraints {
    pub(crate) width: AxisConstraint,
    pub(crate) height: AxisConstraint,
    pub(crate) expand_x: bool,
    pub(crate) expand_y: bool,
    pub(crate) transparent_x: bool,
    pub(crate) transparent_y: bool,
    pub(crate) x_align: Option<XAlign>,
    pub(crate) y_align: Option<YAlign>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct AxisConstraint {
    pub(crate) lower: Option<f32>,
    pub(crate) upper: Option<f32>,
}

impl AxisConstraint {
    pub(crate) fn new(lower: Option<f32>, upper: Option<f32>) -> Self {
        assert!(Self::check_constraints(lower, upper));
        Self { lower, upper }
    }

    pub(crate) fn none() -> Self {
        Self::new(None, None)
    }

    pub(crate) fn combine_adjacent_priority(self, other: Self) -> Self {
        let lower = match (self.lower, other.lower) {
            (None, None) => None,
            (None, Some(a)) | (Some(a), None) => Some(a),
            (Some(bound_a), Some(bound_b)) => Some(bound_a.max(bound_b)),
        };
        let upper = match (self.upper, other.upper) {
            (None, None) => None,
            (None, Some(_)) | (Some(_), None) => None,
            (Some(bound_a), Some(bound_b)) => Some(bound_a.max(bound_b)),
        };
        AxisConstraint::new(lower, upper)
    }

    pub(crate) fn combine_sum(self, other: Self, spacing: f32) -> Self {
        let lower = match (self.lower, other.lower) {
            (None, None) => None,
            (None, Some(bound)) | (Some(bound), None) => Some(bound + spacing),
            (Some(bound_a), Some(bound_b)) => Some(bound_a + bound_b + spacing),
        };
        let upper = match (self.upper, other.upper) {
            (None, None) => None,
            (None, Some(_)) | (Some(_), None) => None,
            (Some(bound_a), Some(bound_b)) => Some(bound_a + bound_b + spacing),
        };
        AxisConstraint::new(lower, upper)
    }

    pub(crate) fn combine_parent_child(&self, child: Option<Self>) -> Self {
        let Some(child) = child else { return *self };
        // If child lower bound is higher than parent upper bound
        // the parent's upper bound should be used as the upper AND lower bound
        // This behavior applies to an upper bound being lower than a lower bound
        // in the same way. The parent's bounds should always override the child's
        let lower = match (self.lower, self.upper, child.lower) {
            (_, Some(parent_upper), Some(child_lower)) if child_lower > parent_upper => {
                Some(parent_upper)
            }
            (Some(parent_lower), _, _) => Some(parent_lower),
            (None, _, Some(child_lower)) => Some(child_lower),
            _ => None,
        };

        let upper = match (self.upper, self.lower, child.upper) {
            (Some(parent_upper), _, _) => Some(parent_upper),
            (None, Some(parent_lower), Some(child_upper)) if child_upper < parent_lower => {
                Some(parent_lower)
            }
            (None, _, Some(child_upper)) => Some(child_upper),
            _ => None,
        };

        AxisConstraint::new(lower, upper)
    }

    fn check_constraints(lower: Option<f32>, upper: Option<f32>) -> bool {
        if let (Some(lower_unwrapped), Some(upper_unwrapped)) = (lower, upper) {
            lower_unwrapped <= upper_unwrapped
        } else {
            true
        }
    }
}
impl Constraints {
    pub(crate) fn should_expand_x(&self) -> bool {
        self.expand_x || self.width.upper.is_none()
    }

    pub(crate) fn should_expand_y(&self) -> bool {
        self.expand_y || self.height.upper.is_none()
    }

    pub(crate) fn combine_parent_child(&self, child: Option<Self>) -> Self {
        Constraints {
            width: self.width.combine_parent_child(child.map(|c| c.width)),
            height: self.height.combine_parent_child(child.map(|c| c.height)),
            ..*self
        }
    }
}

impl Default for Constraints {
    fn default() -> Self {
        Constraints {
            width: AxisConstraint::none(),
            height: AxisConstraint::none(),
            expand_x: false,
            expand_y: false,
            transparent_x: false,
            transparent_y: false,
            x_align: None,
            y_align: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum XAlign {
    Leading,
    Center,
    Trailing,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum YAlign {
    Top,
    Center,
    Bottom,
}

/// An alignment along the X and/or Y axis
#[derive(Debug, Clone, Copy)]
pub enum Align {
    /// Aligns to the top
    Top,
    /// Aligns to the vertical center
    CenterY,
    /// Aligns to the bottom
    Bottom,

    /// Aligns to the left in LTR layout
    Leading,
    /// Aligns to the horizontal center
    CenterX,
    /// Aligns to the right in LTR layout
    Trailing,

    /// Aligns to the top left in LTR layout
    TopLeading,
    /// Aligns to the top center
    TopCenter,
    /// Aligns to the top right in LTR layout
    TopTrailing,
    /// Aligns to the middle right in LTR layout
    CenterTrailing,
    /// Aligns to the bottom right in LTR layout
    BottomTrailing,
    /// Aligns to the bottom middle
    BottomCenter,
    /// Aligns to the bottom left in LTR layout
    BottomLeading,
    /// Aligns to the middle left in LTR layout
    CenterLeading,
    /// Aligns to the center in LTR layout - the default alignment
    CenterCenter,
}

impl Align {
    pub(crate) fn axis_aligns(&self) -> (Option<XAlign>, Option<YAlign>) {
        match self {
            Align::TopLeading => (Some(XAlign::Leading), Some(YAlign::Top)),
            Align::TopCenter => (Some(XAlign::Center), Some(YAlign::Top)),
            Align::TopTrailing => (Some(XAlign::Trailing), Some(YAlign::Top)),
            Align::CenterTrailing => (Some(XAlign::Trailing), Some(YAlign::Center)),
            Align::BottomTrailing => (Some(XAlign::Trailing), Some(YAlign::Bottom)),
            Align::BottomCenter => (Some(XAlign::Center), Some(YAlign::Bottom)),
            Align::BottomLeading => (Some(XAlign::Leading), Some(YAlign::Bottom)),
            Align::CenterLeading => (Some(XAlign::Leading), Some(YAlign::Center)),
            Align::CenterCenter => (Some(XAlign::Center), Some(YAlign::Center)),
            Align::Top => (None, Some(YAlign::Top)),
            Align::CenterY => (None, Some(YAlign::Center)),
            Align::Bottom => (None, Some(YAlign::Bottom)),
            Align::Leading => (Some(XAlign::Leading), None),
            Align::CenterX => (Some(XAlign::Center), None),
            Align::Trailing => (Some(XAlign::Trailing), None),
        }
    }
}

/// An allocation of space as a rectangle
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Area {
    /// Origin - usually the left-most X
    pub x: f32,
    /// Origin - usually the upper-most Y
    pub y: f32,
    /// Available width, starting at `x`
    pub width: f32,
    /// Available height, starting at `y`
    pub height: f32,
}

impl Area {
    /// Creates a new [`Area`].
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
    pub(crate) fn constrained(
        &self,
        constraints: &Option<Constraints>,
        contextual_x_align: Option<XAlign>,
        contextual_y_align: Option<YAlign>,
    ) -> Area {
        let Some(constraints) = constraints else {
            return *self;
        };

        let width = match (
            constraints.width.lower,
            if constraints.expand_x {
                None
            } else {
                constraints.width.upper
            },
        ) {
            (None, None) => self.width,
            (None, Some(upper)) => self.width.min(upper),
            (Some(lower), None) => self.width.max(lower),
            (Some(lower), Some(upper)) => self.width.clamp(lower, upper.max(lower)),
        };
        let height = match (
            constraints.height.lower,
            if constraints.expand_y {
                None
            } else {
                constraints.height.upper
            },
        ) {
            (None, None) => self.height,
            (None, Some(upper)) => self.height.min(upper),
            (Some(lower), None) => self.height.max(lower),
            (Some(lower), Some(upper)) => self.height.clamp(lower, upper.max(lower)),
        };

        let x_align = contextual_x_align
            .or(constraints.x_align)
            .unwrap_or(XAlign::Center);
        let y_align = contextual_y_align
            .or(constraints.y_align)
            .unwrap_or(YAlign::Center);

        let x = match x_align {
            XAlign::Leading => self.x,
            XAlign::Trailing => self.x + (self.width - width),
            XAlign::Center => self.x + (self.width * 0.5) - (width * 0.5),
        };

        let y = match y_align {
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

pub enum LayoutType<'a, D, S = ()> {
    Draw(Option<DrawFn<'a, D, S>>),
    Column {
        spacing: f32,
        x_align: Option<XAlign>,
        y_align: Option<YAlign>,
    },
    Row {
        spacing: f32,
        x_align: Option<XAlign>,
        y_align: Option<YAlign>,
    },
    Stack {
        x_align: Option<XAlign>,
        y_align: Option<YAlign>,
    },
    Padding {
        leading: f32,
        trailing: f32,
        top: f32,
        bottom: f32,
    },
    Offset {
        x: f32,
        y: f32,
    },
    Space,
    Empty,
}
