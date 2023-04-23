use crate::matrix::Matrix;
use std::ops::Neg;

impl<T, const ROWS: usize, const COLUMNS: usize> Neg for Matrix<T, ROWS, COLUMNS>
where
    T: Copy + Neg,
    <T as Neg>::Output: Copy,
{
    type Output = Matrix<<T as Neg>::Output, ROWS, COLUMNS>;

    fn neg(self) -> Self::Output {
        Self::Output::from_array(self.0.map(|c| c.map(|v| -v)))
    }
}

#[test]
fn negate_positive() {
    let matrix = Matrix::<i32, 3, 3>::from_array([[52, 84, 37], [77, 1, 22], [20, 106, 80]]);
    let negative =
        Matrix::<i32, 3, 3>::from_array([[-52, -84, -37], [-77, -1, -22], [-20, -106, -80]]);

    assert_eq!(
        -matrix, negative,
        "Matrix negation is correct for matrix of positive values"
    );
}

#[test]
fn negate_negative() {
    let matrix =
        Matrix::<i32, 3, 3>::from_array([[-52, -84, -37], [-77, -1, -22], [-20, -106, -80]]);
    let negative = Matrix::<i32, 3, 3>::from_array([[52, 84, 37], [77, 1, 22], [20, 106, 80]]);

    assert_eq!(
        -matrix, negative,
        "Matrix negation is correct for matrix of negative values"
    );
}

#[test]
fn negate_zero() {
    let matrix = Matrix::<i32, 3, 3>::from_array([[0, 0, 0], [0, 0, 0], [0, 0, 0]]);

    assert_eq!(
        -matrix, matrix,
        "Matrix negation does not change a matrix containing only 0 values"
    );
}

#[test]
fn negate_mixed() {
    let matrix = Matrix::<i32, 3, 3>::from_array([[111, -116, 0], [97, 4, 21], [61, -124, 43]]);
    let negative =
        Matrix::<i32, 3, 3>::from_array([[-111, 116, 0], [-97, -4, -21], [-61, 124, -43]]);

    assert_eq!(-matrix, negative, "Matrix negation is correct for matrix with a combination of positive, negative, and 0 values");
}
