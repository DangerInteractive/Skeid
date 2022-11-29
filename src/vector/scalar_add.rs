use crate::marker::Scalar;
use crate::ops::componentwise::{AssignComponentwiseOp, ComponentwiseOp};
use crate::vector::Vector;
use std::ops::{Add, AddAssign};

impl<T, const ROWS: usize, Rhs> Add<Rhs> for Vector<T, ROWS>
where
    T: Copy + Add<Rhs>,
    Rhs: Scalar + Copy,
    <T as Add<Rhs>>::Output: Copy,
{
    type Output = Vector<<T as Add<Rhs>>::Output, ROWS>;

    fn add(self, rhs: Rhs) -> Self::Output {
        self.componentwise_op(rhs, move |lhs_value, rhs_value| lhs_value + rhs_value)
    }
}

impl<T, const ROWS: usize, Rhs> AddAssign<Rhs> for Vector<T, ROWS>
where
    T: Copy + AddAssign<Rhs>,
    Rhs: Scalar + Copy,
{
    fn add_assign(&mut self, rhs: Rhs) {
        self.assign_componentwise_op(rhs, move |lhs_value, rhs_value| *lhs_value += rhs_value)
    }
}
