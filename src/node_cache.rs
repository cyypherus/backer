use std::fmt::Debug;

use crate::{
    constraints::SizeConstraints,
    layout::NodeValue,
    models::{Area, XAlign, YAlign},
};

pub(crate) struct NodeCache<'nodes, T, U: Copy> {
    pub(crate) kind: NodeValue<'nodes, T, U>,
    pub(crate) cache_area: Option<Area>,
    pub(crate) cached_constraints: Option<SizeConstraints>,
}

impl<'nodes, T, U: Copy> NodeCache<'nodes, T, U> {
    pub(crate) fn new(kind: NodeValue<'nodes, T, U>) -> Self {
        Self {
            kind,
            cache_area: None,
            cached_constraints: None,
        }
    }
}

impl<T, U: Copy> Debug for NodeCache<'_, T, U> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NodeCache")
            .field("kind", &self.kind)
            .field("cache_area", &self.cache_area)
            .field("cached_constraints", &self.cached_constraints)
            .finish()
    }
}

impl<T, Lens: Copy> NodeCache<'_, T, Lens> {
    pub(crate) fn constraints(
        &mut self,
        available_area: Area,
        t: &mut T,
        u: Lens,
    ) -> Option<SizeConstraints> {
        if let (Some(cache), Some(constraints)) = (self.cache_area, self.cached_constraints) {
            if cache == available_area {
                return Some(constraints);
            }
        }
        let constraints = self.kind.constraints(available_area, t, u);
        self.cache_area = Some(available_area);
        self.cached_constraints = constraints;
        constraints
    }
    pub(crate) fn layout(
        &mut self,
        available_area: Area,
        contextual_x_align: Option<XAlign>,
        contextual_y_align: Option<YAlign>,
        state: &mut T,
        lens: Lens,
    ) {
        self.kind.layout(
            available_area,
            contextual_x_align,
            contextual_y_align,
            state,
            lens,
        );
    }
    pub(crate) fn draw(&mut self, t: &mut T, lens: Lens, contextual_visibility: bool) {
        self.kind.draw(t, lens, contextual_visibility)
    }
}
