//! math operator for calculating dot products (for linear algebra)

/// math operator for calculating dot products (for linear algebra)
pub trait Dot<Rhs = Self> {
    /// the type returned by [Dot::dot](Dot::dot)
    type Output;

    /// calculate dot product
    fn dot(self, rhs: Rhs) -> Self::Output;
}
