use std::array::from_fn;
use std::ops::Div;

mod add;
mod divide;
mod multiply;
mod subtract;

#[derive(Copy, Clone)]
pub struct Vector<T: Sized, const S: usize> {
    array: [T; S],
}

pub type Vector2<T> = Vector<T, 2>;
pub type Vector3<T> = Vector<T, 3>;
pub type Vector4<T> = Vector<T, 4>;

impl<T: Sized + Copy, const S: usize> Vector<T, S> {
    pub fn magnitude_f64(&self) -> f64
    where
        T: Into<f64>,
    {
        let mut sum = 0.0;
        for i in 0..S {
            let x = self.array[i].into();
            sum += x.powi(2);
        }
        sum.sqrt()
    }

    pub fn magnitude_f32(&self) -> f32
    where
        T: Into<f32>,
    {
        let mut sum = 0.0;
        for i in 0..S {
            let x = self.array[i].into();
            sum += x.powi(2)
        }
        sum.sqrt()
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

    #[inline]
    fn into_scalar_op<Out, R: Copy>(self, rhs: R, f: fn(T, R) -> Out) -> Vector<Out, S> {
        Vector {
            array: from_fn(move |i| f(self.array[i], rhs)),
        }
    }

    #[inline]
    fn into_vector_op<R: Copy, Out>(self, rhs: Vector<R, S>, f: fn(T, R) -> Out) -> Vector<Out, S> {
        Vector {
            array: from_fn(move |i| f(self.array[i], rhs.array[i])),
        }
    }

    #[inline]
    fn assign_from_scalar_op<R: Copy>(&mut self, rhs: R, f: fn(T, R) -> T) {
        for i in 0..S {
            self.array[i] = f(self.array[i], rhs);
        }
    }

    #[inline]
    fn assign_from_vector_op<R: Copy>(&mut self, rhs: Vector<R, S>, f: fn(T, R) -> T) {
        for i in 0..S {
            self.array[i] = f(self.array[i], rhs.array[i]);
        }
    }
}
