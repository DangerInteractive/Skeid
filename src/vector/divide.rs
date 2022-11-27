use crate::ops::componentwise::{AssignComponentwiseOp, ComponentwiseOp};
use crate::vector::Vector;
use std::ops::{Div, DivAssign};

impl<T, const R: usize, By> Div<Vector<By, R>> for Vector<T, R>
where
    T: Copy + Div<By, Output = T>,
    By: Copy,
{
    type Output = Vector<T, R>;

    fn div(self, rhs: Vector<By, R>) -> Self::Output {
        self.componentwise_op(rhs, move |lhs_value: T, rhs_value: By| {
            lhs_value / rhs_value
        })
    }
}

impl<T, const R: usize, By> DivAssign<Vector<By, R>> for Vector<T, R>
where
    T: Copy + DivAssign<By>,
    By: Copy,
{
    fn div_assign(&mut self, rhs: Vector<By, R>) {
        self.assign_componentwise_op(rhs, move |lhs_value: &mut T, rhs_value: By| {
            *lhs_value /= rhs_value
        })
    }
}
