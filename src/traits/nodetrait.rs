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
    'nodes,
    State,
    ScopedState,
    Scope: Fn(&mut State) -> &mut ScopedState,
    ScopedTree: Fn(&mut ScopedState) -> Node<'nodes, ScopedState>,
> {
    scope: Scope,
    tree: ScopedTree,
    node: Option<Node<'nodes, ScopedState>>,
    t: PhantomData<State>,
}

impl<
        'nodes,
        State,
        ScopedState,
        Scope: Fn(&mut State) -> &mut ScopedState,
        ScopedTree: Fn(&mut ScopedState) -> Node<'nodes, ScopedState>,
    > Scoper<'nodes, State, ScopedState, Scope, ScopedTree>
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

impl<
        'nodes,
        State,
        ScopedState,
        Scope: Fn(&mut State) -> &mut ScopedState,
        ScopedTree: Fn(&mut ScopedState) -> Node<'nodes, ScopedState>,
    > Debug for Scoper<'nodes, State, ScopedState, Scope, ScopedTree>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Kms")
    }
}

impl<
        'nodes,
        State,
        ScopedState,
        Scope: Fn(&mut State) -> &mut ScopedState,
        ScopedTree: Fn(&mut ScopedState) -> Node<'nodes, ScopedState>,
    > NodeTrait<State> for Scoper<'nodes, State, ScopedState, Scope, ScopedTree>
{
    fn constraints(&mut self, available_area: Area, state: &mut State) -> Option<SizeConstraints> {
        let substate = (self.scope)(state);
        let node = self.node.get_or_insert((self.tree)(substate));
        let substate = (self.scope)(state);
        node.inner.constraints(available_area, substate)
    }

    fn layout(
        &mut self,
        available_area: Area,
        contextual_x_align: Option<XAlign>,
        contextual_y_align: Option<YAlign>,
        state: &mut State,
    ) {
        let substate = (self.scope)(state);
        let node = self.node.get_or_insert((self.tree)(substate));
        let substate = (self.scope)(state);
        node.inner.layout(
            available_area,
            contextual_x_align,
            contextual_y_align,
            substate,
        );
    }

    fn draw(&mut self, state: &mut State, contextual_visibility: bool) {
        let substate = (self.scope)(state);
        let node = self.node.get_or_insert((self.tree)(substate));
        node.inner.draw(substate, contextual_visibility);

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
