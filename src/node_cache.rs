use std::fmt::Debug;

use crate::{
    constraints::SizeConstraints,
    layout::NodeValue,
    models::{Area, XAlign, YAlign},
};

pub(crate) struct NodeCache<'nodes, State> {
    pub(crate) kind: NodeValue<'nodes, State>,
    cache_area: Option<Area>,
    cached_constraints: Option<SizeConstraints>,
}

impl<'nodes, State> NodeCache<'nodes, State> {
    pub(crate) fn new(kind: NodeValue<'nodes, State>) -> Self {
        Self {
            kind,
            cache_area: None,
            cached_constraints: None,
        }
    }
}

impl<'nodes, State> Debug for NodeCache<'nodes, State> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NodeCache")
            .field("kind", &self.kind)
            .field("cache_area", &self.cache_area)
            .field("cached_constraints", &self.cached_constraints)
            .finish()
    }
}

impl<State> NodeCache<'_, State> {
    pub(crate) fn constraints<'nodes, 'state>(
        &'nodes mut self,
        available_area: Area,
        state: &'state mut State,
    ) -> Option<SizeConstraints> {
        if let (Some(cache), Some(constraints)) = (self.cache_area, self.cached_constraints) {
            if cache == available_area {
                return Some(constraints);
            }
        }
        let constraints = self.kind.constraints(available_area, state);
        self.cache_area = Some(available_area);
        self.cached_constraints = constraints;
        constraints
    }
    pub(crate) fn layout(
        &mut self,
        available_area: Area,
        contextual_x_align: Option<XAlign>,
        contextual_y_align: Option<YAlign>,
        state: &mut State,
    ) {
        self.kind.layout(
            available_area,
            contextual_x_align,
            contextual_y_align,
            state,
        );
    }
    pub(crate) fn draw(&mut self, state: &mut State, contextual_visibility: bool) {
        self.kind.draw(state, contextual_visibility)
    }
}
