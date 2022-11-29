use crate::matrix::Matrix;
use crate::ops::componentwise::{AssignComponentwiseOp, ComponentwiseOp};
use std::ops::{Mul, MulAssign};

impl<T, const ROWS: usize, const COLUMNS: usize, Rhs> Mul<Rhs> for Matrix<T, ROWS, COLUMNS>
where
    T: Copy + Mul<Rhs>,
    Rhs: Copy,
    <T as Mul<Rhs>>::Output: Copy,
{
    type Output = Matrix<<T as Mul<Rhs>>::Output, ROWS, COLUMNS>;

    fn mul(self, rhs: Rhs) -> Self::Output {
        self.componentwise_op(rhs, move |lhs_value, rhs_value| lhs_value * rhs_value)
    }
}

impl<T, const ROWS: usize, const COLUMNS: usize, Rhs> MulAssign<Rhs> for Matrix<T, ROWS, COLUMNS>
where
    T: Copy + MulAssign<Rhs>,
    Rhs: Copy,
{
    fn mul_assign(&mut self, rhs: Rhs) {
        self.assign_componentwise_op(rhs, move |lhs_value, rhs_value| *lhs_value *= rhs_value)
    }
}
