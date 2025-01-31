use std::fmt::Debug;

use crate::{
    constraints::SizeConstraints,
    models::{Area, XAlign, YAlign},
    traits::NodeTrait,
    Node,
};

pub(crate) struct Scoper<'n, SubState, ScopeStateFn> {
    pub(crate) scope_fn: ScopeStateFn,
    pub(crate) node: Node<'n, SubState>,
}

/// Anonymous result to return from the closure passed to `nodes::scope`
/// See `nodes::scope`
pub struct ScopeCtxResult {
    value: ResultValue,
}

enum ResultValue {
    Void,
    Constraints(Option<SizeConstraints>),
}

impl<SubState, ScopeStateFn> Debug for Scoper<'_, SubState, ScopeStateFn> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Scoper")
            .field("scope_fn", &"<function>")
            .field("node", &self.node)
            .finish()
    }
}

type WithScopedFnPointer<SubState> = fn(
    area: Area,
    contextual_x_align: Option<XAlign>,
    contextual_y_align: Option<YAlign>,
    contextual_visibility: bool,
    &mut Node<SubState>,
    &mut SubState,
) -> ResultValue;

/// Contextual state for scoping, see `nodes::scope`
pub struct ScopeCtx<'a, 'nodes, SubState> {
    node: &'a mut Node<'nodes, SubState>,
    area: Area,
    contextual_x_align: Option<XAlign>,
    contextual_y_align: Option<YAlign>,
    contextual_visibility: bool,
    with_scoped: WithScopedFnPointer<SubState>,
}

impl<SubState> ScopeCtx<'_, '_, SubState> {
    /// Takes the subset of state being scoped to, returns an anonymous result to be returned from the closure passed into `nodes::scope`
    pub fn with_scoped(self, scoped: &mut SubState) -> ScopeCtxResult {
        ScopeCtxResult {
            value: (self.with_scoped)(
                self.area,
                self.contextual_x_align,
                self.contextual_y_align,
                self.contextual_visibility,
                self.node,
                scoped,
            ),
        }
    }
    /// Used when scoping to a state that is potentially "invalid", such as when scoping to an `Option` in a way that will unwrap the state for the child nodes
    /// Returns an anonymous result to be returned from the closure passed into `nodes::scope`
    pub fn empty(self) -> ScopeCtxResult {
        ScopeCtxResult {
            value: ResultValue::Void,
        }
    }
}

impl<'nodes, State, SubState, ScopeStateFn> NodeTrait<State>
    for Scoper<'nodes, SubState, ScopeStateFn>
where
    ScopeStateFn: Fn(ScopeCtx<'_, 'nodes, SubState>, &mut State) -> ScopeCtxResult,
{
    fn constraints(&mut self, available_area: Area, state: &mut State) -> Option<SizeConstraints> {
        let ScopeCtxResult {
            value: ResultValue::Constraints(constraints),
        } = (self.scope_fn)(
            ScopeCtx {
                node: &mut self.node,
                area: available_area,
                contextual_x_align: None,
                contextual_y_align: None,
                contextual_visibility: false,
                with_scoped: |area: Area,
                              _contextual_x_align: Option<XAlign>,
                              _contextual_y_align: Option<YAlign>,
                              _contextual_visibility: bool,
                              node: &mut Node<SubState>,
                              sc: &mut SubState| {
                    ResultValue::Constraints(node.inner.constraints(area, sc))
                },
            },
            state,
        )
        else {
            return None;
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
                node: &mut self.node,
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
            return;
        };
    }

    fn draw(&mut self, state: &mut State, contextual_visibility: bool) {
        let ScopeCtxResult {
            value: ResultValue::Void,
        } = (self.scope_fn)(
            ScopeCtx {
                node: &mut self.node,
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
            return;
        };
    }
}
