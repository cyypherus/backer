use std::rc::Rc;

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

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum XAlign {
    Leading,
    Center,
    Trailing,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum YAlign {
    Top,
    Center,
    Bottom,
}

impl Align {
    pub(crate) fn axis_aligns(&self) -> (Option<XAlign>, Option<YAlign>) {
        let (x_align, y_align) = match self {
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
        };
        (x_align, y_align)
    }
}

/// An allocation of screen space as a rectangle
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
    #[allow(unused)]
    pub(crate) fn zero() -> Self {
        Self {
            x: 0.,
            y: 0.,
            width: 0.,
            height: 0.,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Padding {
    pub(crate) leading: f32,
    pub(crate) trailing: f32,
    pub(crate) top: f32,
    pub(crate) bottom: f32,
}

type DimensionFn<State> = Option<Rc<dyn Fn(f32, &mut State) -> f32>>;

pub(crate) struct Size<State> {
    pub(crate) width_min: Option<f32>,
    pub(crate) width_max: Option<f32>,
    pub(crate) height_min: Option<f32>,
    pub(crate) height_max: Option<f32>,
    pub(crate) x_align: Option<XAlign>,
    pub(crate) y_align: Option<YAlign>,
    pub(crate) aspect: Option<f32>,
    pub(crate) dynamic_height: DimensionFn<State>,
    pub(crate) dynamic_width: DimensionFn<State>,
    pub(crate) expand_x: bool,
    pub(crate) expand_y: bool,
}

impl<State> Clone for Size<State> {
    fn clone(&self) -> Self {
        Self {
            width_min: self.width_min,
            width_max: self.width_max,
            height_min: self.height_min,
            height_max: self.height_max,
            x_align: self.x_align,
            y_align: self.y_align,
            aspect: self.aspect,
            dynamic_height: self.dynamic_height.clone(),
            dynamic_width: self.dynamic_width.clone(),
            expand_x: self.expand_x,
            expand_y: self.expand_y,
        }
    }
}

impl<State> std::fmt::Debug for Size<State> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Size")
            .field("width_min", &self.width_min)
            .field("width_max", &self.width_max)
            .field("height_min", &self.height_min)
            .field("height_max", &self.height_max)
            .field("x_align", &self.x_align)
            .field("y_align", &self.y_align)
            .field("aspect", &self.aspect)
            .field("dynamic_height", &"<function>")
            .field("dynamic_width", &"<function>")
            .field("expand_x", &self.expand_x)
            .field("expand_y", &self.expand_y)
            .finish()
    }
}

impl<State> Default for Size<State> {
    fn default() -> Self {
        Self::new()
    }
}

impl<State> Size<State> {
    /// Creates a default size object to add constraints to
    pub(crate) fn new() -> Self {
        Size {
            width_min: None,
            width_max: None,
            height_min: None,
            height_max: None,
            x_align: None,
            y_align: None,
            aspect: None,
            dynamic_height: None,
            dynamic_width: None,
            expand_x: false,
            expand_y: false,
        }
    }
}
