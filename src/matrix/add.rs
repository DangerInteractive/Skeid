use crate::matrix::Matrix;
use std::ops::{Add, AddAssign};

impl<T, const R: usize, const C: usize, By> Add<Matrix<By, R, C>> for Matrix<T, R, C>
where
    T: Sized + Copy + Add<By>,
    By: Sized + Copy,
    <T as Add<By>>::Output: Copy,
{
    type Output = Matrix<<T as Add<By>>::Output, R, C>;

    fn add(self, rhs: Matrix<By, R, C>) -> Self::Output {
        self.into_componentwise_op(rhs, move |lhs_value, rhs_value| lhs_value + rhs_value)
    }
}

impl<T, const R: usize, const C: usize, By> AddAssign<Matrix<By, R, C>> for Matrix<T, R, C>
where
    T: Sized + Copy + Add<By, Output = T>,
    By: Sized + Copy,
    <T as Add<By>>::Output: Copy,
{
    fn add_assign(&mut self, rhs: Matrix<By, R, C>) {
        self.assign_from_componentwise_op(rhs, move |lhs_value, rhs_value| lhs_value + rhs_value)
    }
}
