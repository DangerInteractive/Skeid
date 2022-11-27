use crate::ops::componentwise::{AssignComponentwiseOp, ComponentwiseOp};
use crate::vector::Vector;
use std::ops::{Mul, MulAssign};

impl<T, const R: usize, By> Mul<Vector<By, R>> for Vector<T, R>
where
    T: Copy + Mul<By, Output = T>,
    By: Copy,
{
    type Output = Vector<T, R>;

    fn mul(self, rhs: Vector<By, R>) -> Self::Output {
        self.componentwise_op(rhs, move |lhs_value: T, rhs_value: By| {
            lhs_value * rhs_value
        })
    }
}

impl<T, const R: usize, By> MulAssign<Vector<By, R>> for Vector<T, R>
where
    T: Copy + MulAssign<By>,
    By: Copy,
{
    fn mul_assign(&mut self, rhs: Vector<By, R>) {
        self.assign_componentwise_op(rhs, move |lhs_value: &mut T, rhs_value: By| {
            *lhs_value *= rhs_value
        })
    }
}
