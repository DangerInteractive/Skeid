/// The cross product operator (for vector/matrix math)
pub trait Cross<Rhs = Self> {
    /// The resulting type after applying the cross product operator
    type Output;

    /// Performs the cross product operation
    fn cross(self, rhs: Rhs) -> Self::Output;
}
