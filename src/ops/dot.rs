/// The dot product operator (for vector/matrix math)
pub trait Dot<Rhs = Self> {
    /// The resulting type after applying the dot product operator
    type Output;

    /// Performs the dot product operation
    fn dot(self, rhs: Rhs) -> Self::Output;
}
