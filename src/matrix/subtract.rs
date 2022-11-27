use crate::matrix::Matrix;
use crate::ops::componentwise::{AssignComponentwiseOp, ComponentwiseOp};
use std::ops::{Sub, SubAssign};

impl<T, const R: usize, const C: usize, By> Sub<Matrix<By, R, C>> for Matrix<T, R, C>
where
    T: Copy + Sub<By, Output = T>,
    By: Copy,
{
    type Output = Matrix<T, R, C>;

    fn sub(self, rhs: Matrix<By, R, C>) -> Self::Output {
        self.componentwise_op(rhs, move |lhs_value: T, rhs_value: By| {
            lhs_value - rhs_value
        })
    }
}

impl<T, const R: usize, const C: usize, By> SubAssign<Matrix<By, R, C>> for Matrix<T, R, C>
where
    T: Copy + SubAssign<By>,
    By: Copy,
{
    fn sub_assign(&mut self, rhs: Matrix<By, R, C>) {
        self.assign_componentwise_op(rhs, move |lhs_value: &mut T, rhs_value: By| {
            *lhs_value -= rhs_value
        })
    }
}
