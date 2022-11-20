use crate::vector::Vector;
use std::ops::{Div, DivAssign};

impl<T, const S: usize, By> Div<Vector<By, S>> for Vector<T, S>
where
    T: Sized + Copy + Div<By>,
    By: Sized + Copy,
    <T as Div<By>>::Output: Copy,
{
    type Output = Vector<<T as Div<By>>::Output, S>;

    fn div(self, rhs: Vector<By, S>) -> Self::Output {
        self.into_componentwise_op(rhs, move |lhs_value, rhs_value| lhs_value / rhs_value)
    }
}

impl<T, const S: usize, By> DivAssign<Vector<By, S>> for Vector<T, S>
where
    T: Sized + Copy + Div<By, Output = T>,
    By: Sized + Copy,
{
    fn div_assign(&mut self, rhs: Vector<By, S>) {
        self.assign_from_componentwise_op(rhs, move |lhs_value, rhs_value| lhs_value / rhs_value)
    }
}
