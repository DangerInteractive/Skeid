use crate::marker::Scalar;
use crate::ops::componentwise::{AssignComponentwise, Componentwise};
use crate::vector::Vector;
use std::ops::{Mul, MulAssign};

impl<T, const ROWS: usize, Rhs> Mul<Rhs> for Vector<T, ROWS>
where
    T: Copy + Mul<Rhs>,
    Rhs: Scalar + Copy,
    <T as Mul<Rhs>>::Output: Copy,
{
    type Output = Vector<<T as Mul<Rhs>>::Output, ROWS>;

    fn mul(self, rhs: Rhs) -> Self::Output {
        self.componentwise(rhs, |lhs_value, rhs_value| lhs_value * rhs_value)
    }
}

impl<T, const ROWS: usize, Rhs> MulAssign<Rhs> for Vector<T, ROWS>
where
    T: Copy + MulAssign<Rhs>,
    Rhs: Scalar + Copy,
{
    fn mul_assign(&mut self, rhs: Rhs) {
        self.assign_componentwise(rhs, |lhs_value, rhs_value| *lhs_value *= rhs_value)
    }
}
