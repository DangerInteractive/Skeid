use crate::vector::Vector;
use std::iter::FusedIterator;

/// an iterator that determines the order indices are returned by a `VectorIterator`
pub trait VectorIndexIterator<const ROWS: usize>: Iterator<Item = usize> {}

/// an implementation of `VectorIndexIterator`
/// that starts at index 0 and moves forward until the last element
#[derive(Debug)]
pub struct ForwardVectorIndexIterator<const ROWS: usize>(Option<usize>);

impl<const ROWS: usize> Iterator for ForwardVectorIndexIterator<ROWS> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.0;
        self.0 = match self.0 {
            Some(i) if i < ROWS - 1 => Some(i + 1),
            Some(_) => None, // fuse to None after returning ROWS - 1
            None => None,    // once None is returned, always return None
        };
        index
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (ROWS, Some(ROWS))
    }
}

impl<const ROWS: usize> Default for ForwardVectorIndexIterator<ROWS> {
    fn default() -> Self {
        Self(Some(0))
    }
}

impl<const ROWS: usize> ExactSizeIterator for ForwardVectorIndexIterator<ROWS> {}

impl<const ROWS: usize> FusedIterator for ForwardVectorIndexIterator<ROWS> {}

/// an implementation of `VectorIndexIterator`
/// that starts at the last index and moves backwards until index 0
#[derive(Debug)]
pub struct BackwardVectorIndexIterator<const ROWS: usize>(Option<usize>);

impl<const ROWS: usize> Iterator for BackwardVectorIndexIterator<ROWS> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.0;
        self.0 = match self.0 {
            Some(i) if i > 0 => Some(i - 1),
            Some(_) => None, // fuse to None after returning 0
            None => None,    // once None is returned, always return None
        };
        index
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (ROWS, Some(ROWS))
    }
}

impl<const ROWS: usize> Default for BackwardVectorIndexIterator<ROWS> {
    fn default() -> Self {
        Self(Some(ROWS))
    }
}

impl<const ROWS: usize> ExactSizeIterator for BackwardVectorIndexIterator<ROWS> {}

impl<const ROWS: usize> FusedIterator for BackwardVectorIndexIterator<ROWS> {}

/// an iterator for consuming the elements of a `Vector`
#[derive(Debug)]
pub struct VectorIterator<T, I, const ROWS: usize>
where
    T: Copy,
    I: Iterator<Item = usize> + FusedIterator,
{
    vector: Vector<T, ROWS>,
    row_iterator: I,
}

impl<T, I, const ROWS: usize> Iterator for VectorIterator<T, I, ROWS>
where
    T: Copy,
    I: Iterator<Item = usize> + FusedIterator,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let index = match self.row_iterator.next() {
            Some(i) => i,
            _ => return None,
        };
        Some(self.vector[index])
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.row_iterator.size_hint()
    }
}

impl<T, I, const ROWS: usize> ExactSizeIterator for VectorIterator<T, I, ROWS>
where
    T: Copy,
    I: Iterator<Item = usize> + FusedIterator + ExactSizeIterator,
{
}

impl<T, I, const ROWS: usize> FusedIterator for VectorIterator<T, I, ROWS>
where
    T: Copy,
    I: Iterator<Item = usize> + FusedIterator,
{
}

impl<T, const ROWS: usize> IntoIterator for Vector<T, ROWS>
where
    T: Copy,
{
    type Item = T;
    type IntoIter = VectorIterator<T, ForwardVectorIndexIterator<ROWS>, ROWS>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            vector: self,
            row_iterator: Default::default(),
        }
    }
}

/// an iterator for reading (by reference) the elements of a `Vector`
#[derive(Debug)]
pub struct VectorRefIterator<'a, T, I, const ROWS: usize>
where
    T: Copy,
    I: Iterator<Item = usize> + FusedIterator,
{
    vector: &'a Vector<T, ROWS>,
    row_iterator: I,
}

impl<'a, T, I, const ROWS: usize> Iterator for VectorRefIterator<'a, T, I, ROWS>
where
    T: Copy,
    I: Iterator<Item = usize> + FusedIterator,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let index = match self.row_iterator.next() {
            Some(i) => i,
            _ => return None,
        };
        Some(&self.vector[index])
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.row_iterator.size_hint()
    }
}

impl<T, I, const ROWS: usize> ExactSizeIterator for VectorRefIterator<'_, T, I, ROWS>
where
    T: Copy,
    I: Iterator<Item = usize> + FusedIterator + ExactSizeIterator,
{
}

impl<T, I, const ROWS: usize> FusedIterator for VectorRefIterator<'_, T, I, ROWS>
where
    T: Copy,
    I: Iterator<Item = usize> + FusedIterator,
{
}

impl<'a, T, const ROWS: usize> IntoIterator for &'a Vector<T, ROWS>
where
    T: Copy,
{
    type Item = &'a T;
    type IntoIter = VectorRefIterator<'a, T, ForwardVectorIndexIterator<ROWS>, ROWS>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            vector: self,
            row_iterator: Default::default(),
        }
    }
}
