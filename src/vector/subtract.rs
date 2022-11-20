use crate::vector::Vector;
use std::ops::{Sub, SubAssign};

impl<T, const S: usize, By> Sub<Vector<By, S>> for Vector<T, S>
where
    T: Sized + Copy + Sub<By>,
    By: Sized + Copy,
{
    type Output = Vector<<T as Sub<By>>::Output, S>;

    fn sub(self, rhs: Vector<By, S>) -> Self::Output {
        self.into_componentwise_op::<By, <T as Sub<By>>::Output>(
            rhs,
            move |lhs_value, rhs_value| lhs_value - rhs_value,
        )
    }
}

impl<T, const S: usize, By> SubAssign<Vector<By, S>> for Vector<T, S>
where
    T: Sized + Copy + Sub<By, Output = T>,
    By: Sized + Copy,
{
    fn sub_assign(&mut self, rhs: Vector<By, S>) {
        self.assign_from_componentwise_op::<By>(rhs, move |lhs_value, rhs_value| {
            lhs_value - rhs_value
        })
    }
}
