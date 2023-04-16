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
