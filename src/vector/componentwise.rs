use crate::ops::componentwise::{AssignComponentwiseOp, ComponentwiseOp};
use crate::vector::Vector;
use std::array::from_fn;

/// Operate on two vectors of the same dimensions
impl<T, RC, const R: usize> ComponentwiseOp<T, Vector<RC, R>, RC> for Vector<T, R>
where
    T: Copy,
    RC: Copy,
{
    type OutputComponent = T;
    type Output = Vector<Self::OutputComponent, R>;

    fn componentwise_op(
        self,
        rhs: Vector<RC, R>,
        op: fn(T, RC) -> Self::OutputComponent,
    ) -> Self::Output {
        Vector::from_array(from_fn(move |row| op(self[row], rhs[row])))
    }
}

/// Operate on two vectors of the same dimensions, in place on left-hand-side vector
impl<T, RC, const R: usize> AssignComponentwiseOp<T, Vector<RC, R>, RC> for Vector<T, R>
where
    T: Copy,
    RC: Copy,
{
    fn assign_componentwise_op(&mut self, rhs: Vector<RC, R>, op: fn(&mut T, RC)) {
        for row in 0..R {
            op(&mut self[row], rhs[row]);
        }
    }
}
