use crate::matrix::Matrix;
use crate::vector::Vector;
use std::array::from_fn;
use std::ops::{Mul, MulAssign};

impl<T, const R: usize, const C: usize, By> Mul<By> for Matrix<T, R, C>
where
    T: Sized + Copy + Mul<By>,
    By: Sized + Copy,
    <T as Mul<By>>::Output: Copy,
{
    type Output = Matrix<<T as Mul<By>>::Output, R, C>;

    fn mul(self, rhs: By) -> Self::Output {
        self.into_scalar_op(rhs, move |lhs_value, rhs_value| {
            Vector::from_array(from_fn(move |i| lhs_value[i] * rhs_value))
        })
    }
}

impl<T, const R: usize, const C: usize, By> MulAssign<By> for Matrix<T, R, C>
where
    T: Sized + Copy + Mul<By, Output = T>,
    By: Sized + Copy,
    <T as Mul<By>>::Output: Copy,
{
    fn mul_assign(&mut self, rhs: By) {
        self.assign_from_scalar_op(rhs, move |lhs_value, rhs_value| {
            Vector::from_array(from_fn(move |i| lhs_value[i] * rhs_value))
        })
    }
}
