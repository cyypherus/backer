use crate::{constraints::SizeConstraints, models::Area, traits::Drawable};
use lilt::Animated;
pub use lilt::Easing;
use std::{
    collections::HashMap,
    hash::{DefaultHasher, Hash, Hasher},
    time::Instant,
};

#[macro_export]
/// Simple source code based identifier
macro_rules! id {
    () => {{
        format!("{}:{}:{}", file!(), line!(), column!())
    }};
}
// pub mod const_simple_hash {
//     /// A simple compile-time hash function based on FNV-1a
//     pub const fn simple_hash(input: &str) -> u64 {
//         const SEED: u64 = 0xcbf29ce484222325; // FNV offset basis
//         const PRIME: u64 = 0x100000001b3; // FNV prime
//         let mut hash = SEED;
//         let bytes = input.as_bytes();
//         let mut i = 0;
//         while i < bytes.len() {
//             hash ^= bytes[i] as u64;
//             hash = hash.wrapping_mul(PRIME);
//             i += 1;
//         }
//         hash
//     }
// }

// #[macro_export]
// /// Simple source code-based identifier with compile-time hashing
// macro_rules! id {
//     () => {{
//         const HASH: u64 = {
//             // Concatenate file, line, and column information
//             let input = concat!(file!(), ":", line!(), ":", column!());
//             // Compute the hash using the helper function
//             backer::transitions::const_simple_hash::simple_hash(input)
//         };
//         HASH
//     }};
//     ($input:expr) => {{
//         const HASH: u64 = {
//             // Concatenate user input with file, line, and column information
//             let input = concat!($input, ":", file!(), ":", line!(), ":", column!());
//             // Compute the hash using the helper function
//             backer::transitions::const_simple_hash::simple_hash(input)
//         };
//         HASH
//     }};
// }

impl<State: TransitionState, T: TransitionDrawable<State>> Drawable<State> for T {
    fn draw(&mut self, area: Area, state: &mut State, visible: bool) {
        let now = Instant::now();
        let mut hasher = DefaultHasher::new();
        self.id().hash(&mut hasher);
        let hsh = hasher.finish();
        let mut bank = state.bank().clone();
        let mut anim = bank.animations.remove(&hsh).unwrap_or(AnimArea {
            visible: Animated::new(visible)
                .duration(self.duration())
                .easing(self.easing())
                .delay(self.delay()),
            x: Animated::new(area.x)
                .duration(self.duration())
                .easing(self.easing())
                .delay(self.delay()),
            y: Animated::new(area.y)
                .duration(self.duration())
                .easing(self.easing())
                .delay(self.delay()),
            width: Animated::new(area.width)
                .duration(self.duration())
                .easing(self.easing())
                .delay(self.delay()),
            height: Animated::new(area.height)
                .duration(self.duration())
                .easing(self.easing())
                .delay(self.delay()),
        });
        anim.visible.transition(visible, now);
        anim.x.transition(area.x, now);
        anim.y.transition(area.y, now);
        anim.width.transition(area.width, now);
        anim.height.transition(area.height, now);
        if visible || anim.visible.in_progress(now) {
            self.draw_interpolated(
                Area {
                    x: anim.x.animate_wrapped(now),
                    y: anim.y.animate_wrapped(now),
                    width: anim.width.animate_wrapped(now),
                    height: anim.height.animate_wrapped(now),
                },
                state,
                visible,
                anim.visible.animate_bool(0., 1., now),
            )
        }
        bank.animations.insert(hsh, anim);
        *state.bank() = bank;
    }
    fn constraints(
        &mut self,
        available_area: Area,
        state: &mut State,
    ) -> Option<crate::constraints::SizeConstraints> {
        <Self as TransitionDrawable<State>>::constraints(self, available_area, state)
    }
}
/// A drawable object with interpolated transitions & layout
pub trait TransitionDrawable<State: TransitionState> {
    /// Draws the content with interpolated area & visibility
    fn draw_interpolated(
        &mut self,
        area: Area,
        state: &mut State,
        visible: bool,
        visible_amount: f32,
    );
    fn constraints(&self, available_area: Area, state: &mut State) -> Option<SizeConstraints>;
    /// Uniquely identifies the drawable across renders for interpolation
    fn id(&self) -> &u64;
    /// The easing curve to use for interpolation
    fn easing(&self) -> Easing;
    /// The duration of interpolation
    fn duration(&self) -> f32;
    /// The delay of the interpolation
    fn delay(&self) -> f32;
}
/// Implements storage for animation state
pub trait TransitionState {
    /// Returns mutable access to a stored AnimationBank where animation state is stored
    fn bank(&mut self) -> &mut AnimationBank;
}
#[derive(Debug, Clone)]
struct AnimArea {
    visible: Animated<bool, Instant>,
    x: Animated<f32, Instant>,
    y: Animated<f32, Instant>,
    width: Animated<f32, Instant>,
    height: Animated<f32, Instant>,
}
#[derive(Debug, Clone)]
/// State storage for animation state
pub struct AnimationBank {
    animations: HashMap<u64, AnimArea>,
}
impl Default for AnimationBank {
    fn default() -> Self {
        Self::new()
    }
}
impl AnimationBank {
    /// Initialize an empty `AnimationBank`
    pub fn new() -> Self {
        Self {
            animations: HashMap::new(),
        }
    }
    /// Checks if any animations are currently in progress
    pub fn in_progress(&self, time: Instant) -> bool {
        for value in self.animations.values() {
            if value.visible.in_progress(time)
                || value.x.in_progress(time)
                || value.y.in_progress(time)
                || value.width.in_progress(time)
                || value.height.in_progress(time)
            {
                return true;
            }
        }
        false
    }
}
