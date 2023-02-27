use crate::ops::sqrt::Sqrt;
use std::array::from_fn;
use std::ops::{AddAssign, Div, DivAssign, Mul};

mod add;
mod componentwise;
mod componentwise_scalar;
mod cross;
mod divide;
mod dot;
mod index;
mod multiply;
mod scalar_add;
mod scalar_divide;
mod scalar_multiply;
mod scalar_subtract;
mod subtract;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct Vector<T: Sized + Copy, const ROWS: usize>([T; ROWS]);

pub type Vector2<T> = Vector<T, 2>;
pub type Vector3<T> = Vector<T, 3>;
pub type Vector4<T> = Vector<T, 4>;

impl<T, const ROWS: usize> Vector<T, ROWS>
where
    T: Sized + Copy,
{
    pub const fn from_array(array: [T; ROWS]) -> Self {
        Self(array)
    }

    pub const fn from_value(value: T) -> Self {
        Self::from_array([value; ROWS])
    }

    pub fn from_fn<F: FnMut(usize) -> T>(func: F) -> Self {
        Vector::from_array(from_fn(func))
    }

    pub fn magnitude_squared<Output>(self) -> Output
    where
        T: Into<Output>,
        Output: Copy + Default + Mul + AddAssign<<Output as Mul>::Output>,
    {
        let mut sum: Output = Default::default();
        for i in 0..ROWS {
            let x = self[i].into();
            sum += x * x;
        }
        sum
    }

    pub fn magnitude<Output>(self) -> <Output as Sqrt>::Output
    where
        T: Into<Output>,
        Output: Copy + Default + Mul + AddAssign<<Output as Mul>::Output> + Sqrt,
    {
        self.magnitude_squared().sqrt()
    }

    pub fn normalize<Output>(self) -> <Vector<T, ROWS> as Div<<Output as Sqrt>::Output>>::Output
    where
        T: Into<Output>,
        Output: AddAssign + Copy + Default + Div + Mul<Output = Output> + Sqrt,
        Vector<T, ROWS>: Div<<Output as Sqrt>::Output>,
    {
        self / self.magnitude()
    }

    pub fn assign_normalize(&mut self)
    where
        T: AddAssign<<T as Mul>::Output> + Default + Div + Mul + Sqrt<Output = T>,
        Vector<T, ROWS>: DivAssign<T>,
    {
        *self /= self.magnitude::<T>();
    }
}

impl<T, const ROWS: usize> Vector<T, ROWS>
where
    T: Copy + From<i8>,
{
    pub fn zero() -> Self {
        Self::from_value(T::from(0))
    }
}
