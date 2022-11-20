use std::ops::{Add, AddAssign};
use crate::marker::Scalar;
use crate::vector::Vector;

impl<T, const S: usize, By> Add<Vector<By, S>> for Vector<T, S>
where
    T: Sized + Copy + Add<By>,
    By: Sized + Copy,
{
    type Output = Vector<<T as Add<By>>::Output, S>;

    fn add(self, rhs: Vector<By, S>) -> Self::Output {
        self.into_vector_op::<By, <T as Add<By>>::Output>(rhs, move |lhs_value, rhs_value| {
            lhs_value + rhs_value
        })
    }
}

impl<T, const S: usize, By> Add<By> for Vector<T, S>
where
    T: Sized + Copy + Add<By>,
    By: Scalar + Sized + Copy,
{
    type Output = Vector<<T as Add<By>>::Output, S>;

    fn add(self, rhs: By) -> Self::Output {
        self.into_scalar_op(rhs, move |lhs_value, rhs_value| lhs_value + rhs_value)
    }
}

impl<T, const S: usize, By> AddAssign<Vector<By, S>> for Vector<T, S>
where
    T: Sized + Copy + Add<By, Output = T>,
    By: Sized + Copy,
{
    fn add_assign(&mut self, rhs: Vector<By, S>) {
        self.assign_from_vector_op::<By>(rhs, move |lhs_value, rhs_value| lhs_value + rhs_value)
    }
}

impl<T, const S: usize, By> AddAssign<By> for Vector<T, S>
where
    T: Sized + Copy + Add<By, Output = T>,
    By: Scalar + Sized + Copy,
{
    fn add_assign(&mut self, rhs: By) {
        self.assign_from_scalar_op::<By>(rhs, move |lhs_value, rhs_value| lhs_value + rhs_value)
    }
}
