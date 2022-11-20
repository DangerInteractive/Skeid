use crate::vector::Vector;
use std::ops::{Mul, MulAssign};

impl<T, const S: usize, By> Mul<Vector<By, S>> for Vector<T, S>
where
    T: Sized + Copy + Mul<By>,
    By: Sized + Copy,
    <T as Mul<By>>::Output: Copy,
{
    type Output = Vector<<T as Mul<By>>::Output, S>;

    fn mul(self, rhs: Vector<By, S>) -> Self::Output {
        self.into_componentwise_op(rhs, move |lhs_value, rhs_value| lhs_value * rhs_value)
    }
}

impl<T, const S: usize, By> MulAssign<Vector<By, S>> for Vector<T, S>
where
    T: Sized + Copy + Mul<By, Output = T>,
    By: Sized + Copy,
{
    fn mul_assign(&mut self, rhs: Vector<By, S>) {
        self.assign_from_componentwise_op(rhs, move |lhs_value, rhs_value| lhs_value * rhs_value)
    }
}
