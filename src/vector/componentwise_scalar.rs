use crate::ops::componentwise::{AssignComponentwiseOp, ComponentwiseOp};
use crate::vector::Vector;
use std::array::from_fn;

impl<T, Rhs, const R: usize> ComponentwiseOp<T, Rhs, Rhs> for Vector<T, R>
where
    T: Copy,
    Rhs: Copy,
{
    type OutputComponent = T;
    type Output = Vector<Self::OutputComponent, R>;

    fn componentwise_op(self, rhs: Rhs, op: fn(T, Rhs) -> Self::OutputComponent) -> Self::Output {
        Vector::from_array(from_fn(move |row| op(self[row], rhs)))
    }
}

impl<T, Rhs, const R: usize> AssignComponentwiseOp<T, Rhs, Rhs> for Vector<T, R>
where
    T: Copy,
    Rhs: Copy,
{
    fn assign_componentwise_op(&mut self, rhs: Rhs, op: fn(&mut T, Rhs)) {
        for row in 0..R {
            op(&mut self[row], rhs);
        }
    }
}
