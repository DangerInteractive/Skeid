use crate::matrix::Matrix;
use crate::ops::componentwise::{AssignComponentwiseOp, ComponentwiseOp};
use std::ops::{Add, AddAssign};

impl<T, const ROWS: usize, const COLUMNS: usize, Rhs> Add<Matrix<Rhs, ROWS, COLUMNS>>
    for Matrix<T, ROWS, COLUMNS>
where
    T: Copy + Add<Rhs>,
    Rhs: Copy,
    <T as Add<Rhs>>::Output: Copy,
{
    type Output = Matrix<<T as Add<Rhs>>::Output, ROWS, COLUMNS>;

    fn add(self, rhs: Matrix<Rhs, ROWS, COLUMNS>) -> Self::Output {
        self.componentwise_op(rhs, move |lhs_value: T, rhs_value: Rhs| {
            lhs_value + rhs_value
        })
    }
}

impl<T, const ROWS: usize, const COLUMNS: usize, Rhs> AddAssign<Matrix<Rhs, ROWS, COLUMNS>>
    for Matrix<T, ROWS, COLUMNS>
where
    T: Copy + AddAssign<Rhs>,
    Rhs: Copy,
{
    fn add_assign(&mut self, rhs: Matrix<Rhs, ROWS, COLUMNS>) {
        self.assign_componentwise_op(rhs, move |lhs_value: &mut T, rhs_value: Rhs| {
            *lhs_value += rhs_value
        })
    }
}
