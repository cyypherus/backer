use std::fmt::Debug;

use crate::{
    models::{Area, XAlign, YAlign},
    traits::NodeTrait,
    Node, SizeConstraints,
};

pub(crate) struct Scoper<'n, SubState, ScopeStateFn> {
    scope_fn: ScopeStateFn,
    n: Node<'n, SubState>,
}

// impl<SubState, ScopeStateFn> Scoper<SubState, ScopeStateFn> {
//     pub(crate) fn new(
//         scope_fn: ScopeStateFn,
//         tree: impl Fn(&mut SubState) -> Node<SubState> + 'static,
//     ) -> Self {
//         Self {
//             scope_fn,
//             // layout: Layout::new(tree),
//             n: None,
//             _s: PhantomData,
//         }
//     }
// }

pub struct ScopeCtxResult {
    value: ResultValue,
}

enum ResultValue {
    Void,
    Constraints(Option<SizeConstraints>),
}
impl<'n, SubState, ScopeStateFn> Debug for Scoper<'n, SubState, ScopeStateFn> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Kms")
    }
}

pub struct ScopeCtx<'a, 'n, SubState> {
    // layout: &'a mut Layout<SubState>,
    node: &'a mut Node<'n, SubState>,
    area: Area,
    contextual_x_align: Option<XAlign>,
    contextual_y_align: Option<YAlign>,
    contextual_visibility: bool,
    with_scoped: fn(
        area: Area,
        contextual_x_align: Option<XAlign>,
        contextual_y_align: Option<YAlign>,
        contextual_visibility: bool,
        // &mut Layout<SubState>,
        &mut Node<SubState>,
        &mut SubState,
    ) -> ResultValue,
}

impl<'a, 'n, SubState> ScopeCtx<'a, 'n, SubState> {
    pub fn with_scoped(self, scoped: &mut SubState) -> ScopeCtxResult {
        ScopeCtxResult {
            value: (self.with_scoped)(
                self.area,
                self.contextual_x_align,
                self.contextual_y_align,
                self.contextual_visibility,
                // self.layout,
                self.node,
                scoped,
            ),
        }
    }
}
impl<'n, State, SubState, ScopeStateFn> NodeTrait<State> for Scoper<'n, SubState, ScopeStateFn>
where
    ScopeStateFn: Fn(ScopeCtx<'_, 'n, SubState>, &mut State) -> ScopeCtxResult,
{
    fn constraints(&mut self, available_area: Area, state: &mut State) -> Option<SizeConstraints> {
        let ScopeCtxResult {
            value: ResultValue::Constraints(constraints),
        } = (self.scope_fn)(
            ScopeCtx {
                // layout: &mut self.layout,
                node: &mut self.n,
                area: available_area,
                contextual_x_align: None,
                contextual_y_align: None,
                contextual_visibility: false,
                with_scoped: |area: Area,
                              _contextual_x_align: Option<XAlign>,
                              _contextual_y_align: Option<YAlign>,
                              _contextual_visibility: bool,
                              // layout: &mut Layout<SubState>,
                              node: &mut Node<SubState>,
                              sc: &mut SubState| {
                    // let constraints: Option<SizeConstraints>;
                    // if let Some(node) = node {
                    //     constraints = node.inner.constraints(area, sc);
                    // } else {
                    //     let mut laid_out = (layout.tree)(sc);
                    //     constraints = laid_out.inner.constraints(area, sc);
                    //     *node = Some(laid_out);
                    // }
                    // ResultValue::Constraints(constraints)
                    ResultValue::Constraints(None)
                },
            },
            state,
        )
        else {
            unreachable!()
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
        let ScopeCtxResult {
            value: ResultValue::Void,
        } = (self.scope_fn)(
            ScopeCtx {
                node: &mut self.n,
                area: available_area,
                contextual_x_align,
                contextual_y_align,
                contextual_visibility: false,
                with_scoped: |available_area: Area,
                              contextual_x_align: Option<XAlign>,
                              contextual_y_align: Option<YAlign>,
                              _contextual_visibility: bool,
                              node: &mut Node<SubState>,
                              sc: &mut SubState| {
                    node.inner
                        .layout(available_area, contextual_x_align, contextual_y_align, sc);
                    ResultValue::Void
                },
            },
            state,
        )
        else {
            unreachable!()
        };
    }

    fn draw(&mut self, state: &mut State, contextual_visibility: bool) {
        let ScopeCtxResult {
            value: ResultValue::Void,
        } = (self.scope_fn)(
            ScopeCtx {
                node: &mut self.n,
                area: Area::zero(),
                contextual_x_align: None,
                contextual_y_align: None,
                contextual_visibility,
                with_scoped: |_available_area: Area,
                              _contextual_x_align: Option<XAlign>,
                              _contextual_y_align: Option<YAlign>,
                              contextual_visibility: bool,
                              node: &mut Node<SubState>,
                              sc: &mut SubState| {
                    node.inner.draw(sc, contextual_visibility);
                    ResultValue::Void
                },
            },
            state,
        )
        else {
            unreachable!()
        };
    }
}
