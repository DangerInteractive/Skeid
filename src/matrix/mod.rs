//! matrix math

use crate::matrix::iterator::{MatrixIterator, MatrixReferenceIterator};
use std::array::from_fn;

mod add;
mod componentwise;
mod componentwise_scalar;
mod default;
mod from;
mod index;
pub mod iterator;
mod matrix_multiply;
mod neg;
mod scalar_divide;
mod scalar_multiply;
mod subtract;
mod vector_multiply;

/// a matrix data structure holding a rectangular array or table of numbers, used in linear algebra
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct Matrix<T: Sized + Copy, const ROWS: usize, const COLUMNS: usize>([[T; ROWS]; COLUMNS]);

/// a 2x2 matrix
pub type Matrix2<T> = Matrix<T, 2, 2>;

/// a 3x3 matrix
pub type Matrix3<T> = Matrix<T, 3, 3>;

/// a 4x4 matrix
pub type Matrix4<T> = Matrix<T, 4, 4>;

impl<T, const ROWS: usize, const COLUMNS: usize> Matrix<T, ROWS, COLUMNS>
where
    T: Copy,
{
    /// create a matrix from a 2-dimensional array (column-major)
    #[must_use]
    pub const fn from_array(array: [[T; ROWS]; COLUMNS]) -> Self {
        Self(array)
    }

    /// create a matrix where all elements are a given value
    #[must_use]
    pub const fn from_value(value: T) -> Self {
        Self::from_array([[value; ROWS]; COLUMNS])
    }

    /// create a matrix element-by-element via a callback function
    /// that takes the index of the row and column and returns the value to be initialized at that position
    #[must_use]
    pub fn from_fn<F: FnMut(MatrixCoordinate) -> T>(mut func: F) -> Self {
        Self::from_array(from_fn(|column| {
            from_fn(|row| func(MatrixCoordinate::new(column, row)))
        }))
    }

    /// transpose the matrix
    #[must_use]
    pub fn transpose(&self) -> Matrix<T, COLUMNS, ROWS> {
        Matrix::from_fn(|coord| self[MatrixCoordinate::new(coord.column, coord.row)])
    }

    /// get a matrix iterator that uses a custom index iterator
    #[must_use]
    pub const fn into_iter_for<I: Iterator<Item = MatrixCoordinate>>(
        self,
        coordinate_iterator: I,
    ) -> MatrixIterator<T, ROWS, COLUMNS, I> {
        MatrixIterator {
            matrix: self,
            coordinate_iterator,
        }
    }

    /// get a matrix reference iterator that uses a custom index iterator
    #[must_use]
    pub const fn as_iter_for<I: Iterator<Item = MatrixCoordinate>>(
        &self,
        coordinate_iterator: I,
    ) -> MatrixReferenceIterator<T, ROWS, COLUMNS, I> {
        MatrixReferenceIterator {
            matrix: self,
            coordinate_iterator,
        }
    }
}

/// Matrices where `T` can be converted from an `i8`
///
/// `i8` was chosen as the `From` type because we want to support any size integer or float easily,
/// and because the functions in this impl should only need to create values of 0, 1, or -1.
impl<T, const ROWS: usize, const COLUMNS: usize> Matrix<T, ROWS, COLUMNS>
where
    T: Copy + From<i8>,
{
    /// get an instance of a zero matrix (all elements contain 0)
    #[must_use]
    pub fn zero() -> Self {
        Self::from_value(T::from(0))
    }
}

/// Square Matrices where `T` can be converted from an `i8`
///
/// `i8` was chosen as the `From` type because we want to support any size integer or float easily,
/// and because the functions in this impl should only need to create values of 0, 1, or -1.
impl<T, const SIZE: usize> Matrix<T, SIZE, SIZE>
where
    T: Copy + From<i8>,
{
    /// get an instance of an identity matrix
    #[must_use]
    pub fn identity() -> Self {
        let mut identity_matrix = Self::zero();
        for index in 0..SIZE {
            identity_matrix[MatrixCoordinate::new(index, index)] = T::from(1);
        }
        identity_matrix
    }
}

/// A coordinate referring to an element of a `Matrix`
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct MatrixCoordinate {
    /// 0-indexed column index for a matrix
    pub column: usize,
    /// 0-indexed row index for a matrix
    pub row: usize,
}

impl MatrixCoordinate {
    /// create a new coordinate
    #[must_use]
    pub const fn new(column: usize, row: usize) -> Self {
        Self { column, row }
    }
}
