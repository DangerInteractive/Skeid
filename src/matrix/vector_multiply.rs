//! Operators to multiply a matrix by a vector

use crate::matrix::{Matrix, MatrixCoordinate};
use crate::vector::Vector;
use std::ops::{AddAssign, Mul};

impl<MatrixT, VectorT, const MATRIX_COLUMNS: usize, const MATRIX_ROWS: usize>
    Mul<Vector<VectorT, MATRIX_COLUMNS>> for Matrix<MatrixT, MATRIX_ROWS, MATRIX_COLUMNS>
where
    MatrixT: Copy + Mul<VectorT>,
    VectorT: Copy,
    <MatrixT as Mul<VectorT>>::Output: AddAssign + Copy + Default,
{
    type Output = Vector<<MatrixT as Mul<VectorT>>::Output, MATRIX_COLUMNS>;

    fn mul(self, rhs: Vector<VectorT, MATRIX_COLUMNS>) -> Self::Output {
        Vector::from_fn(|row| {
            let mut sum = Default::default();
            for k in 0..MATRIX_COLUMNS {
                sum += self[MatrixCoordinate::new(row, k)] * rhs[k];
            }
            sum
        })
    }
}
