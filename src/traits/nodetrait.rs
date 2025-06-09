use crate::{
    constraints::SizeConstraints,
    models::{Area, XAlign, YAlign},
};
use std::fmt::Debug;

pub(crate) trait NodeTrait<T, U>: Debug {
    fn constraints(
        &mut self,
        available_area: Area,
        t: &mut T,
        u: &mut U,
    ) -> Option<SizeConstraints>;
    fn layout(
        &mut self,
        available_area: Area,
        contextual_x_align: Option<XAlign>,
        contextual_y_align: Option<YAlign>,
        t: &mut T,
        u: &mut U,
    );
    fn draw(&mut self, t: &mut T, u: &mut U, contextual_visibility: bool);
}
