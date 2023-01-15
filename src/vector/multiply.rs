use crate::ops::componentwise::{AssignComponentwise, Componentwise};
use crate::vector::Vector;
use std::ops::{Mul, MulAssign};

impl<T, const ROWS: usize, Rhs> Mul<Vector<Rhs, ROWS>> for Vector<T, ROWS>
where
    T: Copy + Mul<Rhs>,
    Rhs: Copy,
    <T as Mul<Rhs>>::Output: Copy,
{
    type Output = Vector<<T as Mul<Rhs>>::Output, ROWS>;

    fn mul(self, rhs: Vector<Rhs, ROWS>) -> Self::Output {
        self.componentwise(rhs, |lhs_value: T, rhs_value: Rhs| lhs_value * rhs_value)
    }
}

impl<T, const ROWS: usize, Rhs> MulAssign<Vector<Rhs, ROWS>> for Vector<T, ROWS>
where
    T: Copy + MulAssign<Rhs>,
    Rhs: Copy,
{
    fn mul_assign(&mut self, rhs: Vector<Rhs, ROWS>) {
        self.assign_componentwise(rhs, |lhs_value: &mut T, rhs_value: Rhs| {
            *lhs_value *= rhs_value
        })
    }
}
