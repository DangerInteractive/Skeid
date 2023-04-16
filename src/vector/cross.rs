use crate::ops::cross::Cross;
use crate::vector::Vector;
use std::ops::{Mul, Sub};

#[cfg(test)]
use crate::ops::dot::Dot;

impl<T> Cross for Vector<T, 3>
where
    T: Copy + Mul,
    <T as Mul>::Output: Sub,
    <<T as Mul>::Output as Sub>::Output: Copy,
{
    type Output = Vector<<<T as Mul>::Output as Sub>::Output, 3>;

    fn cross(self, rhs: Self) -> Self::Output {
        Vector::from_array([
            (self[1] * rhs[2]) - (self[2] * rhs[1]),
            (self[2] * rhs[0]) - (self[0] * rhs[2]),
            (self[0] * rhs[1]) - (self[1] * rhs[0]),
        ])
    }
}

#[test]
fn self_cross() {
    let vector_a = Vector::<i32, 3>::from_array([1, 2, 3]);
    let vector_zero = Vector::<i32, 3>::zero();

    assert_eq!(
        vector_a.cross(vector_a),
        vector_zero,
        "The cross product of a vector with itself is the zero vector: a×a = 0"
    );
}

#[test]
fn anticommutativity() {
    let vector_a = Vector::<i32, 3>::from_array([1, 2, 3]);
    let vector_b = Vector::<i32, 3>::from_array([4, 5, 6]);

    assert_eq!(
        vector_a.cross(vector_b),
        -vector_b.cross(vector_a),
        "Vector cross product is anticommutative: a×b = -b×a"
    );
}

#[test]
fn distributive_law_over_addition() {
    let vector_a = Vector::<i32, 3>::from_array([1, 2, 3]);
    let vector_b = Vector::<i32, 3>::from_array([4, 5, 6]);
    let vector_c = Vector::<i32, 3>::from_array([7, 8, 9]);

    assert_eq!(
        vector_a.cross(vector_b + vector_c),
        vector_a.cross(vector_b) + vector_a.cross(vector_c),
        "Vector cross product obeys distributive law over addition: a×(b+c) = a×b+a×c"
    );
}

#[test]
fn jacobi_identity() {
    let vector_a = Vector::<i32, 3>::from_array([1, 2, 3]);
    let vector_b = Vector::<i32, 3>::from_array([4, 5, 6]);
    let vector_c = Vector::<i32, 3>::from_array([7, 8, 9]);
    let vector_zero = Vector::<i32, 3>::zero();

    assert_eq!(
        vector_a.cross(vector_b.cross(vector_c))
            + vector_b.cross(vector_c.cross(vector_a))
            + vector_c.cross(vector_a.cross(vector_b)),
        vector_zero,
        "Vector cross product obeys Jacobi identity: a×(b×c)+b×(c×a)+c×(a×b) = 0"
    )
}

#[test]
fn sum_of_two_cross_products_identity() {
    let vector_a = Vector::<i32, 3>::from_array([1, 2, 3]);
    let vector_b = Vector::<i32, 3>::from_array([4, 5, 6]);
    let vector_c = Vector::<i32, 3>::from_array([7, 8, 9]);
    let vector_d = Vector::<i32, 3>::from_array([10, 11, 12]);

    assert_eq!(
        vector_a.cross(vector_b) + vector_c.cross(vector_d),
        (vector_a - vector_c).cross(vector_b - vector_d) + vector_a.cross(vector_d) + vector_c.cross(vector_b),
        "Vector cross product obeys identity for sum of two cross products: a×b+c×d = (a-c)×(b-d)+a×d+c×b"
    );
}

#[test]
fn scalar_factorization() {
    let vector_a = Vector::<i32, 3>::from_array([1, 2, 3]);
    let vector_b = Vector::<i32, 3>::from_array([4, 5, 6]);
    let scalar_t = 123;

    let result_1 = (vector_a * scalar_t).cross(vector_b);
    let result_2 = vector_a.cross(vector_b * scalar_t);
    let result_3 = vector_a.cross(vector_b) * scalar_t;

    assert_eq!(
        result_1, result_2,
        "Vector cross product obeys scalar factorization variation 1: (at)×b = a×(bt)"
    );
    assert_eq!(
        result_2, result_3,
        "Vector cross product obeys scalar factorization variation 2: a×(bt) = (a×b)t"
    );
    assert_eq!(
        result_3, result_1,
        "Vector cross product obeys scalar factorization variation 3: (a×b)t = (at)×b"
    );
}

#[test]
fn vector_triple_product() {
    let vector_a = Vector::<i32, 3>::from_array([1, 2, 3]);
    let vector_b = Vector::<i32, 3>::from_array([4, 5, 6]);
    let vector_c = Vector::<i32, 3>::from_array([7, 8, 9]);

    assert_eq!(
        vector_a.cross(vector_b.cross(vector_c)),
        vector_b * vector_a.dot(vector_c) - vector_c * vector_a.dot(vector_b),
        "Vector cross product obeys vector triple product: a×(b×c) = b(a·c)-c(a·b)"
    );
}

#[test]
fn lagrange_identity() {
    let vector_a = Vector::<i32, 3>::from_array([1, 2, 3]);
    let vector_b = Vector::<i32, 3>::from_array([4, 5, 6]);

    let a_cross_b = vector_a.cross(vector_b);
    let a_dot_b = vector_a.dot(vector_b);

    assert_eq!(
        a_cross_b.dot(a_cross_b),
        vector_a.dot(vector_a) * vector_b.dot(vector_b) - (a_dot_b * a_dot_b),
        "Vector cross product obeys Lagrange's identity: (a×b)² = a²b²-(a·b)²"
    );
}
