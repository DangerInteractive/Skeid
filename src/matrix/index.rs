use crate::matrix::Matrix;
use std::ops::{Index, IndexMut};

impl<T, const R: usize, const C: usize> Index<(usize, usize)> for Matrix<T, R, C>
where
    T: Copy,
{
    type Output = T;

    fn index(&self, (column, row): (usize, usize)) -> &Self::Output {
        &self.0[column][row]
    }
}

impl<T, const R: usize, const C: usize> IndexMut<(usize, usize)> for Matrix<T, R, C>
where
    T: Copy,
{
    fn index_mut(&mut self, (column, row): (usize, usize)) -> &mut Self::Output {
        &mut self.0[column][row]
    }
}
