use crate::vector::Vector;
use std::ops::{Index, IndexMut};

impl<T, const R: usize> Index<usize> for Vector<T, R>
where
    T: Copy,
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T, const R: usize> IndexMut<usize> for Vector<T, R>
where
    T: Copy,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}
