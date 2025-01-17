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

pub(crate) struct Scoper<State, SubState, ScopeStateFn>
where
    ScopeStateFn: Fn(ScopeCtx<'_, SubState>, &mut State) -> ScopeCtxResult,
{
    scope_fn: ScopeStateFn,
    layout: Layout<SubState>,
    n: Option<Node<SubState>>,
    _s: PhantomData<State>,
}

impl<State, SubState, ScopeStateFn> Scoper<State, SubState, ScopeStateFn>
where
    ScopeStateFn: Fn(ScopeCtx<'_, SubState>, &mut State) -> ScopeCtxResult,
{
    pub(crate) fn new(
        scope_fn: ScopeStateFn,
        tree: impl Fn(&mut SubState) -> Node<SubState> + 'static,
    ) -> Self {
        Self {
            scope_fn,
            layout: Layout::new(tree),
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

impl<State, SubState, ScopeStateFn> Debug for Scoper<State, SubState, ScopeStateFn>
where
    ScopeStateFn: Fn(ScopeCtx<'_, SubState>, &mut State) -> ScopeCtxResult,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Kms")
    }
}

pub struct ScopeCtx<'a, SubState> {
    layout: &'a mut Layout<SubState>,
    node: &'a mut Option<Node<SubState>>,
    area: Area,
    contextual_x_align: Option<XAlign>,
    contextual_y_align: Option<YAlign>,
    contextual_visibility: bool,
    with_scoped: fn(
        area: Area,
        contextual_x_align: Option<XAlign>,
        contextual_y_align: Option<YAlign>,
        contextual_visibility: bool,
        &mut Layout<SubState>,
        &mut Option<Node<SubState>>,
        &mut SubState,
    ) -> ResultValue,
}

impl<'a, SubState> ScopeCtx<'a, SubState> {
    pub fn with_scoped(self, scoped: &mut SubState) -> ScopeCtxResult {
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

impl<State, SubState, ScopeStateFn> NodeTrait<State> for Scoper<State, SubState, ScopeStateFn>
where
    ScopeStateFn: Fn(ScopeCtx<'_, SubState>, &mut State) -> ScopeCtxResult,
{
    fn constraints(&mut self, available_area: Area, state: &mut State) -> Option<SizeConstraints> {
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
                                   layout: &mut Layout<SubState>,
                                   node: &mut Option<Node<SubState>>,
                                   sc: &mut SubState| {
                    let constraints: Option<SizeConstraints>;
                    if let Some(node) = node {
                        constraints = node.inner.constraints(area, sc);
                    } else {
                        let mut laid_out = (layout.tree)(sc);
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
                layout: &mut self.layout,
                node: &mut self.n,
                area: available_area,
                contextual_x_align,
                contextual_y_align,
                contextual_visibility: false,
                with_scoped: move |available_area: Area,
                                   contextual_x_align: Option<XAlign>,
                                   contextual_y_align: Option<YAlign>,
                                   _contextual_visibility: bool,
                                   layout: &mut Layout<SubState>,
                                   node: &mut Option<Node<SubState>>,
                                   sc: &mut SubState| {
                    if let Some(node) = node {
                        node.inner.layout(
                            available_area,
                            contextual_x_align,
                            contextual_y_align,
                            sc,
                        );
                    } else {
                        let mut laid_out = (layout.tree)(sc);
                        laid_out.inner.layout(
                            available_area,
                            contextual_x_align,
                            contextual_y_align,
                            sc,
                        );
                        *node = Some(laid_out);
                    }
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
                layout: &mut self.layout,
                node: &mut self.n,
                area: Area::zero(),
                contextual_x_align: None,
                contextual_y_align: None,
                contextual_visibility,
                with_scoped: move |_available_area: Area,
                                   _contextual_x_align: Option<XAlign>,
                                   _contextual_y_align: Option<YAlign>,
                                   contextual_visibility: bool,
                                   layout: &mut Layout<SubState>,
                                   node: &mut Option<Node<SubState>>,
                                   sc: &mut SubState| {
                    if let Some(node) = node {
                        node.inner.draw(sc, contextual_visibility);
                    } else {
                        let mut laid_out = (layout.tree)(sc);
                        laid_out.inner.draw(sc, contextual_visibility);
                        *node = Some(laid_out);
                    }
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

#[cfg(test)]
mod test {
    use crate::{models::Area, nodes::*, traits::nodetrait::ScopeCtx};
    #[test]
    fn test_thing() {
        struct A {
            b: B,
        }
        struct B;

        let mut node = row(vec![scoper(
            |ctx: ScopeCtx<B>, s: &mut A| {
                let scoped = &mut s.b;
                ctx.with_scoped(scoped)
            },
            |_b: &mut B| space().height(100.),
        )]);

        // let mut node = space().width(100.);
        let mut state = A { b: B };
        let c = node
            .inner
            .constraints(Area::new(0., 0., 100., 100.), &mut state);
    }
}
