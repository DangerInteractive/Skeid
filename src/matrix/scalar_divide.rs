use crate::matrix::Matrix;
use crate::ops::componentwise::{AssignComponentwise, Componentwise};
use std::ops::{Div, DivAssign};

impl<T, const ROWS: usize, const COLUMNS: usize, Rhs> Div<Rhs> for Matrix<T, ROWS, COLUMNS>
where
    T: Copy + Div<Rhs>,
    Rhs: Copy,
    <T as Div<Rhs>>::Output: Copy,
{
    type Output = Matrix<<T as Div<Rhs>>::Output, ROWS, COLUMNS>;

    fn div(self, rhs: Rhs) -> Self::Output {
        self.componentwise(rhs, |lhs_value, rhs_value| lhs_value / rhs_value)
    }
}

impl<T, const ROWS: usize, const COLUMNS: usize, Rhs> DivAssign<Rhs> for Matrix<T, ROWS, COLUMNS>
where
    T: Copy + DivAssign<Rhs>,
    Rhs: Copy,
{
    fn div_assign(&mut self, rhs: Rhs) {
        self.assign_componentwise(rhs, |lhs_value, rhs_value| *lhs_value /= rhs_value)
    }
}
