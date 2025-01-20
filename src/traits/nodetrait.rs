use crate::{
    constraints::SizeConstraints,
    models::{Area, XAlign, YAlign},
    Node,
};
use std::{fmt::Debug, marker::PhantomData};

pub(crate) trait NodeTrait<State>: Debug {
    fn constraints(&mut self, available_area: Area, state: &mut State) -> Option<SizeConstraints>;
    fn layout(
        &mut self,
        available_area: Area,
        contextual_x_align: Option<XAlign>,
        contextual_y_align: Option<YAlign>,
        state: &mut State,
    );
    fn draw(&mut self, state: &mut State, contextual_visibility: bool);
}

pub(crate) struct Scoper<
    't,
    T,
    U,
    Scope: Fn(&mut T) -> &mut U,
    ScopedTree: Fn(&mut U) -> Node<'t, U>,
> {
    scope: Scope,
    tree: ScopedTree,
    node: Option<Node<'t, U>>,
    t: PhantomData<T>,
}

impl<'t, T, U, Scope: Fn(&mut T) -> &mut U, ScopedTree: Fn(&mut U) -> Node<'t, U>>
    Scoper<'t, T, U, Scope, ScopedTree>
{
    pub(crate) fn new(scope: Scope, tree_fn: ScopedTree) -> Self {
        Self {
            scope,
            tree: tree_fn,
            node: None,
            t: PhantomData,
        }
    }
}

impl<'t, T, U, Scope: Fn(&mut T) -> &mut U, ScopedTree: Fn(&mut U) -> Node<'t, U>> Debug
    for Scoper<'t, T, U, Scope, ScopedTree>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Kms")
    }
}

impl<'t, State, U, Scope: Fn(&mut State) -> &mut U, ScopedTree: Fn(&mut U) -> Node<'t, U>>
    NodeTrait<State> for Scoper<'t, State, U, Scope, ScopedTree>
{
    fn constraints(&mut self, available_area: Area, state: &mut State) -> Option<SizeConstraints> {
        let constraints: Option<SizeConstraints>;
        let substate = (self.scope)(state);
        if let Some(ref mut node) = self.node {
            constraints = node.inner.constraints(available_area, substate);
        } else {
            let mut laid_out = (self.tree)(substate);
            constraints = laid_out.inner.constraints(available_area, substate);
            self.node = Some(laid_out);
        };
        constraints
    }

    fn layout(
        &mut self,
        available_area: Area,
        contextual_x_align: Option<XAlign>,
        contextual_y_align: Option<YAlign>,
        state: &mut State,
    ) {
        let substate = (self.scope)(state);
        if let Some(ref mut node) = self.node {
            node.inner.layout(
                available_area,
                contextual_x_align,
                contextual_y_align,
                substate,
            );
        } else {
            let mut laid_out = (self.tree)(substate);
            laid_out.inner.layout(
                available_area,
                contextual_x_align,
                contextual_y_align,
                substate,
            );
            self.node = Some(laid_out);
        }
    }

    fn draw(&mut self, state: &mut State, contextual_visibility: bool) {
        let substate = (self.scope)(state);
        if let Some(ref mut node) = self.node {
            node.inner.draw(substate, contextual_visibility);
        } else {
            let mut laid_out = (self.tree)(substate);
            laid_out.inner.draw(substate, contextual_visibility);
            self.node = Some(laid_out);
        }
        // let ScopeCtxResult {
        //     value: ResultValue::Void,
        // } = (self.scope)(
        //     ScopeCtx {
        //         layout: &mut self.layout,
        //         node: &mut self.n,
        //         area: Area::zero(),
        //         contextual_x_align: None,
        //         contextual_y_align: None,
        //         contextual_visibility,
        //         with_scoped: move |_available_area: Area,
        //                            _contextual_x_align: Option<XAlign>,
        //                            _contextual_y_align: Option<YAlign>,
        //                            contextual_visibility: bool,
        //                            layout: &mut Layout<SubState>,
        //                            node: &mut Option<Node<SubState>>,
        //                            sc: &mut SubState| {
        //             if let Some(node) = node {
        //                 node.inner.draw(sc, contextual_visibility);
        //             } else {
        //                 let mut laid_out = (layout.tree)(sc);
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
