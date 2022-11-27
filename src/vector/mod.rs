use std::ops::{Div, DivAssign};

mod add;
mod componentwise;
mod componentwise_scalar;
mod divide;
mod index;
mod multiply;
mod scalar_add;
mod scalar_divide;
mod scalar_multiply;
mod scalar_subtract;
mod subtract;

#[derive(Copy, Clone)]
pub struct Vector<T: Sized + Copy, const R: usize>([T; R]);

pub type Vector2<T> = Vector<T, 2>;
pub type Vector3<T> = Vector<T, 3>;
pub type Vector4<T> = Vector<T, 4>;

impl<T: Sized + Copy, const R: usize> Vector<T, R> {
    pub fn from_array(array: [T; R]) -> Self {
        Vector(array)
    }

    pub fn magnitude_squared_f64(&self) -> f64
    where
        T: Into<f64>,
    {
        let mut sum = 0.0;
        for i in 0..R {
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
        for i in 0..R {
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
        Vector<T, R>: Div<f64, Output = Vector<T, R>>,
    {
        *self / self.magnitude_f64()
    }

    pub fn normalize_f32(&self) -> Self
    where
        T: Div<f32> + Into<f32>,
        Vector<T, R>: Div<f32, Output = Vector<T, R>>,
    {
        *self / self.magnitude_f32()
    }

    pub fn into_normalized_f64(self) -> Self
    where
        T: Div<f64> + Into<f64>,
        Vector<T, R>: Div<f64, Output = Vector<T, R>>,
    {
        self / self.magnitude_f64()
    }

    pub fn into_normalized_f32(self) -> Self
    where
        T: Div<f32> + Into<f32>,
        Vector<T, R>: Div<f32, Output = Vector<T, R>>,
    {
        self / self.magnitude_f32()
    }

    pub fn assign_normalized_f64(&mut self)
    where
        T: Div<f64> + Into<f64>,
        Vector<T, R>: DivAssign<f64>,
    {
        *self /= self.magnitude_f64();
    }

    pub fn assign_normalized_f32(&mut self)
    where
        T: Div<f32> + Into<f32>,
        Vector<T, R>: DivAssign<f32>,
    {
        *self /= self.magnitude_f32();
    }
}
