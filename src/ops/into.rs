//! conversion between types, only because specialization is still unstable

/// An exact copy of the `Into` trait from rust's standard library.
///
/// There is a blanket implementation using our custom `From` trait, so nothing should implement this directly.
///
/// This was done to bypass the fact that the standard library makes a blanket implementation `impl<T, U> Into<U> for T where U: From<T>`,
/// which then hits the blanket implementation `impl From<T> for T` also from the standard library,
/// which essentially makes it impossible to do something like `impl<T, U> From<Vector<U>> for Vector<T>`
/// because implicitly, T and U could be identical types,
/// and Rust has no way of expressing the type constraint that `T` and `U` must not be the same type,
/// and Rust still hasn't stabilized specialization yet (and may never do so at this rate),
/// and therefore it is impossible to implement the standard `Into` trait in this way without conflicting.
///
/// This will (hopefully) be replaced with the standard library's `Into` trait someday.
/// Prerequisites for this are either:
/// - specialization is ever stabilized (the RFC was written in 2015, and we're still waiting...),
/// - or it ever becomes possible to write a trait bound expressing: `where T is not U`.
pub trait Into<T> {
    /// convert `Self` into `T`
    fn into(self) -> T;
}

/// the same blanket impl as the standard library, nothing should need to implement this directly
impl<T, U> Into<U> for T
where
    U: From<T>,
{
    fn into(self) -> U {
        U::from(self)
    }
}
