use std::array::from_fn;

mod add;
mod componentwise;
mod componentwise_scalar;
mod index;
mod scalar_divide;
mod scalar_multiply;
mod subtract;

#[derive(Copy, Clone)]
pub struct Matrix<T: Sized + Copy, const R: usize, const C: usize>([[T; R]; C]);

pub type Matrix2<T> = Matrix<T, 2, 2>;
pub type Matrix3<T> = Matrix<T, 3, 3>;
pub type Matrix4<T> = Matrix<T, 4, 4>;

impl<T, const R: usize, const C: usize> Matrix<T, R, C>
where
    T: Copy,
{
    pub fn from_array(array: [[T; R]; C]) -> Self {
        Matrix(array)
    }

    pub fn transpose(&self) -> Matrix<T, C, R> {
        Matrix::from_array(from_fn(move |i| from_fn(move |j| self[(j, i)])))
    }
}
