use crate::vector::Vector;

pub struct Matrix<T, const R: usize, const C: usize> {
    array: [Vector<T, C>; R],
}

pub type Matrix2<T> = Matrix<T, 2, 2>;
pub type Matrix3<T> = Matrix<T, 3, 3>;
pub type Matrix4<T> = Matrix<T, 4, 4>;
