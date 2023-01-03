use crate::ops::cross::Cross;
use crate::vector::Vector;
use std::ops::{Mul, Sub};

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
