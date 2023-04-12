use crate::marker::Scalar;
use crate::matrix::Matrix;
use crate::ops::componentwise::{AssignComponentwise, Componentwise};
use std::ops::{Mul, MulAssign};

impl<T, const ROWS: usize, const COLUMNS: usize, Rhs> Mul<Rhs> for Matrix<T, ROWS, COLUMNS>
where
    T: Copy + Mul<Rhs>,
    Rhs: Scalar + Copy,
    <T as Mul<Rhs>>::Output: Copy,
{
    type Output = Matrix<<T as Mul<Rhs>>::Output, ROWS, COLUMNS>;

    fn mul(self, rhs: Rhs) -> Self::Output {
        self.componentwise(rhs, |lhs_value, rhs_value| lhs_value * rhs_value)
    }
}

impl<T, const ROWS: usize, const COLUMNS: usize, Rhs> MulAssign<Rhs> for Matrix<T, ROWS, COLUMNS>
where
    T: Copy + MulAssign<Rhs>,
    Rhs: Scalar + Copy,
{
    fn mul_assign(&mut self, rhs: Rhs) {
        self.assign_componentwise(rhs, |lhs_value, rhs_value| *lhs_value *= rhs_value)
    }
}

#[test]
fn known_product() {
    let matrix_a = Matrix::<i32, 3, 3>::from_array([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
    let scalar_t = 123;
    let product =
        Matrix::<i32, 3, 3>::from_array([[123, 246, 369], [492, 615, 738], [861, 984, 1107]]);

    assert_eq!(
        matrix_a * scalar_t,
        product,
        "Matrix-Scalar multiplication matches known product"
    )
}

#[test]
fn associative_law() {
    let scalar_s = 123;
    let scalar_t = 456;
    let matrix_a = Matrix::<i32, 3, 3>::from_array([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);

    assert_eq!(
        matrix_a * (scalar_s * scalar_t),
        (matrix_a * scalar_s) * scalar_t,
        "Matrix-Scalar multiplication obeys the associative law: A(st) = (As)t"
    )
}

// Not sure if I can test the commutative law: tA = At
// since Rust won't allow implementing math operators with built-in types on the left hand side.
// We're relying on this law being true to force users of the library to put the Matrix on the left

#[test]
fn distributive_law() {
    let scalar_s = 123;
    let scalar_t = 456;
    let matrix_a = Matrix::<i32, 3, 3>::from_array([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
    let matrix_b = Matrix::<i32, 3, 3>::from_array([[10, 11, 12], [13, 14, 15], [16, 17, 18]]);

    assert_eq!(
        (matrix_a + matrix_b) * scalar_t,
        (matrix_a * scalar_t) + (matrix_b * scalar_t),
        "Matrix-Scalar multiplication obeys the distributive law variation 1: (A+B)t = At+Bt"
    );
    assert_eq!(
        matrix_a * (scalar_s + scalar_t),
        (matrix_a * scalar_s) + (matrix_a * scalar_t),
        "Matrix-Scalar multiplication obeys the distributive law variation 2: A(s+t) = As+At"
    )
}
