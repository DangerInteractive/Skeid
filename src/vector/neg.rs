use crate::vector::Vector;
use std::ops::Neg;

impl<T, const ROWS: usize> Neg for Vector<T, ROWS>
where
    T: Copy + Neg,
    <T as Neg>::Output: Copy,
{
    type Output = Vector<<T as Neg>::Output, ROWS>;

    fn neg(self) -> Self::Output {
        Self::Output::from_array(self.0.map(|v| -v))
    }
}

#[test]
fn negate_positive() {
    let vector = Vector::<i32, 3>::from_array([1, 244, 56]);
    let negative = Vector::<i32, 3>::from_array([-1, -244, -56]);

    assert_eq!(
        -vector, negative,
        "Vector negation is correct for vector of positive values"
    );
}

#[test]
fn negate_negative() {
    let vector = Vector::<i32, 3>::from_array([-1, -244, -56]);
    let negative = Vector::<i32, 3>::from_array([1, 244, 56]);

    assert_eq!(
        -vector, negative,
        "Vector negation is correct for vector of negative values"
    );
}

#[test]
fn negate_zero() {
    let vector = Vector::<i32, 3>::from_array([0, 0, 0]);

    assert_eq!(
        -vector, vector,
        "Vector negation does not change a vector containing only 0 values"
    );
}

#[test]
fn negate_mixed() {
    let vector = Vector::<i32, 3>::from_array([-124, 64, 0]);
    let negative = Vector::<i32, 3>::from_array([124, -64, 0]);

    assert_eq!(-vector, negative, "Vector negation is correct for vector with a combination of positive, negative, and 0 values");
}
