use crate::ops::componentwise::{AssignComponentwiseOp, ComponentwiseOp};
use crate::vector::Vector;
use std::ops::{Add, AddAssign};

impl<T, const ROWS: usize, Rhs> Add<Vector<Rhs, ROWS>> for Vector<T, ROWS>
where
    T: Copy + Add<Rhs>,
    Rhs: Copy,
    <T as Add<Rhs>>::Output: Copy,
{
    type Output = Vector<<T as Add<Rhs>>::Output, ROWS>;

    fn add(self, rhs: Vector<Rhs, ROWS>) -> Self::Output {
        self.componentwise_op(rhs, move |lhs_value: T, rhs_value: Rhs| {
            lhs_value + rhs_value
        })
    }
}

impl<T, const ROWS: usize, Rhs> AddAssign<Vector<Rhs, ROWS>> for Vector<T, ROWS>
where
    T: Copy + AddAssign<Rhs>,
    Rhs: Copy,
{
    fn add_assign(&mut self, rhs: Vector<Rhs, ROWS>) {
        self.assign_componentwise_op(rhs, move |lhs_value: &mut T, rhs_value: Rhs| {
            *lhs_value += rhs_value
        })
    }
}
