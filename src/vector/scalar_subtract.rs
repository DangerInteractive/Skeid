use crate::marker::Scalar;
use crate::ops::componentwise::{AssignComponentwise, Componentwise};
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
        self.componentwise(rhs, |lhs_value, rhs_value| lhs_value - rhs_value)
    }
}

impl<T, const ROWS: usize, Rhs> SubAssign<Rhs> for Vector<T, ROWS>
where
    T: Copy + SubAssign<Rhs>,
    Rhs: Scalar + Copy,
{
    fn sub_assign(&mut self, rhs: Rhs) {
        self.assign_componentwise(rhs, |lhs_value, rhs_value| *lhs_value -= rhs_value)
    }
}
