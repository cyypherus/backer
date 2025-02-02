use crate::{
    constraints::SizeConstraints,
    models::{Area, XAlign, YAlign},
    traits::NodeTrait,
    Node,
};
use std::fmt::Debug;

pub(crate) struct Scoper<'nodes, ScopedState, Scope> {
    pub(crate) scope: Scope,
    pub(crate) node: Node<'nodes, ScopedState>,
}

impl<ScopedState, Scope> Debug for Scoper<'_, ScopedState, Scope> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Scoper")
            .field("scope", &"<function>")
            .field("node", &self.node)
            .finish()
    }
}

impl<State, ScopedState, Scope> NodeTrait<State> for Scoper<'_, ScopedState, Scope>
where
    Scope: Fn(&mut State) -> &mut ScopedState,
{
    fn constraints(&mut self, available_area: Area, state: &mut State) -> Option<SizeConstraints> {
        let substate = (self.scope)(state);
        self.node.inner.constraints(available_area, substate)
    }

    fn layout(
        &mut self,
        available_area: Area,
        contextual_x_align: Option<XAlign>,
        contextual_y_align: Option<YAlign>,
        state: &mut State,
    ) {
        let substate = (self.scope)(state);
        self.node.inner.layout(
            available_area,
            contextual_x_align,
            contextual_y_align,
            substate,
        );
    }

    fn draw(&mut self, state: &mut State, contextual_visibility: bool) {
        let substate = (self.scope)(state);
        self.node.inner.draw(substate, contextual_visibility);
    }
}

pub(crate) struct OptionScoper<'nodes, ScopedState, Scope> {
    pub(crate) scope: Scope,
    pub(crate) node: Node<'nodes, ScopedState>,
}

impl<ScopedState, Scope> Debug for OptionScoper<'_, ScopedState, Scope> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OptionScoper")
            .field("scope", &"<function>")
            .field("node", &self.node)
            .finish()
    }
}

impl<State, ScopedState, Scope> NodeTrait<State> for OptionScoper<'_, ScopedState, Scope>
where
    Scope: Fn(&mut State) -> &mut Option<ScopedState>,
{
    fn constraints(&mut self, available_area: Area, state: &mut State) -> Option<SizeConstraints> {
        if let Some(substate) = (self.scope)(state) {
            self.node.inner.constraints(available_area, substate)
        } else {
            None
        }
    }

    fn layout(
        &mut self,
        available_area: Area,
        contextual_x_align: Option<XAlign>,
        contextual_y_align: Option<YAlign>,
        state: &mut State,
    ) {
        if let Some(substate) = (self.scope)(state) {
            self.node.inner.layout(
                available_area,
                contextual_x_align,
                contextual_y_align,
                substate,
            )
        }
    }

    fn draw(&mut self, state: &mut State, contextual_visibility: bool) {
        if let Some(substate) = (self.scope)(state) {
            self.node.inner.draw(substate, contextual_visibility)
        }
    }
}

pub(crate) struct OwnedScoper<'nodes, ScopedState, Scope, Embed> {
    pub(crate) scope: Scope,
    pub(crate) embed: Embed,
    pub(crate) node: Node<'nodes, ScopedState>,
}

impl<ScopedState, Scope, Embed> Debug for OwnedScoper<'_, ScopedState, Scope, Embed> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OwnedScoper")
            .field("scope", &"<function>")
            .field("node", &self.node)
            .finish()
    }
}

impl<State, ScopedState, Scope, Embed> NodeTrait<State>
    for OwnedScoper<'_, ScopedState, Scope, Embed>
where
    Scope: Fn(&mut State) -> ScopedState,
    Embed: Fn(&mut State, ScopedState),
{
    fn constraints(&mut self, available_area: Area, state: &mut State) -> Option<SizeConstraints> {
        let mut substate = (self.scope)(state);
        let result = self.node.inner.constraints(available_area, &mut substate);
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
        self.node.inner.layout(
            available_area,
            contextual_x_align,
            contextual_y_align,
            &mut substate,
        );
        (self.embed)(state, substate);
    }

    fn draw(&mut self, state: &mut State, contextual_visibility: bool) {
        let mut substate = (self.scope)(state);
        self.node.inner.draw(&mut substate, contextual_visibility);
        (self.embed)(state, substate);
    }
}
