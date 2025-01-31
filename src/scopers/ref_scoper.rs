use crate::{
    constraints::SizeConstraints,
    models::{Area, XAlign, YAlign},
    traits::NodeTrait,
    Layout, Node,
};
use std::{fmt::Debug, marker::PhantomData};

pub(crate) struct RefScoper<'nodes, ScopedState, WithScoped, Scope, ScopedTree> {
    scope: Scope,
    tree: ScopedTree,
    node: Option<Node<'nodes, ScopedState>>,
    _w: PhantomData<WithScoped>,
}

impl<ScopedState, WithScoped, Scope, ScopedTree>
    RefScoper<'_, ScopedState, WithScoped, Scope, ScopedTree>
{
    pub(crate) fn new(scope: Scope, tree_fn: ScopedTree) -> Self {
        Self {
            scope,
            tree: tree_fn,
            node: None,
            _w: PhantomData,
        }
    }
}

impl<ScopedState, WithScoped, Scope, ScopedTree> Debug
    for RefScoper<'_, ScopedState, WithScoped, Scope, ScopedTree>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Kms")
    }
}

trait ScopeFn<WithScoped, State> {
    fn scope(with_scoped: WithScoped, state: &mut State) -> ScopeCtxResult;
}

impl<
        'nodes,
        State,
        ScopedState,
        WithScoped,
        ScopedTree: Fn(&mut ScopedState) -> Node<'_, ScopedState> + 'static,
        Scope: Fn(&mut State, ScopeCtx<'_, ScopedState, ScopedTree>) -> ScopeCtxResult,
    > NodeTrait<'nodes, State> for RefScoper<'nodes, ScopedState, WithScoped, Scope, ScopedTree>
{
    fn constraints(
        &'nodes mut self,
        available_area: Area,
        state: &mut State,
    ) -> Option<SizeConstraints> {
        let ScopeCtxResult {
            value: ResultValue::Constraints(constraints),
        } = (self.scope)(
            state,
            ScopeCtx {
                node: &mut self.node,
                area: available_area,
                contextual_x_align: None,
                contextual_y_align: None,
                contextual_visibility: false,
                tree: &self.tree,
                with_scoped: |area, _, _, _, tree, node, substate| {
                    // let node = node.get_or_insert(tree(substate));
                    // node.inner.constraints(area, substate);
                    ResultValue::Void
                },
            },
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
        todo!()
        // let substate = (self.scope)(state);
        // let node = self.node.get_or_insert((self.tree)(substate));
        // node.inner.layout(
        //     available_area,
        //     contextual_x_align,
        //     contextual_y_align,
        //     substate,
        // );
    }

    fn draw(&mut self, state: &mut State, contextual_visibility: bool) {
        todo!()
        // let substate = (self.scope)(state);
        // let node = self.node.get_or_insert((self.tree)(substate));
        // node.inner.draw(substate, contextual_visibility);
    }
}

pub struct ScopeCtx<'a, SubState, ScopedTree> {
    // layout: &'a mut Layout<'a, SubState>,
    node: &'a mut Option<Node<'a, SubState>>,
    area: Area,
    contextual_x_align: Option<XAlign>,
    contextual_y_align: Option<YAlign>,
    contextual_visibility: bool,
    tree: &'a ScopedTree,
    with_scoped: fn(
        area: Area,
        contextual_x_align: Option<XAlign>,
        contextual_y_align: Option<YAlign>,
        contextual_visibility: bool,
        &ScopedTree,
        &mut Option<Node<SubState>>,
        &mut SubState,
    ) -> ResultValue,
}

impl<'a, SubState, ScopedTree> ScopeCtx<'a, SubState, ScopedTree> {
    pub fn with_scoped(self, scoped: &mut SubState) -> ScopeCtxResult {
        ScopeCtxResult {
            value: (self.with_scoped)(
                self.area,
                self.contextual_x_align,
                self.contextual_y_align,
                self.contextual_visibility,
                &self.tree,
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
