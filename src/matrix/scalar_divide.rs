use crate::matrix::Matrix;
use crate::ops::componentwise::{AssignComponentwiseOp, ComponentwiseOp};
use std::ops::{Div, DivAssign};

impl<T, const R: usize, const C: usize, By> Div<By> for Matrix<T, R, C>
where
    T: Copy + Div<By, Output = T>,
    By: Copy,
{
    type Output = Matrix<T, R, C>;

    fn div(self, rhs: By) -> Self::Output {
        self.componentwise_op(rhs, move |lhs_value, rhs_value| lhs_value / rhs_value)
    }
}

impl<T, const R: usize, const C: usize, By> DivAssign<By> for Matrix<T, R, C>
where
    T: Copy + DivAssign<By>,
    By: Copy,
{
    fn div_assign(&mut self, rhs: By) {
        self.assign_componentwise_op(rhs, move |lhs_value, rhs_value| *lhs_value /= rhs_value)
    }
}
