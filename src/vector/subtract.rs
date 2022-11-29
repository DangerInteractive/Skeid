use crate::ops::componentwise::{AssignComponentwiseOp, ComponentwiseOp};
use crate::vector::Vector;
use std::ops::{Sub, SubAssign};

impl<T, const ROWS: usize, Rhs> Sub<Vector<Rhs, ROWS>> for Vector<T, ROWS>
where
    T: Copy + Sub<Rhs>,
    Rhs: Copy,
    <T as Sub<Rhs>>::Output: Copy,
{
    type Output = Vector<<T as Sub<Rhs>>::Output, ROWS>;

    fn sub(self, rhs: Vector<Rhs, ROWS>) -> Self::Output {
        self.componentwise_op(rhs, move |lhs_value: T, rhs_value: Rhs| {
            lhs_value - rhs_value
        })
    }
}

impl<T, const ROWS: usize, Rhs> SubAssign<Vector<Rhs, ROWS>> for Vector<T, ROWS>
where
    T: Copy + SubAssign<Rhs>,
    Rhs: Copy,
{
    fn sub_assign(&mut self, rhs: Vector<Rhs, ROWS>) {
        self.assign_componentwise_op(rhs, move |lhs_value: &mut T, rhs_value: Rhs| {
            *lhs_value -= rhs_value
        })
    }
}
