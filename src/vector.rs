use crate::marker::Scalar;
use std::array::from_fn;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

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
            sum += x * x;
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
            sum += x * x;
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

impl<T, const S: usize, By> Add<Vector<By, S>> for Vector<T, S>
where
    T: Sized + Copy + Add<By>,
    By: Sized + Copy,
{
    type Output = Vector<<T as Add<By>>::Output, S>;

    fn add(self, rhs: Vector<By, S>) -> Self::Output {
        self.into_vector_op::<By, <T as Add<By>>::Output>(rhs, move |lhs_value, rhs_value| {
            lhs_value + rhs_value
        })
    }
}

impl<T, const S: usize, By> Add<By> for Vector<T, S>
where
    T: Sized + Copy + Add<By>,
    By: Scalar + Sized + Copy,
{
    type Output = Vector<<T as Add<By>>::Output, S>;

    fn add(self, rhs: By) -> Self::Output {
        self.into_scalar_op(rhs, move |lhs_value, rhs_value| lhs_value + rhs_value)
    }
}

impl<T, const S: usize, By> AddAssign<Vector<By, S>> for Vector<T, S>
where
    T: Sized + Copy + Add<By, Output = T>,
    By: Sized + Copy,
{
    fn add_assign(&mut self, rhs: Vector<By, S>) {
        self.assign_from_vector_op::<By>(rhs, move |lhs_value, rhs_value| lhs_value + rhs_value)
    }
}

impl<T, const S: usize, By> AddAssign<By> for Vector<T, S>
where
    T: Sized + Copy + Add<By, Output = T>,
    By: Scalar + Sized + Copy,
{
    fn add_assign(&mut self, rhs: By) {
        self.assign_from_scalar_op::<By>(rhs, move |lhs_value, rhs_value| lhs_value + rhs_value)
    }
}

impl<T, const S: usize, By> Sub<Vector<By, S>> for Vector<T, S>
where
    T: Sized + Copy + Sub<By>,
    By: Sized + Copy,
{
    type Output = Vector<<T as Sub<By>>::Output, S>;

    fn sub(self, rhs: Vector<By, S>) -> Self::Output {
        self.into_vector_op::<By, <T as Sub<By>>::Output>(rhs, move |lhs_value, rhs_value| {
            lhs_value - rhs_value
        })
    }
}

impl<T, const S: usize, By> Sub<By> for Vector<T, S>
where
    T: Sized + Copy + Sub<By>,
    By: Scalar + Sized + Copy,
{
    type Output = Vector<<T as Sub<By>>::Output, S>;

    fn sub(self, rhs: By) -> Self::Output {
        self.into_scalar_op(rhs, move |lhs_value, rhs_value| lhs_value - rhs_value)
    }
}

impl<T, const S: usize, By> SubAssign<Vector<By, S>> for Vector<T, S>
where
    T: Sized + Copy + Sub<By, Output = T>,
    By: Sized + Copy,
{
    fn sub_assign(&mut self, rhs: Vector<By, S>) {
        self.assign_from_vector_op::<By>(rhs, move |lhs_value, rhs_value| lhs_value - rhs_value)
    }
}

impl<T, const S: usize, By> SubAssign<By> for Vector<T, S>
where
    T: Sized + Copy + Sub<By, Output = T>,
    By: Scalar + Sized + Copy,
{
    fn sub_assign(&mut self, rhs: By) {
        self.assign_from_scalar_op::<By>(rhs, move |lhs_value, rhs_value| lhs_value - rhs_value)
    }
}

impl<T, const S: usize, By> Mul<Vector<By, S>> for Vector<T, S>
where
    T: Sized + Copy + Mul<By>,
    By: Sized + Copy,
{
    type Output = Vector<<T as Mul<By>>::Output, S>;

    fn mul(self, rhs: Vector<By, S>) -> Self::Output {
        self.into_vector_op(rhs, move |lhs_value, rhs_value| lhs_value * rhs_value)
    }
}

impl<T, const S: usize, By> Mul<By> for Vector<T, S>
where
    T: Sized + Copy + Mul<By>,
    By: Scalar + Sized + Copy,
{
    type Output = Vector<<T as Mul<By>>::Output, S>;

    fn mul(self, rhs: By) -> Self::Output {
        self.into_scalar_op(rhs, move |lhs_value, rhs_value| lhs_value * rhs_value)
    }
}

impl<T, const S: usize, By> MulAssign<Vector<By, S>> for Vector<T, S>
where
    T: Sized + Copy + Mul<By, Output = T>,
    By: Sized + Copy,
{
    fn mul_assign(&mut self, rhs: Vector<By, S>) {
        self.assign_from_vector_op(rhs, move |lhs_value, rhs_value| lhs_value * rhs_value)
    }
}

impl<T, const S: usize, By> MulAssign<By> for Vector<T, S>
where
    T: Sized + Copy + Mul<By, Output = T>,
    By: Scalar + Sized + Copy,
{
    fn mul_assign(&mut self, rhs: By) {
        self.assign_from_scalar_op::<By>(rhs, move |lhs_value, rhs_value| lhs_value * rhs_value)
    }
}

impl<T, const S: usize, By> Div<Vector<By, S>> for Vector<T, S>
where
    T: Sized + Copy + Div<By>,
    By: Sized + Copy,
{
    type Output = Vector<<T as Div<By>>::Output, S>;

    fn div(self, rhs: Vector<By, S>) -> Self::Output {
        self.into_vector_op(rhs, move |lhs_value, rhs_value| lhs_value / rhs_value)
    }
}

impl<T, const S: usize, By> Div<By> for Vector<T, S>
where
    T: Sized + Copy + Div<By>,
    By: Scalar + Sized + Copy,
{
    type Output = Vector<<T as Div<By>>::Output, S>;

    fn div(self, rhs: By) -> Self::Output {
        self.into_scalar_op(rhs, move |lhs_value, rhs_value| lhs_value / rhs_value)
    }
}

impl<T, const S: usize, By> DivAssign<Vector<By, S>> for Vector<T, S>
where
    T: Sized + Copy + Div<By, Output = T>,
    By: Sized + Copy,
{
    fn div_assign(&mut self, rhs: Vector<By, S>) {
        self.assign_from_vector_op(rhs, move |lhs_value, rhs_value| lhs_value / rhs_value)
    }
}

impl<T, const S: usize, By> DivAssign<By> for Vector<T, S>
where
    T: Sized + Copy + Div<By, Output = T>,
    By: Scalar + Sized + Copy,
{
    fn div_assign(&mut self, rhs: By) {
        self.assign_from_scalar_op::<By>(rhs, move |lhs_value, rhs_value| lhs_value / rhs_value)
    }
}
