use crate::{
    constraints::SizeConstraints,
    models::{Area, XAlign, YAlign},
    Node,
};
use std::fmt::Debug;

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

pub(crate) struct OwnedScoper<'nodes, ScopedState, Scope, Embed, ScopedTree> {
    scope: Scope,
    embed: Embed,
    tree: ScopedTree,
    node: Option<Node<'nodes, ScopedState>>,
}

impl<ScopedState, Scope, Embed, ScopedTree> OwnedScoper<'_, ScopedState, Scope, Embed, ScopedTree> {
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
    for OwnedScoper<'_, ScopedState, Scope, Embed, ScopedTree>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Kms")
    }
}

impl<'nodes, State, ScopedState, Scope, Embed, ScopedTree> NodeTrait<State>
    for OwnedScoper<'nodes, ScopedState, Scope, Embed, ScopedTree>
where
    Scope: Fn(&mut State) -> ScopedState,
    Embed: Fn(&mut State, ScopedState),
    ScopedTree: Fn(&mut ScopedState) -> Node<'nodes, ScopedState>,
{
    fn constraints(&mut self, available_area: Area, state: &mut State) -> Option<SizeConstraints> {
        let mut substate = (self.scope)(state);
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
        let mut substate = (self.scope)(state);
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
        let mut substate = (self.scope)(state);
        let node = self.node.get_or_insert((self.tree)(&mut substate));
        node.inner.draw(&mut substate, contextual_visibility);
        (self.embed)(state, substate);
    }
}

// pub(crate) struct OptionScoper<
//     'nodes,
//     State,
//     ScopedState,
//     Scope: Fn(&mut State) -> Option<ScopedState>,
//     Embed: Fn(&mut State, ScopedState),
//     ScopedTree: Fn(&mut ScopedState) -> Node<'nodes, ScopedState>,
// > {
//     scope: Scope,
//     embed: Embed,
//     tree: ScopedTree,
//     node: Option<Node<'nodes, ScopedState>>,
//     t: PhantomData<State>,
// }

// impl<
//         'nodes,
//         State,
//         ScopedState,
//         Scope: Fn(&mut State) -> Option<ScopedState>,
//         Embed: Fn(&mut State, ScopedState),
//         ScopedTree: Fn(&mut ScopedState) -> Node<'nodes, ScopedState>,
//     > OptionScoper<'nodes, State, ScopedState, Scope, Embed, ScopedTree>
// {
//     pub(crate) fn new(scope: Scope, embed: Embed, tree_fn: ScopedTree) -> Self {
//         Self {
//             scope,
//             embed,
//             tree: tree_fn,
//             node: None,
//             t: PhantomData,
//         }
//     }
// }

// impl<
//         'nodes,
//         State,
//         ScopedState,
//         Scope: Fn(&mut State) -> Option<ScopedState>,
//         Embed: Fn(&mut State, ScopedState),
//         ScopedTree: Fn(&mut ScopedState) -> Node<'nodes, ScopedState>,
//     > Debug for OptionScoper<'nodes, State, ScopedState, Scope, Embed, ScopedTree>
// {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.write_str("Kms")
//     }
// }

// impl<
//         'nodes,
//         State,
//         ScopedState,
//         Scope: Fn(&mut State) -> Option<ScopedState>,
//         Embed: Fn(&mut State, ScopedState),
//         ScopedTree: Fn(&mut ScopedState) -> Node<'nodes, ScopedState>,
//     > NodeTrait<State> for OptionScoper<'nodes, State, ScopedState, Scope, Embed, ScopedTree>
// {
//     fn constraints(&mut self, available_area: Area, state: &mut State) -> Option<SizeConstraints> {
//         let mut substate = (self.scope)(state)?;
//         let node = self.node.get_or_insert((self.tree)(&mut substate));
//         let result = node.inner.constraints(available_area, &mut substate);
//         (self.embed)(state, substate);
//         result
//     }

//     fn layout(
//         &mut self,
//         available_area: Area,
//         contextual_x_align: Option<XAlign>,
//         contextual_y_align: Option<YAlign>,
//         state: &mut State,
//     ) {
//         let Some(mut substate) = (self.scope)(state) else {
//             return;
//         };
//         let node = self.node.get_or_insert((self.tree)(&mut substate));
//         node.inner.layout(
//             available_area,
//             contextual_x_align,
//             contextual_y_align,
//             &mut substate,
//         );
//         (self.embed)(state, substate);
//     }

//     fn draw(&mut self, state: &mut State, contextual_visibility: bool) {
//         let Some(mut substate) = (self.scope)(state) else {
//             return;
//         };
//         let node = self.node.get_or_insert((self.tree)(&mut substate));
//         node.inner.draw(&mut substate, contextual_visibility);
//         (self.embed)(state, substate);
//     }
// }

// pub(crate) struct RefScoper<
//     'nodes,
//     State,
//     ScopedState,
//     Scope: Fn(&mut State) -> &mut ScopedState,
//     ScopedTree: Fn(&mut ScopedState) -> Node<'nodes, ScopedState>,
// > {
//     scope: Scope,
//     tree: ScopedTree,
//     node: Option<Node<'nodes, ScopedState>>,
//     t: PhantomData<State>,
// }

// impl<
//         'nodes,
//         State,
//         ScopedState,
//         Scope: Fn(&mut State) -> &mut ScopedState,
//         ScopedTree: Fn(&mut ScopedState) -> Node<'nodes, ScopedState>,
//     > RefScoper<'nodes, State, ScopedState, Scope, ScopedTree>
// {
//     pub(crate) fn new(scope: Scope, tree_fn: ScopedTree) -> Self {
//         Self {
//             scope,
//             tree: tree_fn,
//             node: None,
//             t: PhantomData,
//         }
//     }
// }

// impl<
//         'nodes,
//         State,
//         ScopedState,
//         Scope: Fn(&mut State) -> &mut ScopedState,
//         ScopedTree: Fn(&mut ScopedState) -> Node<'nodes, ScopedState>,
//     > Debug for RefScoper<'nodes, State, ScopedState, Scope, ScopedTree>
// {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.write_str("Kms")
//     }
// }

// impl<
//         'nodes,
//         State,
//         ScopedState,
//         Scope: Fn(&mut State) -> &mut ScopedState,
//         ScopedTree: Fn(&mut ScopedState) -> Node<'nodes, ScopedState>,
//     > NodeTrait<State> for RefScoper<'nodes, State, ScopedState, Scope, ScopedTree>
// {
//     fn constraints(&mut self, available_area: Area, state: &mut State) -> Option<SizeConstraints> {
//         let substate = (self.scope)(state);
//         let node = self.node.get_or_insert((self.tree)(substate));
//         node.inner.constraints(available_area, substate)
//     }

//     fn layout(
//         &mut self,
//         available_area: Area,
//         contextual_x_align: Option<XAlign>,
//         contextual_y_align: Option<YAlign>,
//         state: &mut State,
//     ) {
//         let substate = (self.scope)(state);
//         let node = self.node.get_or_insert((self.tree)(substate));
//         node.inner.layout(
//             available_area,
//             contextual_x_align,
//             contextual_y_align,
//             substate,
//         );
//     }

//     fn draw(&mut self, state: &mut State, contextual_visibility: bool) {
//         let substate = (self.scope)(state);
//         let node = self.node.get_or_insert((self.tree)(substate));
//         node.inner.draw(substate, contextual_visibility);
//     }
// }
