use crate::marker::Scalar;
use crate::ops::componentwise::{AssignComponentwiseOp, ComponentwiseOp};
use crate::vector::Vector;
use std::ops::{Mul, MulAssign};

impl<T, const R: usize, By> Mul<By> for Vector<T, R>
where
    T: Copy + Mul<By, Output = T>,
    By: Scalar + Copy,
{
    type Output = Vector<T, R>;

    fn mul(self, rhs: By) -> Self::Output {
        self.componentwise_op(rhs, move |lhs_value, rhs_value| lhs_value * rhs_value)
    }
}

impl<T, const R: usize, By> MulAssign<By> for Vector<T, R>
where
    T: Copy + MulAssign<By>,
    By: Scalar + Copy,
{
    fn mul_assign(&mut self, rhs: By) {
        self.assign_componentwise_op(rhs, move |lhs_value, rhs_value| *lhs_value *= rhs_value)
    }
}
