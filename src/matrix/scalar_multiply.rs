use crate::matrix::Matrix;
use crate::ops::componentwise::{AssignComponentwiseOp, ComponentwiseOp};
use std::ops::{Mul, MulAssign};

impl<T, const R: usize, const C: usize, By> Mul<By> for Matrix<T, R, C>
where
    T: Copy + Mul<By, Output = T>,
    By: Copy,
{
    type Output = Matrix<T, R, C>;

    fn mul(self, rhs: By) -> Self::Output {
        self.componentwise_op(rhs, move |lhs_value, rhs_value| lhs_value * rhs_value)
    }
}

impl<T, const R: usize, const C: usize, By> MulAssign<By> for Matrix<T, R, C>
where
    T: Copy + MulAssign<By>,
    By: Copy,
{
    fn mul_assign(&mut self, rhs: By) {
        self.assign_componentwise_op(rhs, move |lhs_value, rhs_value| *lhs_value *= rhs_value)
    }
}
