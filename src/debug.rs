use crate::layout::NodeValue;
use std::fmt;

impl<State> fmt::Debug for NodeValue<State> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NodeValue::Padding { amounts, element } => f
                .debug_struct("Padding")
                .field("amounts", amounts)
                .field("element", element)
                .finish(),
            NodeValue::Column {
                elements,
                spacing,
                align,
                off_axis_align,
            } => f
                .debug_struct("Column")
                .field("elements", elements)
                .field("spacing", spacing)
                .field("align", align)
                .field("off_axis_align", off_axis_align)
                .finish(),
            NodeValue::Row {
                elements,
                spacing,
                align,
                off_axis_align,
            } => f
                .debug_struct("Row")
                .field("elements", elements)
                .field("spacing", spacing)
                .field("align", align)
                .field("off_axis_align", off_axis_align)
                .finish(),
            NodeValue::Stack {
                elements,
                x_align,
                y_align,
            } => f
                .debug_struct("Stack")
                .field("elements", elements)
                .field("x_align", x_align)
                .field("y_align", y_align)
                .finish(),
            NodeValue::Group(elements) => f.debug_tuple("Group").field(elements).finish(),
            NodeValue::Offset {
                offset_x,
                offset_y,
                element,
            } => f
                .debug_struct("Offset")
                .field("offset_x", offset_x)
                .field("offset_y", offset_y)
                .field("element", element)
                .finish(),
            NodeValue::Draw(drawable) => f.debug_tuple("Draw").field(drawable).finish(),
            NodeValue::Explicit { options, element } => f
                .debug_struct("Explicit")
                .field("options", &options)
                .field("element", element)
                .finish(),
            NodeValue::Space => write!(f, "Space"),
            NodeValue::Empty => write!(f, "Empty"),
            NodeValue::AreaReader { .. } => write!(f, "WidthReader"),
            NodeValue::Coupled {
                element,
                coupled,
                over,
            } => f
                .debug_struct("Coupled")
                .field("element", &element)
                .field("coupled", coupled)
                .field("over", over)
                .finish(),
            NodeValue::NodeTrait { .. } => f.debug_struct("NodeTrait").finish(),
            NodeValue::Visibility { visible, element } => f
                .debug_struct("Visibility")
                .field("element", &element)
                .field("visible", visible)
                .finish(),
        }
    }
}
