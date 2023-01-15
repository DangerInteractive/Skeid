use crate::marker::Scalar;
use crate::ops::componentwise::{AssignComponentwise, Componentwise};
use crate::vector::Vector;
use std::ops::{Div, DivAssign};

impl<T, const ROWS: usize, Rhs> Div<Rhs> for Vector<T, ROWS>
where
    T: Copy + Div<Rhs>,
    Rhs: Scalar + Copy,
    <T as Div<Rhs>>::Output: Copy,
{
    type Output = Vector<<T as Div<Rhs>>::Output, ROWS>;

    fn div(self, rhs: Rhs) -> Self::Output {
        self.componentwise(rhs, |lhs_value, rhs_value| lhs_value / rhs_value)
    }
}

impl<T, const ROWS: usize, Rhs> DivAssign<Rhs> for Vector<T, ROWS>
where
    T: Copy + DivAssign<Rhs>,
    Rhs: Scalar + Copy,
{
    fn div_assign(&mut self, rhs: Rhs) {
        self.assign_componentwise(rhs, |lhs_value, rhs_value| *lhs_value /= rhs_value)
    }
}
