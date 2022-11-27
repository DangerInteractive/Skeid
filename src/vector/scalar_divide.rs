use crate::marker::Scalar;
use crate::ops::componentwise::{AssignComponentwiseOp, ComponentwiseOp};
use crate::vector::Vector;
use std::ops::{Div, DivAssign};

impl<T, const R: usize, By> Div<By> for Vector<T, R>
where
    T: Copy + Div<By, Output = T>,
    By: Scalar + Copy,
{
    type Output = Vector<T, R>;

    fn div(self, rhs: By) -> Self::Output {
        self.componentwise_op(rhs, move |lhs_value, rhs_value| lhs_value / rhs_value)
    }
}

impl<T, const R: usize, By> DivAssign<By> for Vector<T, R>
where
    T: Copy + DivAssign<By>,
    By: Scalar + Copy,
{
    fn div_assign(&mut self, rhs: By) {
        self.assign_componentwise_op(rhs, move |lhs_value, rhs_value| *lhs_value /= rhs_value)
    }
}
