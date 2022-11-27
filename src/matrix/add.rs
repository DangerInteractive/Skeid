use crate::matrix::Matrix;
use crate::ops::componentwise::{AssignComponentwiseOp, ComponentwiseOp};
use std::ops::{Add, AddAssign};

impl<T, const R: usize, const C: usize, By> Add<Matrix<By, R, C>> for Matrix<T, R, C>
where
    T: Copy + Add<By, Output = T>,
    By: Copy,
{
    type Output = Matrix<T, R, C>;

    fn add(self, rhs: Matrix<By, R, C>) -> Self::Output {
        self.componentwise_op(rhs, move |lhs_value: T, rhs_value: By| {
            lhs_value + rhs_value
        })
    }
}

impl<T, const R: usize, const C: usize, By> AddAssign<Matrix<By, R, C>> for Matrix<T, R, C>
where
    T: Copy + AddAssign<By>,
    By: Copy,
{
    fn add_assign(&mut self, rhs: Matrix<By, R, C>) {
        self.assign_componentwise_op(rhs, move |lhs_value: &mut T, rhs_value: By| {
            *lhs_value += rhs_value
        })
    }
}
