use crate::marker::Scalar;
use crate::ops::componentwise::{AssignComponentwise, Componentwise};
use crate::vector::Vector;
use std::ops::{Mul, MulAssign};

impl<T, const ROWS: usize, Rhs> Mul<Rhs> for Vector<T, ROWS>
where
    T: Copy + Mul<Rhs>,
    Rhs: Scalar + Copy,
    <T as Mul<Rhs>>::Output: Copy,
{
    type Output = Vector<<T as Mul<Rhs>>::Output, ROWS>;

    fn mul(self, rhs: Rhs) -> Self::Output {
        self.componentwise(rhs, |lhs_value, rhs_value| lhs_value * rhs_value)
    }
}

impl<T, const ROWS: usize, Rhs> MulAssign<Rhs> for Vector<T, ROWS>
where
    T: Copy + MulAssign<Rhs>,
    Rhs: Scalar + Copy,
{
    fn mul_assign(&mut self, rhs: Rhs) {
        self.assign_componentwise(rhs, |lhs_value, rhs_value| *lhs_value *= rhs_value)
    }
}

#[test]
fn associative_law() {
    let vector_a = Vector::<i32, 3>::from_array([1, 2, 3]);
    let scalar_s = 123;
    let scalar_t = 234;

    assert_eq!(
        vector_a * (scalar_s * scalar_t),
        (vector_a * scalar_s) * scalar_t,
        "Vector-Scalar multiplication obeys associative law: a(st) = (as)t"
    )
}

// Not sure if I can test the commutative law: ta = at
// since Rust won't allow implementing math operators with built-in types on the left hand side.
// We're relying on this law being true to force users of the library to put the Vector on the left

#[test]
fn distributive_law() {
    let vector_a = Vector::<i32, 3>::from_array([1, 2, 3]);
    let vector_b = Vector::<i32, 3>::from_array([4, 5, 6]);
    let scalar_s = 123;
    let scalar_t = 234;

    assert_eq!(
        (vector_a + vector_b) * scalar_t,
        vector_a * scalar_t + vector_b * scalar_t,
        "Vector-Scalar multiplication obeys distributive law variation 1: (a+b)t = at + bt"
    );

    assert_eq!(
        vector_a * (scalar_s + scalar_t),
        vector_a * scalar_s + vector_a * scalar_t,
        "Vector-Scalar multiplication obeys distributive law variation 2: a(s+t) = as + at"
    );
}
