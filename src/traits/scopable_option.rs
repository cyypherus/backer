use super::Scopable;

/// Implement `ScopableOption` to enable usage with [`Node::scope`] for optional state.
/// For non-optional state, implement [`Scopable`].
pub trait ScopableOption<Scoping, Scoped> {
    /// Provide a scoped mutable reference to an optional subset of your state.
    ///
    /// ```rust
    /// use backer::traits::ScopableOption;
    ///
    /// struct A {
    ///     b: Option<B>,
    /// }
    ///
    /// struct B;
    ///
    /// struct Scoper;
    /// impl ScopableOption<A, B> for Scoper {
    ///     fn scope_option<Result>(scoping: &mut A, f: impl FnOnce(Option<&mut B>) -> Result) -> Result {
    ///        f(scoping.b.as_mut())
    ///     }
    /// }
    /// ```
    fn scope_option<Result>(
        scoping: &mut Scoping,
        f: impl FnOnce(Option<&mut Scoped>) -> Result,
    ) -> Result;
}

impl<T, Scoping, Scoped> ScopableOption<Scoping, Scoped> for T
where
    T: Scopable<Scoping, Scoped>,
{
    fn scope_option<Result>(
        scoping: &mut Scoping,
        f: impl FnOnce(Option<&mut Scoped>) -> Result,
    ) -> Result {
        Self::scope(scoping, |s| f(Some(s)))
    }
}
