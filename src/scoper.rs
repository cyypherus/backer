use crate::{
    constraints::SizeConstraints,
    models::{Area, XAlign, YAlign},
    traits::NodeTrait,
    Node,
};
use std::fmt::Debug;

pub(crate) struct Scoper<'nodes, ScopedT, ScopedU, ScopeT, ScopeU> {
    pub(crate) scope_t: ScopeT,
    pub(crate) scope_u: ScopeU,
    pub(crate) node: Node<'nodes, ScopedT, ScopedU>,
}

impl<ScopedT, ScopedU, ScopeT, ScopeU> Debug for Scoper<'_, ScopedT, ScopedU, ScopeT, ScopeU> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Scoper")
            .field("scope", &"<function>")
            .field("node", &self.node)
            .finish()
    }
}

impl<T, U, ScopedT, ScopedU, ScopeT, ScopeU> NodeTrait<T, U>
    for Scoper<'_, ScopedT, ScopedU, ScopeT, ScopeU>
where
    ScopeT: Fn(&mut T) -> &mut ScopedT,
    ScopeU: Fn(&mut U) -> &mut ScopedU,
{
    fn constraints(
        &mut self,
        available_area: Area,
        t: &mut T,
        u: &mut U,
    ) -> Option<SizeConstraints> {
        let scoped_t = (self.scope_t)(t);
        let scoped_u = (self.scope_u)(u);
        self.node
            .inner
            .constraints(available_area, scoped_t, scoped_u)
    }

    fn layout(
        &mut self,
        available_area: Area,
        contextual_x_align: Option<XAlign>,
        contextual_y_align: Option<YAlign>,
        t: &mut T,
        u: &mut U,
    ) {
        let scoped_t = (self.scope_t)(t);
        let scoped_u = (self.scope_u)(u);
        self.node.inner.layout(
            available_area,
            contextual_x_align,
            contextual_y_align,
            scoped_t,
            scoped_u,
        );
    }

    fn draw(&mut self, t: &mut T, u: &mut U, contextual_visibility: bool) {
        let scoped_t = (self.scope_t)(t);
        let scoped_u = (self.scope_u)(u);
        self.node
            .inner
            .draw(scoped_t, scoped_u, contextual_visibility);
    }
}

pub(crate) struct OptionScoper<'nodes, ScopedT, ScopedU, ScopeT, ScopeU> {
    pub(crate) scope_t: ScopeT,
    pub(crate) scope_u: ScopeU,
    pub(crate) node: Node<'nodes, ScopedT, ScopedU>,
}

impl<ScopedT, ScopedU, ScopeT, ScopeU> Debug
    for OptionScoper<'_, ScopedT, ScopedU, ScopeT, ScopeU>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OptionScoper")
            .field("scope", &"<function>")
            .field("node", &self.node)
            .finish()
    }
}

impl<T, U, ScopedT, ScopedU, ScopeT, ScopeU> NodeTrait<T, U>
    for OptionScoper<'_, ScopedT, ScopedU, ScopeT, ScopeU>
where
    ScopeT: Fn(&mut T) -> &mut Option<ScopedT>,
    ScopeU: Fn(&mut U) -> &mut Option<ScopedU>,
{
    fn constraints(
        &mut self,
        available_area: Area,
        t: &mut T,
        u: &mut U,
    ) -> Option<SizeConstraints> {
        if let (Some(scoped_t), Some(scoped_u)) = ((self.scope_t)(t), (self.scope_u)(u)) {
            self.node
                .inner
                .constraints(available_area, scoped_t, scoped_u)
        } else {
            None
        }
    }

    fn layout(
        &mut self,
        available_area: Area,
        contextual_x_align: Option<XAlign>,
        contextual_y_align: Option<YAlign>,
        t: &mut T,
        u: &mut U,
    ) {
        if let (Some(scoped_t), Some(scoped_u)) = ((self.scope_t)(t), (self.scope_u)(u)) {
            self.node.inner.layout(
                available_area,
                contextual_x_align,
                contextual_y_align,
                scoped_t,
                scoped_u,
            )
        }
    }

    fn draw(&mut self, t: &mut T, u: &mut U, contextual_visibility: bool) {
        if let (Some(scoped_t), Some(scoped_u)) = ((self.scope_t)(t), (self.scope_u)(u)) {
            self.node
                .inner
                .draw(scoped_t, scoped_u, contextual_visibility)
        }
    }
}

pub(crate) struct OwnedScoper<'nodes, ScopedT, ScopedU, ScopeT, ScopeU, Embed> {
    pub(crate) scope_t: ScopeT,
    pub(crate) scope_u: ScopeU,
    pub(crate) embed: Embed,
    pub(crate) node: Node<'nodes, ScopedT, ScopedU>,
}

impl<ScopedT, ScopedU, ScopeT, ScopeU, Embed> Debug
    for OwnedScoper<'_, ScopedT, ScopedU, ScopeT, ScopeU, Embed>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OwnedScoper")
            .field("scope", &"<function>")
            .field("node", &self.node)
            .finish()
    }
}

impl<T, U, ScopedT, ScopedU, ScopeT, ScopeU, Embed> NodeTrait<T, U>
    for OwnedScoper<'_, ScopedT, ScopedU, ScopeT, ScopeU, Embed>
where
    ScopeT: Fn(&mut T) -> ScopedT,
    ScopeU: Fn(&mut U) -> ScopedU,
    Embed: Fn(&mut T, ScopedT, &mut U, ScopedU),
{
    fn constraints(
        &mut self,
        available_area: Area,
        t: &mut T,
        u: &mut U,
    ) -> Option<SizeConstraints> {
        let mut scoped_t = (self.scope_t)(t);
        let mut scoped_u = (self.scope_u)(u);
        let result = self
            .node
            .inner
            .constraints(available_area, &mut scoped_t, &mut scoped_u);
        (self.embed)(t, scoped_t, u, scoped_u);
        result
    }

    fn layout(
        &mut self,
        available_area: Area,
        contextual_x_align: Option<XAlign>,
        contextual_y_align: Option<YAlign>,
        t: &mut T,
        u: &mut U,
    ) {
        let mut scoped_t = (self.scope_t)(t);
        let mut scoped_u = (self.scope_u)(u);
        self.node.inner.layout(
            available_area,
            contextual_x_align,
            contextual_y_align,
            &mut scoped_t,
            &mut scoped_u,
        );
        (self.embed)(t, scoped_t, u, scoped_u);
    }

    fn draw(&mut self, t: &mut T, u: &mut U, contextual_visibility: bool) {
        let mut scoped_t = (self.scope_t)(t);
        let mut scoped_u = (self.scope_u)(u);
        self.node
            .inner
            .draw(&mut scoped_t, &mut scoped_u, contextual_visibility);
        (self.embed)(t, scoped_t, u, scoped_u);
    }
}
