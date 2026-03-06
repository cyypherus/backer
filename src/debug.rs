use crate::{
    Layout,
    types::{DynamicConstraints, LayoutType},
};
use std::fmt::Debug;

impl<'a, A, S> Debug for Layout<'a, A, S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Layout")
            .field("layout", &format!("{:?}", &self.layout))
            .field("constraints", &self.constraints)
            .field("dynamic_constraints", &self.dynamic_constraints)
            .field("resolved", &self.resolved)
            .field("allocated", &self.allocated)
            .field("children", &self.children)
            .finish()
    }
}

impl<'a, A, S> Debug for LayoutType<'a, A, S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LayoutType::Draw(_) => f.debug_tuple("Draw").field(&"<function>").finish(),
            LayoutType::Column {
                spacing,
                x_align,
                y_align,
            } => f
                .debug_struct("Column")
                .field("spacing", spacing)
                .field("x_align", x_align)
                .field("y_align", y_align)
                .finish(),
            LayoutType::Row {
                spacing,
                x_align,
                y_align,
            } => f
                .debug_struct("Row")
                .field("spacing", spacing)
                .field("x_align", x_align)
                .field("y_align", y_align)
                .finish(),
            LayoutType::Stack { x_align, y_align } => f
                .debug_struct("Stack")
                .field("x_align", x_align)
                .field("y_align", y_align)
                .finish(),
            LayoutType::Padding {
                leading,
                trailing,
                top,
                bottom,
            } => f
                .debug_struct("Padding")
                .field("leading", leading)
                .field("trailing", trailing)
                .field("top", top)
                .field("bottom", bottom)
                .finish(),
            LayoutType::Offset { x, y } => f
                .debug_struct("Offset")
                .field("x", x)
                .field("y", y)
                .finish(),
            LayoutType::Space => write!(f, "Space"),
            LayoutType::Empty => write!(f, "Empty"),
            LayoutType::Coupled { over } => f.debug_struct("Coupled").field("over", over).finish(),
            LayoutType::MultipleDraw(_) => write!(f, "MultipleDraw"),
        }
    }
}

impl<'a, S> Debug for DynamicConstraints<'a, S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DynamicConstraints")
            .field("width", &"<function>")
            .field("height", &"<function>")
            .finish()
    }
}
