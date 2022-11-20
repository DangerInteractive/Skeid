use std::ops::{Mul, MulAssign};
use crate::marker::Scalar;
use crate::vector::Vector;

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
