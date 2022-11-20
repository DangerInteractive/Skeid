use crate::matrix::Matrix;
use crate::vector::Vector;
use std::ops::{Index, IndexMut};

impl<T, const R: usize, const C: usize> Index<usize> for Matrix<T, R, C> {
    type Output = Vector<T, R>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.array[index]
    }
}

impl<T, const R: usize, const C: usize> IndexMut<usize> for Matrix<T, R, C> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.array[index]
    }
}
