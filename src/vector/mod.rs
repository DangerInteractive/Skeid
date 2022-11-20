use std::array::from_fn;
use std::ops::{Div, DivAssign};

mod add;
mod divide;
mod index;
mod multiply;
mod subtract;

#[derive(Copy, Clone)]
pub struct Vector<T: Sized, const R: usize> {
    array: [T; R],
}

pub type Vector2<T> = Vector<T, 2>;
pub type Vector3<T> = Vector<T, 3>;
pub type Vector4<T> = Vector<T, 4>;

impl<T: Sized + Copy, const S: usize> Vector<T, S> {
    pub fn magnitude_squared_f64(&self) -> f64
    where
        T: Into<f64>,
    {
        let mut sum = 0.0;
        for i in 0..S {
            let x = self[i].into();
            sum += x.powi(2);
        }
        sum
    }

    pub fn magnitude_squared_f32(&self) -> f32
    where
        T: Into<f32>,
    {
        let mut sum = 0.0;
        for i in 0..S {
            let x = self[i].into();
            sum += x.powi(2)
        }
        sum
    }

    pub fn magnitude_f64(&self) -> f64
    where
        T: Into<f64>,
    {
        self.magnitude_squared_f64().sqrt()
    }

    pub fn magnitude_f32(&self) -> f32
    where
        T: Into<f32>,
    {
        self.magnitude_squared_f32().sqrt()
    }

    pub fn normalize_f64(&self) -> Self
    where
        T: Div<f64> + Into<f64>,
        Vector<T, S>: Div<f64, Output = Vector<T, S>>,
    {
        *self / self.magnitude_f64()
    }

    pub fn normalize_f32(&self) -> Self
    where
        T: Div<f32> + Into<f32>,
        Vector<T, S>: Div<f32, Output = Vector<T, S>>,
    {
        *self / self.magnitude_f32()
    }

    pub fn into_normalized_f64(self) -> Self
    where
        T: Div<f64> + Into<f64>,
        Vector<T, S>: Div<f64, Output = Vector<T, S>>,
    {
        self / self.magnitude_f64()
    }

    pub fn into_normalized_f32(self) -> Self
    where
        T: Div<f32> + Into<f32>,
        Vector<T, S>: Div<f32, Output = Vector<T, S>>,
    {
        self / self.magnitude_f32()
    }

    pub fn assign_normalized_f64(&mut self)
    where
        T: Div<f64> + Into<f64>,
        Vector<T, S>: DivAssign<f64>,
    {
        *self /= self.magnitude_f64();
    }

    pub fn assign_normalized_f32(&mut self)
    where
        T: Div<f32> + Into<f32>,
        Vector<T, S>: DivAssign<f32>,
    {
        *self /= self.magnitude_f32();
    }

    #[inline]
    fn into_scalar_op<Out, Rhs: Copy>(self, rhs: Rhs, f: fn(T, Rhs) -> Out) -> Vector<Out, S> {
        Vector {
            array: from_fn(move |i| f(self[i], rhs)),
        }
    }

    #[inline]
    fn into_componentwise_op<Rhs: Copy, Out>(
        self,
        rhs: Vector<Rhs, S>,
        f: fn(T, Rhs) -> Out,
    ) -> Vector<Out, S> {
        Vector {
            array: from_fn(move |i| f(self[i], rhs.array[i])),
        }
    }

    #[inline]
    fn assign_from_scalar_op<Rhs: Copy>(&mut self, rhs: Rhs, f: fn(T, Rhs) -> T) {
        for i in 0..S {
            self[i] = f(self[i], rhs);
        }
    }

    #[inline]
    fn assign_from_componentwise_op<Rhs: Copy>(&mut self, rhs: Vector<Rhs, S>, f: fn(T, Rhs) -> T) {
        for i in 0..S {
            self[i] = f(self[i], rhs.array[i]);
        }
    }
}
