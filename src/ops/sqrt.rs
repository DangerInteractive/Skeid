//! math operator for calculating square roots

/// math operator for calculating square roots
pub trait Sqrt {
    /// the type returned by [Sqrt::sqrt](Sqrt::sqrt)
    type Output;

    /// calculate square root
    fn sqrt(self) -> Self::Output;
}

impl Sqrt for f32 {
    type Output = f32;

    fn sqrt(self) -> Self::Output {
        self.sqrt()
    }
}

impl Sqrt for f64 {
    type Output = f64;

    fn sqrt(self) -> Self::Output {
        self.sqrt()
    }
}
