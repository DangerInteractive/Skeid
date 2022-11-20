/// The dot product operator (for vector/matrix math)
pub trait Dot<Rhs = Self> {
    /// The resulting type after applying the dot product operator
    type Output;

    /// Performs the dot product operation
    fn dot(self, rhs: Rhs) -> Self::Output;
}

/// The in-place dot product operator (for vector/matrix math)
pub trait DotAssign<Rhs = Self> {
    /// Performs the dot product operation in place (mutates self)
    fn dot_assign(&mut self, rhs: Rhs);
}
