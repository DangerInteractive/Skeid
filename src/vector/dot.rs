use crate::ops::dot::{Dot};
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
