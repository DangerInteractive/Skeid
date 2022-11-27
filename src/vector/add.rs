use crate::ops::componentwise::{AssignComponentwiseOp, ComponentwiseOp};
use crate::vector::Vector;
use std::ops::{Add, AddAssign};

impl<T, const R: usize, By> Add<Vector<By, R>> for Vector<T, R>
where
    T: Copy + Add<By, Output = T>,
    By: Copy,
{
    type Output = Vector<T, R>;

    fn add(self, rhs: Vector<By, R>) -> Self::Output {
        self.componentwise_op(rhs, move |lhs_value: T, rhs_value: By| {
            lhs_value + rhs_value
        })
    }
}

impl<T, const R: usize, By> AddAssign<Vector<By, R>> for Vector<T, R>
where
    T: Copy + AddAssign<By>,
    By: Copy,
{
    fn add_assign(&mut self, rhs: Vector<By, R>) {
        self.assign_componentwise_op(rhs, move |lhs_value: &mut T, rhs_value: By| {
            *lhs_value += rhs_value
        })
    }
}
