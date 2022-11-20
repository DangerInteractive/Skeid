//! A Linear Algebra Library for Game Graphics and Physics

use std::array::from_fn;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

pub struct Vector<T: Sized, const S: usize> {
    array: [T; S],
}

impl<T: Sized + Copy, const S: usize> Vector<T, S> {
    #[inline]
    fn into_op_across<Out>(self, rhs: Self, f: fn(T, T) -> Out) -> Vector<Out, S> {
        Vector {
            array: from_fn(move |i| f(self.array[i], rhs.array[i])),
        }
    }

    #[inline]
    fn assign_op_across(&mut self, rhs: Self, f: fn(T, T) -> T) {
        for i in 0..(S - 1) {
            self.array[i] = f(self.array[i], rhs.array[i]);
        }
    }
}

impl<T: Sized + Copy, const S: usize> Add for Vector<T, S>
where
    T: Add<T>,
{
    type Output = Vector<<T as Add>::Output, S>;

    fn add(self, rhs: Self) -> Self::Output {
        self.into_op_across::<<T as Add>::Output>(rhs, move |lhs_value, rhs_value| {
            lhs_value + rhs_value
        })
    }
}

impl<T: Sized + Copy, const S: usize> AddAssign for Vector<T, S>
where
    T: Add<T, Output = T>,
{
    fn add_assign(&mut self, rhs: Self) {
        self.assign_op_across(rhs, move |lhs_value, rhs_value| lhs_value + rhs_value)
    }
}

impl<T: Sized + Copy, const S: usize> Sub for Vector<T, S>
where
    T: Sub<T>,
{
    type Output = Vector<<T as Sub>::Output, S>;

    fn sub(self, rhs: Self) -> Self::Output {
        self.into_op_across::<<T as Sub>::Output>(rhs, move |lhs_value, rhs_value| {
            lhs_value - rhs_value
        })
    }
}

impl<T: Sized + Copy, const S: usize> SubAssign for Vector<T, S>
where
    T: Sub<T, Output = T>,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.assign_op_across(rhs, move |lhs_value, rhs_value| lhs_value - rhs_value)
    }
}

impl<T: Sized + Copy, const S: usize> Mul for Vector<T, S>
where
    T: Mul<T>,
{
    type Output = Vector<<T as Mul>::Output, S>;

    fn mul(self, rhs: Self) -> Self::Output {
        self.into_op_across(rhs, move |lhs_value, rhs_value| lhs_value * rhs_value)
    }
}

impl<T: Sized + Copy, const S: usize> MulAssign for Vector<T, S>
where
    T: Mul<T, Output = T>,
{
    fn mul_assign(&mut self, rhs: Self) {
        self.assign_op_across(rhs, move |lhs_value, rhs_value| lhs_value * rhs_value)
    }
}

impl<T: Sized + Copy, const S: usize> Div for Vector<T, S>
where
    T: Div<T>,
{
    type Output = Vector<<T as Div>::Output, S>;

    fn div(self, rhs: Self) -> Self::Output {
        self.into_op_across(rhs, move |lhs_value, rhs_value| lhs_value / rhs_value)
    }
}

impl<T: Sized + Copy, const S: usize> DivAssign for Vector<T, S>
where
    T: Div<T, Output = T>,
{
    fn div_assign(&mut self, rhs: Self) {
        self.assign_op_across(rhs, move |lhs_value, rhs_value| lhs_value / rhs_value)
    }
}
