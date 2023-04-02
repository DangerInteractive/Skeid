//! math operator for exponentiation

/// math operator for exponentiation
pub trait Pow<T> {
    /// the type returned by [Pow::pow](Pow::pow)
    type Output;

    /// calculate exponentiation, with `self` as the base, and taking an exponent as an argument
    fn pow(self, pow: T) -> Self::Output;
}

impl Pow<i32> for f32 {
    type Output = f32;

    fn pow(self, pow: i32) -> Self::Output {
        self.powi(pow)
    }
}

impl Pow<f32> for f32 {
    type Output = f32;

    fn pow(self, pow: f32) -> Self::Output {
        self.powf(pow)
    }
}

impl Pow<i32> for f64 {
    type Output = f64;

    fn pow(self, pow: i32) -> Self::Output {
        self.powi(pow)
    }
}

impl Pow<f64> for f64 {
    type Output = f64;

    fn pow(self, pow: f64) -> Self::Output {
        self.powf(pow)
    }
}
