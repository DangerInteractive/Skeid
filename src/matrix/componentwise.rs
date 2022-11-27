use crate::matrix::Matrix;
use crate::ops::componentwise::{AssignComponentwiseOp, ComponentwiseOp};
use std::array::from_fn;

/// Operate on two matrices of the same dimensions
impl<T, RC, const R: usize, const C: usize> ComponentwiseOp<T, Matrix<RC, R, C>, RC>
    for Matrix<T, R, C>
where
    T: Copy,
    RC: Copy,
{
    type OutputComponent = T;
    type Output = Matrix<Self::OutputComponent, R, C>;

    fn componentwise_op(
        self,
        rhs: Matrix<RC, R, C>,
        op: fn(T, RC) -> Self::OutputComponent,
    ) -> Self::Output {
        Matrix::from_array(from_fn(move |column| {
            from_fn(move |row| op(self[(column, row)], rhs[(column, row)]))
        }))
    }
}

/// Operate on two matrices of the same dimensions, in place on left-hand-side matrix
impl<T, RC, const R: usize, const C: usize> AssignComponentwiseOp<T, Matrix<RC, R, C>, RC>
    for Matrix<T, R, C>
where
    T: Copy,
    RC: Copy,
{
    fn assign_componentwise_op(&mut self, rhs: Matrix<RC, R, C>, op: fn(&mut T, RC)) {
        for column in 0..C {
            for row in 0..R {
                op(&mut self[(column, row)], rhs[(column, row)]);
            }
        }
    }
}
