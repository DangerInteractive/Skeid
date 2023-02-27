use std::array::from_fn;

mod add;
mod componentwise;
mod componentwise_scalar;
mod default;
mod index;
mod matrix_multiply;
mod scalar_divide;
mod scalar_multiply;
mod subtract;
mod vector_multiply;

#[derive(Copy, Clone)]
pub struct Matrix<T: Sized + Copy, const ROWS: usize, const COLUMNS: usize>([[T; ROWS]; COLUMNS]);

pub type Matrix2<T> = Matrix<T, 2, 2>;
pub type Matrix3<T> = Matrix<T, 3, 3>;
pub type Matrix4<T> = Matrix<T, 4, 4>;

impl<T, const ROWS: usize, const COLUMNS: usize> Matrix<T, ROWS, COLUMNS>
where
    T: Copy,
{
    /// create a matrix from a 2-dimensional array (column-major)
    pub const fn from_array(array: [[T; ROWS]; COLUMNS]) -> Self {
        Self(array)
    }

    /// create a matrix where all elements are a given value
    pub const fn from_value(value: T) -> Self {
        Self::from_array([[value; ROWS]; COLUMNS])
    }

    /// create a matrix element-by-element via a callback function
    /// that takes the index of the row and column and returns the value to be initialized at that position
    pub fn from_fn<F: FnMut(usize, usize) -> T>(mut func: F) -> Self {
        Self::from_array(from_fn(|column| from_fn(|row| func(row, column))))
    }

    /// transpose the matrix
    pub fn transpose(&self) -> Matrix<T, COLUMNS, ROWS> {
        Matrix::from_fn(|row, column| self[(column, row)])
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
    /// get an instance of a zero matrix
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
    pub fn identity() -> Self {
        let mut identity_matrix = Self::zero();
        for index in 0..SIZE {
            identity_matrix[(index, index)] = T::from(1);
        }
        identity_matrix
    }
}
