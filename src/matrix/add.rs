use crate::matrix::Matrix;
use crate::ops::componentwise::{AssignComponentwise, Componentwise};
use std::ops::{Add, AddAssign};

impl<T, const ROWS: usize, const COLUMNS: usize, Rhs> Add<Matrix<Rhs, ROWS, COLUMNS>>
    for Matrix<T, ROWS, COLUMNS>
where
    T: Copy + Add<Rhs>,
    Rhs: Copy,
    <T as Add<Rhs>>::Output: Copy,
{
    type Output = Matrix<<T as Add<Rhs>>::Output, ROWS, COLUMNS>;

    fn add(self, rhs: Matrix<Rhs, ROWS, COLUMNS>) -> Self::Output {
        self.componentwise(rhs, |lhs_value: T, rhs_value: Rhs| lhs_value + rhs_value)
    }
}

impl<T, const ROWS: usize, const COLUMNS: usize, Rhs> AddAssign<Matrix<Rhs, ROWS, COLUMNS>>
    for Matrix<T, ROWS, COLUMNS>
where
    T: Copy + AddAssign<Rhs>,
    Rhs: Copy,
{
    fn add_assign(&mut self, rhs: Matrix<Rhs, ROWS, COLUMNS>) {
        self.assign_componentwise(rhs, |lhs_value: &mut T, rhs_value: Rhs| {
            *lhs_value += rhs_value
        })
    }
}

#[test]
fn associative_law() {
    let matrix_a = Matrix::<i32, 3, 3>::from_array([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
    let matrix_b = Matrix::<i32, 3, 3>::from_array([[10, 11, 12], [13, 14, 15], [16, 17, 18]]);
    let matrix_c = Matrix::<i32, 3, 3>::from_array([[19, 20, 21], [22, 23, 24], [25, 26, 27]]);

    assert_eq!(
        (matrix_a + matrix_b) + matrix_c,
        matrix_a + (matrix_b + matrix_c),
        "Matrix-Matrix addition obeys associative law: (A+B)+C = A+(B+C)"
    );
}

#[test]
fn commutative_law() {
    let matrix_a = Matrix::<i32, 3, 3>::from_array([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
    let matrix_b = Matrix::<i32, 3, 3>::from_array([[10, 11, 12], [13, 14, 15], [16, 17, 18]]);

    assert_eq!(
        matrix_a + matrix_b,
        matrix_b + matrix_a,
        "Matrix-Matrix addition obeys commutative law: A+B = B+A"
    );
}
