use crate::marker::Scalar;
use crate::ops::componentwise::{AssignComponentwiseOp, ComponentwiseOp};
use crate::vector::Vector;
use std::ops::{Sub, SubAssign};

impl<T, const R: usize, By> Sub<By> for Vector<T, R>
where
    T: Copy + Sub<By, Output = T>,
    By: Scalar + Copy,
{
    type Output = Vector<T, R>;

    fn sub(self, rhs: By) -> Self::Output {
        self.componentwise_op(rhs, move |lhs_value, rhs_value| lhs_value - rhs_value)
    }
}

impl<T, const R: usize, By> SubAssign<By> for Vector<T, R>
where
    T: Copy + SubAssign<By>,
    By: Scalar + Copy,
{
    fn sub_assign(&mut self, rhs: By) {
        self.assign_componentwise_op(rhs, move |lhs_value, rhs_value| *lhs_value -= rhs_value)
    }
}
