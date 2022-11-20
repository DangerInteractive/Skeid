use crate::vector::Vector;
use std::array::from_fn;

pub mod add;
pub mod subtract;
pub mod index;

#[derive(Copy, Clone)]
pub struct Matrix<T: Sized + Copy, const R: usize, const C: usize> {
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

    #[inline]
    fn into_scalar_op<Rhs: Copy, Out: Copy>(
        self,
        rhs: Rhs,
        f: fn(Vector<T, R>, Rhs) -> Vector<Out, R>,
    ) -> Matrix<Out, R, C> {
        Matrix::from_array(from_fn(move |i| f(self[i], rhs)))
    }

    #[inline]
    fn into_componentwise_op<Rhs: Copy, Out: Copy>(
        self,
        rhs: Matrix<Rhs, R, C>,
        f: fn(Vector<T, R>, Vector<Rhs, R>) -> Vector<Out, R>,
    ) -> Matrix<Out, R, C> {
        Matrix::from_array(from_fn(move |i| f(self[i], rhs[i])))
    }

    #[inline]
    fn assign_from_scalar_op<Rhs: Copy>(
        &mut self,
        rhs: Rhs,
        f: fn(Vector<T, R>, Rhs) -> Vector<T, R>,
    ) {
        for i in 0..R {
            self[i] = f(self[i], rhs);
        }
    }

    #[inline]
    fn assign_from_componentwise_op<Rhs: Copy>(
        &mut self,
        rhs: Matrix<Rhs, R, C>,
        f: fn(Vector<T, R>, Vector<Rhs, R>) -> Vector<T, R>,
    ) {
        for i in 0..R {
            self[i] = f(self[i], rhs[i]);
        }
    }
}
