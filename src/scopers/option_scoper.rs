use crate::{
    constraints::SizeConstraints,
    models::{Area, XAlign, YAlign},
    traits::NodeTrait,
    Node,
};
use std::fmt::Debug;

pub(crate) struct OptionScoper<'nodes, ScopedState, Scope, Embed, ScopedTree> {
    scope: Scope,
    embed: Embed,
    tree: ScopedTree,
    node: Option<Node<'nodes, ScopedState>>,
}

impl<ScopedState, Scope, Embed, ScopedTree>
    OptionScoper<'_, ScopedState, Scope, Embed, ScopedTree>
{
    pub(crate) fn new(scope: Scope, embed: Embed, tree_fn: ScopedTree) -> Self {
        Self {
            scope,
            embed,
            tree: tree_fn,
            node: None,
        }
    }
}

impl<ScopedState, Scope, Embed, ScopedTree> Debug
    for OptionScoper<'_, ScopedState, Scope, Embed, ScopedTree>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Kms")
    }
}

impl<
        'nodes,
        State,
        ScopedState,
        Scope: Fn(&mut State) -> Option<ScopedState>,
        Embed: Fn(&mut State, ScopedState),
        ScopedTree: Fn(&mut ScopedState) -> Node<'nodes, ScopedState>,
    > NodeTrait<State> for OptionScoper<'nodes, ScopedState, Scope, Embed, ScopedTree>
{
    fn constraints(&mut self, available_area: Area, state: &mut State) -> Option<SizeConstraints> {
        let mut substate = (self.scope)(state)?;
        let node = self.node.get_or_insert((self.tree)(&mut substate));
        let result = node.inner.constraints(available_area, &mut substate);
        (self.embed)(state, substate);
        result
    }

    fn layout(
        &mut self,
        available_area: Area,
        contextual_x_align: Option<XAlign>,
        contextual_y_align: Option<YAlign>,
        state: &mut State,
    ) {
        let Some(mut substate) = (self.scope)(state) else {
            return;
        };
        let node = self.node.get_or_insert((self.tree)(&mut substate));
        node.inner.layout(
            available_area,
            contextual_x_align,
            contextual_y_align,
            &mut substate,
        );
        (self.embed)(state, substate);
    }

    fn draw(&mut self, state: &mut State, contextual_visibility: bool) {
        let Some(mut substate) = (self.scope)(state) else {
            return;
        };
        let node = self.node.get_or_insert((self.tree)(&mut substate));
        node.inner.draw(&mut substate, contextual_visibility);
        (self.embed)(state, substate);
    }
}
