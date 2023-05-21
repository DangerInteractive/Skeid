//! Operators to multiply two matrices together

use crate::matrix::{Matrix, MatrixCoordinate};
use std::ops::{AddAssign, Mul};

impl<LhsT, RhsT, const LHS_COLUMNS: usize, const LHS_ROWS: usize, const RHS_COLUMNS: usize>
    Mul<Matrix<RhsT, LHS_COLUMNS, RHS_COLUMNS>> for Matrix<LhsT, LHS_ROWS, RHS_COLUMNS>
where
    LhsT: Copy + Mul<RhsT>,
    RhsT: Copy,
    <LhsT as Mul<RhsT>>::Output: AddAssign + Copy + From<i8>,
{
    type Output = Matrix<<LhsT as Mul<RhsT>>::Output, LHS_ROWS, RHS_COLUMNS>;

    fn mul(self, rhs: Matrix<RhsT, LHS_COLUMNS, RHS_COLUMNS>) -> Self::Output {
        Matrix::from_fn(|row, column| {
            let mut sum = From::from(0);
            for k in 0..LHS_COLUMNS {
                sum += self[MatrixCoordinate::new(k, row)] * rhs[MatrixCoordinate::new(column, k)];
            }
            sum
        })
    }
}

#[test]
fn known_product() {
    let matrix_a = Matrix::<i32, 3, 3>::from_array([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
    let matrix_b = Matrix::<i32, 3, 3>::from_array([[10, 11, 12], [13, 14, 15], [16, 17, 18]]);
    let product =
        Matrix::<i32, 3, 3>::from_array([[138, 171, 204], [174, 216, 258], [210, 261, 312]]);

    assert_eq!(
        matrix_a * matrix_b,
        product,
        "Matrix-Matrix multiplication matches known product"
    );
}

#[test]
fn associative_law() {
    let matrix_a = Matrix::<i32, 3, 3>::from_array([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
    let matrix_b = Matrix::<i32, 3, 3>::from_array([[10, 11, 12], [13, 14, 15], [16, 17, 18]]);
    let matrix_c = Matrix::<i32, 3, 3>::from_array([[19, 20, 21], [22, 23, 24], [25, 26, 27]]);

    assert_eq!(
        (matrix_a * matrix_b) * matrix_c,
        matrix_a * (matrix_b * matrix_c),
        "Matrix-Matrix multiplication obeys associative law: (AB)C = A(BC)"
    );
}

#[test]
fn distributive_law() {
    let matrix_a = Matrix::<i32, 3, 3>::from_array([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
    let matrix_b = Matrix::<i32, 3, 3>::from_array([[10, 11, 12], [13, 14, 15], [16, 17, 18]]);
    let matrix_c = Matrix::<i32, 3, 3>::from_array([[19, 20, 21], [22, 23, 24], [25, 26, 27]]);

    assert_eq!(
        matrix_a * (matrix_b + matrix_c),
        (matrix_a * matrix_b) + (matrix_a * matrix_c),
        "Matrix-Matrix multiplication obeys distributive law variation 1: A(B+C) = AB+AC"
    );

    assert_eq!(
        (matrix_a + matrix_b) * matrix_c,
        (matrix_a * matrix_c) + (matrix_b * matrix_c),
        "Matrix-Matrix multiplication obeys distributive law variation 2: (A+B)C = AC+BC"
    );
}

#[test]
fn scalar_factorization() {
    let matrix_a = Matrix::<i32, 3, 3>::from_array([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
    let matrix_b = Matrix::<i32, 3, 3>::from_array([[10, 11, 12], [13, 14, 15], [16, 17, 18]]);
    let scalar_t = 123;

    let result_1 = (matrix_a * scalar_t) * matrix_b;
    let result_2 = matrix_a * (matrix_b * scalar_t);
    let result_3 = (matrix_a * matrix_b) * scalar_t;

    assert_eq!(
        result_1, result_2,
        "Matrix-Matrix multiplication obeys scalar factorization variation 1: (At)B = A(Bt)"
    );
    assert_eq!(
        result_2, result_3,
        "Matrix-Matrix multiplication obeys scalar factorization variation 2: A(Bt) = (AB)t"
    );
    assert_eq!(
        result_3, result_1,
        "Matrix-Matrix multiplication obeys scalar factorization variation 3: (AB)t = (At)B"
    );
}

#[test]
fn product_rule_for_matrix_transpose() {
    let matrix_a = Matrix::<i32, 3, 3>::from_array([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
    let matrix_b = Matrix::<i32, 3, 3>::from_array([[10, 11, 12], [13, 14, 15], [16, 17, 18]]);

    assert_eq!(
        (matrix_a * matrix_b).transpose(),
        matrix_a.transpose() * matrix_b.transpose(),
        "Matrix-Matrix multiplication obeys the product rule for matrix transpose: (AB)ₜ = AₜBₜ"
    )
}
