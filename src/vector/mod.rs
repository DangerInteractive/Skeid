//! vector math

use crate::ops::sqrt::Sqrt;
use crate::vector::iterator::{VectorIterator, VectorReferenceIterator};
use std::array::from_fn;
use std::ops::{AddAssign, Div, DivAssign, Mul};

mod add;
mod componentwise;
mod componentwise_scalar;
mod cross;
mod default;
mod divide;
mod dot;
mod from;
mod index;
pub mod iterator;
mod multiply;
mod neg;
mod scalar_add;
mod scalar_divide;
mod scalar_multiply;
mod scalar_subtract;
mod subtract;

/// a vector data structure, holding an array of numbers, used in linear algebra
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct Vector<T: Copy, const ROWS: usize>([T; ROWS]);

/// a 2-element vector
pub type Vector2<T> = Vector<T, 2>;

/// a 3-element vector
pub type Vector3<T> = Vector<T, 3>;

/// a 4-element vector
pub type Vector4<T> = Vector<T, 4>;

impl<T, const ROWS: usize> Vector<T, ROWS>
where
    T: Sized + Copy,
{
    /// create a vector from an array
    #[must_use]
    pub const fn from_array(array: [T; ROWS]) -> Self {
        Self(array)
    }

    /// create a vector where all elements are a given value
    #[must_use]
    pub const fn from_value(value: T) -> Self {
        Self::from_array([value; ROWS])
    }

    /// create a vector element-by-element via a callback function
    /// that takes the index of the row and returns the value to be initialized at that position
    #[must_use]
    pub fn from_fn<F: FnMut(usize) -> T>(func: F) -> Self {
        Vector::from_array(from_fn(func))
    }

    /// calculate the squared magnitude of the vector, avoiding a costly square root calculation
    #[must_use]
    pub fn magnitude_squared<Output>(self) -> Output
    where
        T: Into<Output>,
        Output: Copy + From<i8> + Mul + AddAssign<<Output as Mul>::Output>,
    {
        let mut sum = Output::from(0);
        for i in 0..ROWS {
            let x = self[i].into();
            sum += x * x;
        }
        sum
    }

    /// calculate the magnitude of the vector
    #[must_use]
    pub fn magnitude<Output>(self) -> <Output as Sqrt>::Output
    where
        T: Into<Output>,
        Output: Copy + From<i8> + Mul + AddAssign<<Output as Mul>::Output> + Sqrt,
    {
        self.magnitude_squared().sqrt()
    }

    /// normalize the vector
    #[must_use]
    pub fn normalize<Output>(self) -> <Vector<T, ROWS> as Div<<Output as Sqrt>::Output>>::Output
    where
        T: Into<Output>,
        Output: AddAssign + Copy + Div + From<i8> + Mul<Output = Output> + Sqrt,
        Vector<T, ROWS>: Div<<Output as Sqrt>::Output>,
    {
        self / self.magnitude()
    }

    /// normalize the vector in place (without copying/allocating)
    pub fn normalize_assign(&mut self)
    where
        T: AddAssign<<T as Mul>::Output> + Div + From<i8> + Mul + Sqrt<Output = T>,
        Vector<T, ROWS>: DivAssign<T>,
    {
        *self /= self.magnitude::<T>();
    }

    /// get a vector iterator that uses a custom index iterator
    pub const fn into_iter_for<I: Iterator<Item = usize>>(
        self,
        row_iterator: I,
    ) -> VectorIterator<T, ROWS, I> {
        VectorIterator {
            vector: self,
            row_iterator,
        }
    }

    /// get a vector reference iterator that uses a custom index iterator
    pub const fn as_iter_for<I: Iterator<Item = usize>>(
        &self,
        row_iterator: I,
    ) -> VectorReferenceIterator<T, ROWS, I> {
        VectorReferenceIterator {
            vector: self,
            row_iterator,
        }
    }
}

impl<T, const ROWS: usize> Vector<T, ROWS>
where
    T: Copy + From<i8>,
{
    /// get an instance of a zero vector (all elements contain 0)
    #[must_use]
    pub fn zero() -> Self {
        Self::from_value(T::from(0))
    }
}
