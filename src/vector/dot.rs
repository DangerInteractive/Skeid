use crate::ops::dot::Dot;
use crate::vector::Vector;
use std::ops::{AddAssign, Mul};

impl<T, const ROWS: usize> Dot for Vector<T, ROWS>
where
    T: Copy + Default + Mul + AddAssign<<T as Mul>::Output>,
{
    type Output = T;

    fn dot(self, rhs: Self) -> Self::Output {
        let mut sum: Self::Output = Default::default();
        for i in 0..ROWS {
            sum += self[i] * rhs[i];
        }
        sum
    }
}

#[test]
fn commutative_law() {
    let vector_a = Vector::<i32, 3>::from_array([1, 2, 3]);
    let vector_b = Vector::<i32, 3>::from_array([4, 5, 6]);

    assert_eq!(
        vector_a.dot(vector_b),
        vector_b.dot(vector_a),
        "Vector dot product obeys commutative law: a·b = b·a"
    );
}

#[test]
fn distributive_law() {
    let vector_a = Vector::<i32, 3>::from_array([1, 2, 3]);
    let vector_b = Vector::<i32, 3>::from_array([4, 5, 6]);
    let vector_c = Vector::<i32, 3>::from_array([7, 8, 9]);

    assert_eq!(
        vector_a.dot(vector_b + vector_c),
        vector_a.dot(vector_b) + vector_a.dot(vector_c),
        "Vector dot product obeys distributive law: a·(b+c) = a·b+a·c"
    );
}

#[test]
fn scalar_factorization() {
    let vector_a = Vector::<i32, 3>::from_array([1, 2, 3]);
    let vector_b = Vector::<i32, 3>::from_array([4, 5, 6]);
    let scalar_t = 123;

    let result_1 = (vector_a * scalar_t).dot(vector_b);
    let result_2 = vector_a.dot(vector_b * scalar_t);
    let result_3 = vector_a.dot(vector_b) * scalar_t;

    assert_eq!(
        result_1, result_2,
        "Vector dot product obeys scalar factorization variation 1: (at)·b = a·(bt)"
    );
    assert_eq!(
        result_2, result_3,
        "Vector dot product obeys scalar factorization variation 2: a·(bt) = (a·b)t"
    );
    assert_eq!(
        result_3, result_1,
        "Vector dot product obeys scalar factorization variation 3: (a·b)t = (at)·b"
    );
}
