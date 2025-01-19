use crate::{
    constraints::SizeConstraints,
    models::{Area, XAlign, YAlign},
    Layout, Node,
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
    State,
    SubState,
    Scope: Fn(&mut State) -> SubState,
    Embed: Fn(SubState, &mut State),
> {
    scope: Scope,
    embed: Embed,
    layout: Layout<SubState>,
    n: Option<Node<SubState>>,
    _s: PhantomData<State>,
}

impl<State, SubState, Scope: Fn(&mut State) -> SubState, Embed: Fn(SubState, &mut State)>
    Scoper<State, SubState, Scope, Embed>
{
    pub(crate) fn new(
        scope: Scope,
        embed: Embed,
        tree: impl Fn(&mut SubState) -> Node<SubState> + 'static,
    ) -> Self {
        Self {
            scope,
            embed,
            layout: Layout::new(tree),
            n: None,
            _s: PhantomData,
        }
    }
}

impl<State, SubState, Scope: Fn(&mut State) -> SubState, Embed: Fn(SubState, &mut State)> Debug
    for Scoper<State, SubState, Scope, Embed>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Kms")
    }
}

impl<State, SubState, Scope: Fn(&mut State) -> SubState, Embed: Fn(SubState, &mut State)>
    NodeTrait<State> for Scoper<State, SubState, Scope, Embed>
{
    fn constraints(&mut self, available_area: Area, state: &mut State) -> Option<SizeConstraints> {
        let constraints: Option<SizeConstraints>;
        let mut substate = (self.scope)(state);
        if let Some(ref mut node) = self.n {
            constraints = node.inner.constraints(available_area, &mut substate);
        } else {
            let mut laid_out = (self.layout.tree)(&mut substate);
            constraints = laid_out.inner.constraints(available_area, &mut substate);
            self.n = Some(laid_out);
        };
        (self.embed)(substate, state);
        constraints
    }

    fn layout(
        &mut self,
        available_area: Area,
        contextual_x_align: Option<XAlign>,
        contextual_y_align: Option<YAlign>,
        state: &mut State,
    ) {
        let mut substate = (self.scope)(state);
        if let Some(ref mut node) = self.n {
            node.inner.layout(
                available_area,
                contextual_x_align,
                contextual_y_align,
                &mut substate,
            );
        } else {
            let mut laid_out = (self.layout.tree)(&mut substate);
            laid_out.inner.layout(
                available_area,
                contextual_x_align,
                contextual_y_align,
                &mut substate,
            );
            self.n = Some(laid_out);
        }
        (self.embed)(substate, state);
    }

    fn draw(&mut self, state: &mut State, contextual_visibility: bool) {
        let mut substate = (self.scope)(state);
        if let Some(ref mut node) = self.n {
            node.inner.draw(&mut substate, contextual_visibility);
        } else {
            let mut laid_out = (self.layout.tree)(&mut substate);
            laid_out.inner.draw(&mut substate, contextual_visibility);
            self.n = Some(laid_out);
        }
        (self.embed)(substate, state);
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
//     use crate::{models::Area, nodes::*};
//     #[test]
//     fn test_thing() {
//         struct A {
//             b: B,
//         }
//         struct B;

//         let mut node = row(vec![scope(
//             |s: &mut A| &mut s.b,
//             |_b: &mut B| space().height(100.),
//         )]);

//         // let mut node = space().width(100.);
//         let mut state = A { b: B };
//         let c = node
//             .inner
//             .constraints(Area::new(0., 0., 100., 100.), &mut state);
//     }
// }
