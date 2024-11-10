use crate::{drawable::Drawable, layout::NodeValue, models::*, node_cache::NodeCache, Node};
use std::rc::Rc;

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

/// Creates a vertical sequence of elements
///
#[doc = container_doc!()]
pub fn column<State>(elements: Vec<Node<State>>) -> Node<State> {
    Node {
        inner: NodeValue::Column {
            elements: filter_empty(ungroup(elements)),
            spacing: 0.,
            align: None,
            off_axis_align: None,
        },
    }
}
/// Creates multiple elements at once.
/// Has no impact on layout.
/// Just a convenience for adding a `Vec` of elements to a sequence node inline.
/// ```rust
/// use backer::*;
/// use backer::models::*;
/// use backer::nodes::*;
/// use backer::traits::Scopable;
///
/// column::<(), ()>(vec![
///     empty(),
///     group(
///         (0..5)
///             .into_iter()
///             .map(|i| empty())
///             .collect()
///     ),
/// ]);
/// ```
pub fn group<State>(elements: Vec<Node<State>>) -> Node<State> {
    Node {
        inner: NodeValue::Group(filter_empty(ungroup(elements))),
    }
}
/// Creates a vertical sequence of elements with the specified spacing between each element.
///
#[doc = container_doc!()]
pub fn column_spaced<State>(spacing: f32, elements: Vec<Node<State>>) -> Node<State> {
    Node {
        inner: NodeValue::Column {
            elements: filter_empty(ungroup(elements)),
            spacing,
            align: None,
            off_axis_align: None,
        },
    }
}
/// Creates a horizontal sequence of elements
///
#[doc = container_doc!()]
pub fn row<State>(elements: Vec<Node<State>>) -> Node<State> {
    Node {
        inner: NodeValue::Row {
            elements: filter_empty(ungroup(elements)),
            spacing: 0.,
            align: None,
            off_axis_align: None,
        },
    }
}
/// Creates a horizontal sequence of elements with the specified spacing between each element.
///
#[doc = container_doc!()]
pub fn row_spaced<State>(spacing: f32, elements: Vec<Node<State>>) -> Node<State> {
    Node {
        inner: NodeValue::Row {
            elements: filter_empty(ungroup(elements)),
            spacing,
            align: None,
            off_axis_align: None,
        },
    }
}
/// Creates a sequence of elements to be laid out on top of each other.
///
#[doc = container_doc!()]
pub fn stack<State>(elements: Vec<Node<State>>) -> Node<State> {
    Node {
        inner: NodeValue::Stack {
            elements: filter_empty(ungroup(elements)),
            x_align: None,
            y_align: None,
        },
    }
}
/// Creates a node that can be drawn
/// This node is the point of integration with the UI library of your choice.
/// ```rust
/// use backer::*;
/// use backer::models::*;
/// use backer::nodes::*;
/// use backer::traits::Scopable;
///
/// struct MyState {}
/// fn my_drawable(state: &mut MyState) -> Node<MyState> {
///  draw(move |area: Area, state: &mut MyState| {
///    // The `area` parameter is the space alotted for your view after layout is calculated
///    // The `state` parameter is *your* mutable state that you pass when you call `draw`.
///    // This closure should draw UI based on the alotted area or update your state so that drawing can be performed later.
///  })
///}
/// ```
pub fn draw<State>(drawable: impl Fn(Area, &mut State) + 'static) -> Node<State> {
    Node {
        inner: NodeValue::Draw(Drawable {
            area: Area::default(),
            draw: Rc::new(move |area, state| drawable(area, state)),
        }),
    }
}
/// Creates an empty space which is laid out the same as any other node.
///
/// To add spacing between each item in a row or column you can also use
/// [`row_spaced`] & [`column_spaced`]
pub fn space<State>() -> Node<State> {
    Node {
        inner: NodeValue::Space,
    }
}
/// Nothing! This will not have any impact on layout - useful for conditionally
/// adding elements to a layout in the case where nothing should be added.
pub fn empty<State>() -> Node<State> {
    Node {
        inner: NodeValue::Empty,
    }
}
/// Return nodes based on available area
///
/// This node comes with caveats! Constraints within an area reader **cannot** expand the area reader itself.
/// If it could - it would create cyclical dependency which may be impossible to resolve.
pub fn area_reader<State>(func: impl Fn(Area, &mut State) -> Node<State> + 'static) -> Node<State> {
    Node {
        inner: NodeValue::AreaReader {
            read: Rc::new(func),
        },
    }
}
/// Return nodes based on available area
///
/// This node comes with caveats! Constraints within an area reader **cannot** expand the area reader itself.
/// If it could - it would create cyclical dependency which may be impossible to resolve.
pub fn area_reader_with<State>(
    func: impl Fn(Area, &mut State) -> Node<State> + 'static,
) -> Node<State> {
    Node {
        inner: NodeValue::AreaReader {
            read: Rc::new(func),
        },
    }
}
fn ungroup<State>(elements: Vec<Node<State>>) -> Vec<NodeCache<State>> {
    elements
        .into_iter()
        .flat_map(|el| {
            if let NodeValue::Group(els) = el.inner {
                els
            } else {
                vec![el.inner]
                    .into_iter()
                    .map(|el| NodeCache::new(el))
                    .collect()
            }
        })
        .collect()
}

fn filter_empty<State>(elements: Vec<NodeCache<State>>) -> Vec<NodeCache<State>> {
    elements
        .into_iter()
        .filter(|el| {
            if let NodeValue::Empty = el.kind {
                return false;
            }
            true
        })
        .collect()
}
