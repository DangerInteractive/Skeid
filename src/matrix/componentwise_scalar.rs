use crate::matrix::Matrix;
use crate::ops::componentwise::{AssignComponentwiseOp, ComponentwiseOp};
use std::array::from_fn;

impl<T, Rhs, const R: usize, const C: usize> ComponentwiseOp<T, Rhs, Rhs> for Matrix<T, R, C>
where
    T: Copy,
    Rhs: Copy,
{
    type OutputComponent = T;
    type Output = Matrix<Self::OutputComponent, R, C>;

    fn componentwise_op(self, rhs: Rhs, op: fn(T, Rhs) -> Self::OutputComponent) -> Self::Output {
        Matrix::from_array(from_fn(move |column| {
            from_fn(move |row| op(self[(column, row)], rhs))
        }))
    }
}

impl<T, Rhs, const R: usize, const C: usize> AssignComponentwiseOp<T, Rhs, Rhs> for Matrix<T, R, C>
where
    T: Copy,
    Rhs: Copy,
{
    fn assign_componentwise_op(&mut self, rhs: Rhs, op: fn(&mut T, Rhs)) {
        for column in 0..C {
            for row in 0..R {
                op(&mut self[(column, row)], rhs)
            }
        }
    }
}
