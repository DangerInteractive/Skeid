use crate::marker::Scalar;
use crate::ops::componentwise::{AssignComponentwiseOp, ComponentwiseOp};
use crate::vector::Vector;
use std::ops::{Sub, SubAssign};

impl<T, const ROWS: usize, Rhs> Sub<Rhs> for Vector<T, ROWS>
where
    T: Copy + Sub<Rhs>,
    Rhs: Scalar + Copy,
    <T as Sub<Rhs>>::Output: Copy,
{
    type Output = Vector<<T as Sub<Rhs>>::Output, ROWS>;

    fn sub(self, rhs: Rhs) -> Self::Output {
        self.componentwise_op(rhs, move |lhs_value, rhs_value| lhs_value - rhs_value)
    }
}

impl<T, const ROWS: usize, Rhs> SubAssign<Rhs> for Vector<T, ROWS>
where
    T: Copy + SubAssign<Rhs>,
    Rhs: Scalar + Copy,
{
    fn sub_assign(&mut self, rhs: Rhs) {
        self.assign_componentwise_op(rhs, move |lhs_value, rhs_value| *lhs_value -= rhs_value)
    }
}
