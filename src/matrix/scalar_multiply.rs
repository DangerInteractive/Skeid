use crate::marker::Scalar;
use crate::matrix::Matrix;
use crate::ops::componentwise::{AssignComponentwise, Componentwise};
use std::ops::{Mul, MulAssign};

impl<T, const ROWS: usize, const COLUMNS: usize, Rhs> Mul<Rhs> for Matrix<T, ROWS, COLUMNS>
where
    T: Copy + Mul<Rhs>,
    Rhs: Scalar + Copy,
    <T as Mul<Rhs>>::Output: Copy,
{
    type Output = Matrix<<T as Mul<Rhs>>::Output, ROWS, COLUMNS>;

    fn mul(self, rhs: Rhs) -> Self::Output {
        self.componentwise(rhs, |lhs_value, rhs_value| lhs_value * rhs_value)
    }
}

impl<T, const ROWS: usize, const COLUMNS: usize, Rhs> MulAssign<Rhs> for Matrix<T, ROWS, COLUMNS>
where
    T: Copy + MulAssign<Rhs>,
    Rhs: Scalar + Copy,
{
    fn mul_assign(&mut self, rhs: Rhs) {
        self.assign_componentwise(rhs, |lhs_value, rhs_value| *lhs_value *= rhs_value)
    }
}
