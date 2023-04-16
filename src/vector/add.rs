use crate::ops::componentwise::{AssignComponentwise, Componentwise};
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
        self.componentwise(rhs, |lhs_value: T, rhs_value: Rhs| lhs_value + rhs_value)
    }
}

impl<T, const ROWS: usize, Rhs> AddAssign<Vector<Rhs, ROWS>> for Vector<T, ROWS>
where
    T: Copy + AddAssign<Rhs>,
    Rhs: Copy,
{
    fn add_assign(&mut self, rhs: Vector<Rhs, ROWS>) {
        self.assign_componentwise(rhs, |lhs_value: &mut T, rhs_value: Rhs| {
            *lhs_value += rhs_value
        })
    }
}

#[test]
fn associative_law() {
    let vector_a = Vector::<i32, 3>::from_array([1, 2, 3]);
    let vector_b = Vector::<i32, 3>::from_array([4, 5, 6]);
    let vector_c = Vector::<i32, 3>::from_array([7, 8, 9]);

    assert_eq!(
        (vector_a + vector_b) + vector_c,
        vector_a + (vector_b + vector_c),
        "Vector-Vector addition obeys associative law: (a+b)+c = a+(b+c)"
    )
}

#[test]
fn commutative_law() {
    let vector_a = Vector::<i32, 3>::from_array([1, 2, 3]);
    let vector_b = Vector::<i32, 3>::from_array([4, 5, 6]);

    assert_eq!(
        vector_a + vector_b,
        vector_b + vector_a,
        "Vector-Vector addition obeys commutative law: a+b = b+a"
    )
}
