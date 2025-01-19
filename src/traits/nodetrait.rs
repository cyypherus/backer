use crate::{
    constraints::SizeConstraints,
    models::{Area, XAlign, YAlign},
    Layout, Node,
};
use std::{fmt::Debug, marker::PhantomData};

pub(crate) trait NodeTrait<'nodes, State>: Debug {
    fn constraints<'state, 'n>(
        &'n mut self,
        available_area: Area,
        state: &'state mut State,
    ) -> Option<SizeConstraints>;
    fn layout<'state>(
        &mut self,
        available_area: Area,
        contextual_x_align: Option<XAlign>,
        contextual_y_align: Option<YAlign>,
        state: &'state mut State,
    );
    fn draw<'state>(&mut self, state: &'state mut State, contextual_visibility: bool);
}

pub(crate) struct Scoper<'nodes, State, SubState, SubLayout: Layout> {
    scope_fn: for<'fstate, 'fnodes> fn(
        ScopeCtx<'nodes, SubState, SubLayout>,
        &'fstate mut State,
    ) -> ScopeCtxResult,
    layout: SubLayout,
    n: Option<Node<'nodes, SubState>>,
    _s: PhantomData<State>,
}

impl<'nodes, State, SubState, SubLayout: Layout> Scoper<'nodes, State, SubState, SubLayout> {
    pub(crate) fn new(
        scope_fn: for<'fstate, 'fnodes> fn(
            ScopeCtx<'fnodes, SubState, SubLayout>,
            &'fstate mut State,
        ) -> ScopeCtxResult,
        tree: SubLayout,
    ) -> Self {
        Self {
            scope_fn,
            layout: tree,
            n: None,
            _s: PhantomData,
        }
    }
}

pub struct ScopeCtxResult {
    value: ResultValue,
}

enum ResultValue {
    Void,
    Constraints(Option<SizeConstraints>),
}

impl<'nodes, State, SubState, SubLayout: Layout> Debug
    for Scoper<'nodes, State, SubState, SubLayout>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Kms")
    }
}

pub struct ScopeCtx<'nodes, SubState, SubLayout: Layout> {
    layout: &'nodes mut SubLayout,
    node: &'nodes mut Option<Node<'nodes, SubState>>,
    area: Area,
    contextual_x_align: Option<XAlign>,
    contextual_y_align: Option<YAlign>,
    contextual_visibility: bool,
    with_scoped: fn(
        area: Area,
        contextual_x_align: Option<XAlign>,
        contextual_y_align: Option<YAlign>,
        contextual_visibility: bool,
        &mut SubLayout,
        &mut Option<Node<'nodes, SubState>>,
        &mut SubState,
    ) -> ResultValue,
}

impl<'state, 'nodes, SubState, SubLayout: Layout> ScopeCtx<'nodes, SubState, SubLayout> {
    pub fn with_scoped(self, scoped: &'state mut SubState) -> ScopeCtxResult {
        ScopeCtxResult {
            value: (self.with_scoped)(
                self.area,
                self.contextual_x_align,
                self.contextual_y_align,
                self.contextual_visibility,
                self.layout,
                self.node,
                scoped,
            ),
        }
    }
}

impl<'nodes, 'scope_nodes, State, SubState, SubLayout: Layout> NodeTrait<'nodes, State>
    for Scoper<'scope_nodes, State, SubState, SubLayout>
where
    'nodes: 'scope_nodes,
{
    fn constraints<'state, 'n: 'scope_nodes>(
        &'n mut self,
        available_area: Area,
        state: &'state mut State,
    ) -> Option<SizeConstraints> {
        let ScopeCtxResult {
            value: ResultValue::Constraints(constraints),
        } = (self.scope_fn)(
            ScopeCtx {
                layout: &mut self.layout,
                node: &mut self.n,
                area: available_area,
                contextual_x_align: None,
                contextual_y_align: None,
                contextual_visibility: false,
                with_scoped: move |area: Area,
                                   _contextual_x_align: Option<XAlign>,
                                   _contextual_y_align: Option<YAlign>,
                                   _contextual_visibility: bool,
                                   layout: &mut SubLayout,
                                   node: &mut Option<Node<SubState>>,
                                   sc: &mut SubState| {
                    let constraints: Option<SizeConstraints>;
                    if let Some(ref mut node) = node {
                        constraints = node.inner.constraints(area, sc);
                    } else {
                        let mut laid_out = layout.tree(sc);
                        constraints = laid_out.inner.constraints(area, sc);
                        *node = Some(laid_out);
                    }
                    ResultValue::Constraints(constraints)
                },
            },
            state,
        )
        else {
            unreachable!()
        };
        constraints
    }

    fn layout<'state>(
        &mut self,
        available_area: Area,
        contextual_x_align: Option<XAlign>,
        contextual_y_align: Option<YAlign>,
        state: &'state mut State,
    ) {
        // let ScopeCtxResult {
        //     value: ResultValue::Void,
        // } = (self.scope_fn)(
        //     ScopeCtx {
        //         layout: &mut self.layout,
        //         node: &mut self.n,
        //         area: available_area,
        //         contextual_x_align,
        //         contextual_y_align,
        //         contextual_visibility: false,
        //         with_scoped: |area: Area,
        //                       _contextual_x_align: Option<XAlign>,
        //                       _contextual_y_align: Option<YAlign>,
        //                       _contextual_visibility: bool,
        //                       layout: &mut SubLayout,
        //                       node: &mut Option<Node<SubState>>,
        //                       sc: &mut SubState| {
        //             if let Some(node) = node {
        //                 node.inner.layout(
        //                     available_area,
        //                     contextual_x_align,
        //                     contextual_y_align,
        //                     sc,
        //                 );
        //             } else {
        //                 let mut laid_out = layout.tree(sc);
        //                 laid_out.inner.layout(
        //                     available_area,
        //                     contextual_x_align,
        //                     contextual_y_align,
        //                     sc,
        //                 );
        //                 *node = Some(laid_out);
        //             }
        //             ResultValue::Void
        //         },
        //     },
        //     state,
        // )
        // else {
        //     unreachable!()
        // };
    }

    fn draw<'state>(&mut self, state: &mut State, contextual_visibility: bool) {
        // let ScopeCtxResult {
        //     value: ResultValue::Void,
        // } = (self.scope_fn)(
        //     ScopeCtx {
        //         layout: &mut self.layout,
        //         node: &mut self.n,
        //         area: Area::zero(),
        //         contextual_x_align: None,
        //         contextual_y_align: None,
        //         contextual_visibility,
        //         with_scoped: move |area: Area,
        //                            _contextual_x_align: Option<XAlign>,
        //                            _contextual_y_align: Option<YAlign>,
        //                            _contextual_visibility: bool,
        //                            layout: &mut SubLayout,
        //                            node: &mut Option<Node<SubState>>,
        //                            sc: &mut SubState| {
        //             if let Some(node) = node {
        //                 node.inner.draw(sc, contextual_visibility);
        //             } else {
        //                 let mut laid_out = layout.tree(sc);
        //                 laid_out.inner.draw(sc, contextual_visibility);
        //                 *node = Some(laid_out);
        //             }
        //             ResultValue::Void
        //         },
        //     },
        //     state,
        // )
        // else {
        //     unreachable!()
        // };
    }
}

// #[cfg(test)]
// mod test {
//     use crate::{models::Area, nodes::*, traits::nodetrait::ScopeCtx};
//     #[test]
//     fn test_thing() {
//         struct A {
//             b: B,
//         }
//         struct B;

//         let mut node = row(vec![scoper(
//             |ctx: ScopeCtx<B>, s: &mut A| {
//                 let scoped = &mut s.b;
//                 ctx.with_scoped(scoped)
//             },
//             |_b: &mut B| space().height(100.),
//         )]);

//         // let mut node = space().width(100.);
//         let mut state = A { b: B };
//         let c = node
//             .inner
//             .constraints(Area::new(0., 0., 100., 100.), &mut state);
//     }
// }
