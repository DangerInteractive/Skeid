use crate::ops::componentwise::{AssignComponentwiseOp, ComponentwiseOp};
use crate::vector::Vector;
use std::ops::{Sub, SubAssign};

impl<T, const R: usize, By> Sub<Vector<By, R>> for Vector<T, R>
where
    T: Copy + Sub<By, Output = T>,
    By: Copy,
{
    type Output = Vector<T, R>;

    fn sub(self, rhs: Vector<By, R>) -> Self::Output {
        self.componentwise_op(rhs, move |lhs_value: T, rhs_value: By| {
            lhs_value - rhs_value
        })
    }
}

impl<T, const R: usize, By> SubAssign<Vector<By, R>> for Vector<T, R>
where
    T: Copy + SubAssign<By>,
    By: Copy,
{
    fn sub_assign(&mut self, rhs: Vector<By, R>) {
        self.assign_componentwise_op(rhs, move |lhs_value: &mut T, rhs_value: By| {
            *lhs_value -= rhs_value
        })
    }
}
