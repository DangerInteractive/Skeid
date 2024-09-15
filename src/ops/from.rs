//! conversion between types, only because specialization is still unstable

/// An exact copy of the `From` trait from rust's standard library.
///
/// This was done to bypass the fact that the standard library makes a blanket implementation `impl From<T> for T`,
/// which essentially makes it impossible to do something like `impl<T, U> From<Vector<U>> for Vector<T>`
/// because implicitly, T and U could be identical types,
/// and rust has no way of expressing the type constraint that `T` and `U` must not be the same type,
/// and rust still hasn't stabilized specialization yet,
/// and therefore it is impossible to implement the standard `From` trait in this way without conflicting.
///
/// This will (hopefully) be replaced with the standard library's `From` trait someday.
/// Prerequisites for this are either:
/// - specialization is ever stabilized (the RFC was written in 2015, and we're still waiting...),
/// - or it ever becomes possible to write a trait bound expressing: `where T is not U`.
pub trait From<T> {
    /// convert `T` into `Self`
    fn from(value: T) -> Self;
}
