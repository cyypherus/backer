use crate::{
    constraints::SizeConstraints,
    models::{Area, XAlign, YAlign},
    traits::NodeTrait,
    Layout, Node,
};
use std::fmt::Debug;

pub(crate) struct RefScoper<'nodes, ScopedState, WithScoped, Scope, ScopedTree> {
    scope: Scope,
    tree: ScopedTree,
    node: Option<Node<'nodes, ScopedState>>,
}

impl<ScopedState, Scope, ScopedTree> RefScoper<'_, ScopedState, Scope, ScopedTree> {
    pub(crate) fn new(scope: Scope, tree_fn: ScopedTree) -> Self {
        Self {
            scope,
            tree: tree_fn,
            node: None,
        }
    }
}

impl<ScopedState, Scope, ScopedTree> Debug for RefScoper<'_, ScopedState, Scope, ScopedTree> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Kms")
    }
}

impl<
        'nodes,
        State,
        ScopedState,
        WithScoped: Fn(&mut ScopedState) -> ScopeCtxResult,
        Scope: Fn(WithScoped, &mut State) -> ScopeCtxResult,
        ScopedTree: Fn(&mut ScopedState) -> Node<'nodes, ScopedState>,
    > NodeTrait<State> for RefScoper<'nodes, ScopedState, WithScoped, Scope, ScopedTree>
{
    fn constraints(&mut self, available_area: Area, state: &mut State) -> Option<SizeConstraints> {
        let substate = (self.scope)(state);
        let node = self.node.get_or_insert((self.tree)(substate));
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
    }
}

pub struct ScopeCtx<'a, SubState> {
    layout: &'a mut Layout<'a, SubState>,
    node: &'a mut Option<Node<'a, SubState>>,
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

pub struct ScopeCtxResult {
    value: ResultValue,
}

enum ResultValue {
    Void,
    Constraints(Option<SizeConstraints>),
}
