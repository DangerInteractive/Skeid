use crate::matrix::Matrix;
use crate::vector::Vector;
use std::array::from_fn;
use std::ops::{Div, DivAssign};

impl<T, const R: usize, const C: usize, By> Div<By> for Matrix<T, R, C>
where
    T: Sized + Copy + Div<By>,
    By: Sized + Copy,
    <T as Div<By>>::Output: Copy,
{
    type Output = Matrix<<T as Div<By>>::Output, R, C>;

    fn div(self, rhs: By) -> Self::Output {
        self.into_scalar_op(rhs, move |lhs_value, rhs_value| {
            Vector::from_array(from_fn(move |i| lhs_value[i] / rhs_value))
        })
    }
}

impl<T, const R: usize, const C: usize, By> DivAssign<By> for Matrix<T, R, C>
where
    T: Sized + Copy + Div<By, Output = T>,
    By: Sized + Copy,
    <T as Div<By>>::Output: Copy,
{
    fn div_assign(&mut self, rhs: By) {
        self.assign_from_scalar_op(rhs, move |lhs_value, rhs_value| {
            Vector::from_array(from_fn(move |i| lhs_value[i] / rhs_value))
        })
    }
}
