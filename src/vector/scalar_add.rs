use crate::marker::Scalar;
use crate::ops::componentwise::{AssignComponentwiseOp, ComponentwiseOp};
use crate::vector::Vector;
use std::ops::{Add, AddAssign};

impl<T, const R: usize, By> Add<By> for Vector<T, R>
where
    T: Copy + Add<By, Output = T>,
    By: Scalar + Copy,
{
    type Output = Vector<T, R>;

    fn add(self, rhs: By) -> Self::Output {
        self.componentwise_op(rhs, move |lhs_value, rhs_value| lhs_value + rhs_value)
    }
}

impl<T, const R: usize, By> AddAssign<By> for Vector<T, R>
where
    T: Copy + AddAssign<By>,
    By: Scalar + Copy,
{
    fn add_assign(&mut self, rhs: By) {
        self.assign_componentwise_op(rhs, move |lhs_value, rhs_value| *lhs_value += rhs_value)
    }
}
