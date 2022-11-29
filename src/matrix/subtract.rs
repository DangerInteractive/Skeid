use crate::matrix::Matrix;
use crate::ops::componentwise::{AssignComponentwiseOp, ComponentwiseOp};
use std::ops::{Sub, SubAssign};

impl<T, const ROWS: usize, const COLUMNS: usize, Rhs> Sub<Matrix<Rhs, ROWS, COLUMNS>>
    for Matrix<T, ROWS, COLUMNS>
where
    T: Copy + Sub<Rhs>,
    Rhs: Copy,
    <T as Sub<Rhs>>::Output: Copy,
{
    type Output = Matrix<<T as Sub<Rhs>>::Output, ROWS, COLUMNS>;

    fn sub(self, rhs: Matrix<Rhs, ROWS, COLUMNS>) -> Self::Output {
        self.componentwise_op(rhs, move |lhs_value: T, rhs_value: Rhs| {
            lhs_value - rhs_value
        })
    }
}

impl<T, const ROWS: usize, const COLUMNS: usize, Rhs> SubAssign<Matrix<Rhs, ROWS, COLUMNS>>
    for Matrix<T, ROWS, COLUMNS>
where
    T: Copy + SubAssign<Rhs>,
    Rhs: Copy,
{
    fn sub_assign(&mut self, rhs: Matrix<Rhs, ROWS, COLUMNS>) {
        self.assign_componentwise_op(rhs, move |lhs_value: &mut T, rhs_value: Rhs| {
            *lhs_value -= rhs_value
        })
    }
}
