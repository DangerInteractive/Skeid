/// The cross product operator (for vector/matrix math)
pub trait Cross<Rhs = Self> {
    /// The resulting type after applying the cross product operator
    type Output;

    /// Performs the cross product operation
    fn cross(self, rhs: Rhs) -> Self::Output;
}

/// The in-place cross product operator (for vector/matrix math)
pub trait CrossAssign<Rhs = Self> {
    /// Performs the cross product operation in place (mutates self)
    fn cross_assign(&mut self, rhs: Rhs);
}
