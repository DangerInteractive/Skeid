use crate::matrix::Matrix;
use std::ops::{Sub, SubAssign};

impl<T, const R: usize, const C: usize, By> Sub<Matrix<By, R, C>> for Matrix<T, R, C>
where
    T: Sized + Copy + Sub<By>,
    By: Sized + Copy,
    <T as Sub<By>>::Output: Copy,
{
    type Output = Matrix<<T as Sub<By>>::Output, R, C>;

    fn sub(self, rhs: Matrix<By, R, C>) -> Self::Output {
        self.into_componentwise_op(rhs, move |lhs_value, rhs_value| lhs_value - rhs_value)
    }
}

impl<T, const R: usize, const C: usize, By> SubAssign<Matrix<By, R, C>> for Matrix<T, R, C>
where
    T: Sized + Copy + Sub<By, Output = T>,
    By: Sized + Copy,
    <T as Sub<By>>::Output: Copy,
{
    fn sub_assign(&mut self, rhs: Matrix<By, R, C>) {
        self.assign_from_componentwise_op(rhs, move |lhs_value, rhs_value| lhs_value - rhs_value)
    }
}
