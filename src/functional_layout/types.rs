use std::fmt::Debug;

use crate::functional_layout::tree::IntoTreeTrait;

type DimensionFn = Option<&'static dyn Fn(f32) -> f32>;

#[derive(Debug, Clone)]
pub struct Layout<A> {
    pub(crate) layout: LayoutType<A>,
    pub(crate) constraints: Constraints,
    pub(crate) dynamic_constraints: DynamicConstraints,
    pub(crate) resolved: Option<Constraints>,
    pub(crate) allocated: Option<Area>,
    pub(crate) children: Vec<Layout<A>>,
}

impl<A> IntoTreeTrait for Layout<A> {
    fn into_data_and_children(mut self) -> (Self, impl DoubleEndedIterator<Item = Self>) {
        let children = std::mem::take(&mut self.children);
        (self, children.into_iter())
    }
}

#[derive(Clone, Default)]
pub(crate) struct DynamicConstraints {
    pub(crate) width: DimensionFn,
    pub(crate) height: DimensionFn,
}

impl Debug for DynamicConstraints {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DynamicConstraints")
            .field("width", &"<function>")
            .field("height", &"<function>")
            .finish()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Constraints {
    pub(crate) width: AxisConstraint,
    pub(crate) height: AxisConstraint,
    pub(crate) expand_x: bool,
    pub(crate) expand_y: bool,
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
            width: AxisConstraint {
                lower: self.width.lower.or(child.and_then(|c| c.width.lower)),
                upper: self.width.upper.or(child.and_then(|c| c.width.upper)),
            },
            height: AxisConstraint {
                lower: self.height.lower.or(child.and_then(|c| c.height.lower)),
                upper: self.height.upper.or(child.and_then(|c| c.height.upper)),
            },
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

#[derive(Debug, Clone, Copy)]
pub enum Align {
    Top,
    CenterY,
    Bottom,
    Leading,
    CenterX,
    Trailing,
    TopLeading,
    TopCenter,
    TopTrailing,
    CenterTrailing,
    BottomTrailing,
    BottomCenter,
    BottomLeading,
    CenterLeading,
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

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Area {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Area {
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

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutType<A> {
    Draw(A),
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
    Coupled {
        over: bool,
    },
}
