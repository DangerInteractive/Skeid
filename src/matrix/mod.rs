use crate::vector::Vector;
use std::array::from_fn;

pub mod index;

pub struct Matrix<T: Copy, const R: usize, const C: usize> {
    array: [Vector<T, R>; C],
}

pub type Matrix2<T> = Matrix<T, 2, 2>;
pub type Matrix3<T> = Matrix<T, 3, 3>;
pub type Matrix4<T> = Matrix<T, 4, 4>;

impl<T, const R: usize, const C: usize> Matrix<T, R, C>
where
    T: Copy,
{
    pub fn from_array(array: [Vector<T, R>; C]) -> Self {
        Matrix { array }
    }

    pub fn transpose(&self) -> Matrix<T, C, R> {
        Matrix::from_array(from_fn(move |i| {
            Vector::from_array(from_fn(move |j| self[j][i]))
        }))
    }
}
