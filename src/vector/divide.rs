use crate::ops::componentwise::{AssignComponentwise, Componentwise};
use crate::vector::Vector;
use std::ops::{Div, DivAssign};

impl<T, const ROWS: usize, Rhs> Div<Vector<Rhs, ROWS>> for Vector<T, ROWS>
where
    T: Copy + Div<Rhs>,
    Rhs: Copy,
    <T as Div<Rhs>>::Output: Copy,
{
    type Output = Vector<<T as Div<Rhs>>::Output, ROWS>;

    fn div(self, rhs: Vector<Rhs, ROWS>) -> Self::Output {
        self.componentwise(rhs, |lhs_value: T, rhs_value: Rhs| lhs_value / rhs_value)
    }
}

impl<T, const ROWS: usize, Rhs> DivAssign<Vector<Rhs, ROWS>> for Vector<T, ROWS>
where
    T: Copy + DivAssign<Rhs>,
    Rhs: Copy,
{
    fn div_assign(&mut self, rhs: Vector<Rhs, ROWS>) {
        self.assign_componentwise(rhs, |lhs_value: &mut T, rhs_value: Rhs| {
            *lhs_value /= rhs_value
        })
    }
}
