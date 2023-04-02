//! math operator for calculating cross products (for linear algebra)

/// math operator for calculating cross products (for linear algebra)
pub trait Cross<Rhs = Self> {
    /// the type returned by [Cross::cross](Cross::cross)
    type Output;

    /// calculate cross product
    fn cross(self, rhs: Rhs) -> Self::Output;
}
