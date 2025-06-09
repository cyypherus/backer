use crate::{
    drawable::{DrawableNode, SomeDrawable},
    layout::NodeValue,
    models::*,
    node_cache::NodeCache,
    scoper::{OptionScoper, OwnedScoper, Scoper},
    traits::Drawable,
    Node,
};

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
pub fn column<'n, T, U>(elements: Vec<impl Into<Node<'n, T, U>>>) -> Node<'n, T, U> {
    Node {
        inner: NodeValue::Column {
            elements: filter_empty(ungroup(convert_into(elements))),
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
///
/// column::<()>(vec![
///     empty(),
///     group(
///         (0..5)
///             .into_iter()
///             .map(|i| empty())
///             .collect()
///     ),
/// ]);
/// ```
pub fn group<'n, T, U>(elements: Vec<impl Into<Node<'n, T, U>>>) -> Node<'n, T, U> {
    Node {
        inner: NodeValue::Group(filter_empty(ungroup(convert_into(elements)))),
    }
}
/// Creates a vertical sequence of elements with the specified spacing between each element.
///
#[doc = container_doc!()]
pub fn column_spaced<'n, T, U>(
    spacing: f32,
    elements: Vec<impl Into<Node<'n, T, U>>>,
) -> Node<'n, T, U> {
    Node {
        inner: NodeValue::Column {
            elements: filter_empty(ungroup(convert_into(elements))),
            spacing,
            align: None,
            off_axis_align: None,
        },
    }
}
/// Creates a horizontal sequence of elements
///
#[doc = container_doc!()]
pub fn row<'n, T, U>(elements: Vec<impl Into<Node<'n, T, U>>>) -> Node<'n, T, U> {
    Node {
        inner: NodeValue::Row {
            elements: filter_empty(ungroup(convert_into(elements))),
            spacing: 0.,
            align: None,
            off_axis_align: None,
        },
    }
}
/// Creates a horizontal sequence of elements with the specified spacing between each element.
///
#[doc = container_doc!()]
pub fn row_spaced<'n, T, U>(
    spacing: f32,
    elements: Vec<impl Into<Node<'n, T, U>>>,
) -> Node<'n, T, U> {
    Node {
        inner: NodeValue::Row {
            elements: filter_empty(ungroup(convert_into(elements))),
            spacing,
            align: None,
            off_axis_align: None,
        },
    }
}
/// Creates a sequence of elements to be laid out on top of each other.
///
#[doc = container_doc!()]
pub fn stack<'n, T, U>(elements: Vec<impl Into<Node<'n, T, U>>>) -> Node<'n, T, U> {
    Node {
        inner: NodeValue::Stack {
            elements: filter_empty(ungroup(convert_into(elements))),
            x_align: None,
            y_align: None,
        },
    }
}
/// Creates a node that can be drawn.
///
/// This node is the point of integration with the UI library of your choice.
/// ```rust
/// use backer::*;
/// use backer::models::*;
/// use backer::nodes::*;
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
pub fn draw<'nodes, T, U>(
    drawable_fn: impl Fn(Area, &mut T, &mut U) + 'static,
) -> Node<'nodes, T, U> {
    Node {
        inner: NodeValue::Draw(DrawableNode {
            area: Area::default(),
            drawable: SomeDrawable::Fn(Box::new(drawable_fn)),
        }),
    }
}
/// Creates a node that can be drawn using an object which implements the `Drawable` trait
/// (or the `TransitionDrawable` trait)
///
/// See [`draw`]
pub fn draw_object<'nodes, T, U>(drawable: impl Drawable<T, U> + 'nodes) -> Node<'nodes, T, U> {
    Node {
        inner: NodeValue::Draw(DrawableNode {
            area: Area::default(),
            drawable: SomeDrawable::Object(Box::new(drawable)),
        }),
    }
}

/// Creates an empty space which is laid out the same as any other node.
///
/// To add spacing between each item in a row or column you can also use
/// [`row_spaced`] & [`column_spaced`]
pub fn space<'nodes, T, U>() -> Node<'nodes, T, U> {
    Node {
        inner: NodeValue::Space,
    }
}
/// Nothing! This will not have any impact on layout - useful for conditionally
/// adding elements to a layout in the case where nothing should be added.
pub fn empty<'nodes, T, U>() -> Node<'nodes, T, U> {
    Node {
        inner: NodeValue::Empty,
    }
}
/// Returns nodes based on available area
///
/// This node comes with caveats! Constraints within an area reader **cannot** expand the area reader itself.
/// If it could - it would create cyclical dependency which may be impossible to resolve.
pub fn area_reader<'nodes, T, U>(
    func: impl Fn(Area, &mut T, &mut U) -> Node<'nodes, T, U> + 'static,
) -> Node<'nodes, T, U> {
    Node {
        inner: NodeValue::AreaReader {
            read: Box::new(func),
        },
    }
}
/// Returns a dynamic set of nodes based on state
pub fn dynamic<'nodes, T, U>(
    func: impl Fn(&'_ mut T, &'_ mut U) -> Node<'nodes, T, U> + 'nodes,
) -> Node<'nodes, T, U> {
    Node {
        inner: NodeValue::Dynamic {
            element: Box::new(func),
            computed: None,
        },
    }
}
/// Scopes state to some derived subset for all children of this node
///
///```rust
/// use backer::*;
/// use backer::models::*;
/// use backer::nodes::*;
///
/// struct A {
///     b: bool,
/// }
/// let layout = dynamic(|_: &mut A| {
///     stack(vec![
///         scope(
///             // This closure selects which state to scope to
///             |a: &mut A| &mut a.b,
///             // These nodes now have direct access to only the boolean
///             draw(|_, b: &mut bool| *b = !*b),
///         ),
///     ])
/// });
///```
pub fn scope<'nodes, T, U, ScopedT: 'nodes, ScopedU: 'nodes>(
    scope_t: impl Fn(&mut T) -> &mut ScopedT + 'nodes,
    scope_u: impl Fn(&mut U) -> &mut ScopedU + 'nodes,
    node: impl Into<Node<'nodes, ScopedT, ScopedU>>,
) -> Node<'nodes, T, U> {
    Node {
        inner: NodeValue::NodeTrait {
            element: Box::new(Scoper {
                scope_t,
                scope_u,
                node: node.into(),
            }),
        },
    }
}
/// Scopes state to some derived *optional* subset which is unwrapped for all children of this node
/// See `nodes::scope`
pub fn scope_unwrap<'nodes, T, U, ScopedT: 'nodes, ScopedU: 'nodes>(
    scope_t: impl Fn(&mut T) -> &mut Option<ScopedT> + 'nodes,
    scope_u: impl Fn(&mut U) -> &mut Option<ScopedU> + 'nodes,
    node: impl Into<Node<'nodes, ScopedT, ScopedU>>,
) -> Node<'nodes, T, U> {
    Node {
        inner: NodeValue::NodeTrait {
            element: Box::new(OptionScoper {
                scope_t,
                scope_u,
                node: node.into(),
            }),
        },
    }
}
/// Scopes state to some owned derivative for all children of this node
/// once the child nodes have operated on state, embed is then called.
///
/// The scope & embed functions are generally called multiple times in a single `draw` call, use them sparingly
/// See `nodes::scope`
pub fn scope_owned<'nodes, T, U, ScopedT: 'nodes, ScopedU: 'nodes>(
    scope_t: impl Fn(&mut T) -> ScopedT + 'nodes,
    scope_u: impl Fn(&mut U) -> ScopedU + 'nodes,
    embed: impl Fn(&mut T, ScopedT, &mut U, ScopedU) + 'nodes,
    node: impl Into<Node<'nodes, ScopedT, ScopedU>>,
) -> Node<'nodes, T, U> {
    Node {
        inner: NodeValue::NodeTrait {
            element: Box::new(OwnedScoper {
                scope_t,
                scope_u,
                embed,
                node: node.into(),
            }),
        },
    }
}
/// Adds intermediate access to the available area allocated to this node during layout, before and after the node is drawn.
pub fn intermediate<'nodes, T, U>(
    before: impl Fn(Area, &mut T, &mut U) + 'nodes,
    after: impl Fn(&mut T, &mut U) + 'nodes,
    element: impl Into<Node<'nodes, T, U>> + 'nodes,
) -> Node<'nodes, T, U> {
    Node {
        inner: NodeValue::Intermediate {
            before: Box::new(before),
            after: Box::new(after),
            area: None,
            element: Box::new(NodeCache::new(element.into().inner)),
        },
    }
}

fn convert_into<'n, T, U>(elements: Vec<impl Into<Node<'n, T, U>>>) -> Vec<Node<'n, T, U>> {
    elements.into_iter().map(|e| e.into()).collect()
}

fn ungroup<T, U>(elements: Vec<Node<T, U>>) -> Vec<NodeCache<T, U>> {
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

fn filter_empty<T, U>(elements: Vec<NodeCache<T, U>>) -> Vec<NodeCache<T, U>> {
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
