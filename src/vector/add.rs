use crate::vector::Vector;
use std::ops::{Add, AddAssign};

impl<T, const S: usize, By> Add<Vector<By, S>> for Vector<T, S>
where
    T: Sized + Copy + Add<By>,
    By: Sized + Copy,
    <T as Add<By>>::Output: Copy,
{
    type Output = Vector<<T as Add<By>>::Output, S>;

    fn add(self, rhs: Vector<By, S>) -> Self::Output {
        self.into_componentwise_op::<By, <T as Add<By>>::Output>(
            rhs,
            move |lhs_value, rhs_value| lhs_value + rhs_value,
        )
    }
}

impl<T, const S: usize, By> AddAssign<Vector<By, S>> for Vector<T, S>
where
    T: Sized + Copy + Add<By, Output = T>,
    By: Sized + Copy,
{
    fn add_assign(&mut self, rhs: Vector<By, S>) {
        self.assign_from_componentwise_op::<By>(rhs, move |lhs_value, rhs_value| {
            lhs_value + rhs_value
        })
    }
}
