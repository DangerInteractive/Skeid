//! iterating over `Vector`s

use crate::vector::Vector;
use std::iter::FusedIterator;
use std::ops::Range;

/// an iterator for consuming the elements of a `Vector`
#[derive(Debug)]
pub struct VectorIterator<T, const ROWS: usize, I>
where
    T: Copy,
    I: Iterator<Item = usize>,
{
    pub(crate) vector: Vector<T, ROWS>,
    pub(crate) row_iterator: I,
}

impl<T, const ROWS: usize, I> Iterator for VectorIterator<T, ROWS, I>
where
    T: Copy,
    I: Iterator<Item = usize>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let index = match self.row_iterator.next() {
            Some(i) => i,
            None => return None,
        };
        // we assume the index given is in bounds (if not, this panics)
        Some(self.vector[index])
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.row_iterator.size_hint()
    }
}

impl<T, const ROWS: usize, I> DoubleEndedIterator for VectorIterator<T, ROWS, I>
where
    T: Copy,
    I: Iterator<Item = usize> + DoubleEndedIterator,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        let index = match self.row_iterator.next_back() {
            Some(i) => i,
            None => return None,
        };
        // we assume the index given is in bounds (if not, this panics)
        Some(self.vector[index])
    }
}

impl<T, const ROWS: usize, I> FusedIterator for VectorIterator<T, ROWS, I>
where
    T: Copy,
    I: Iterator<Item = usize> + FusedIterator,
{
}

impl<T, const ROWS: usize, I> ExactSizeIterator for VectorIterator<T, ROWS, I>
where
    T: Copy,
    I: Iterator<Item = usize> + ExactSizeIterator,
{
}

impl<T, const ROWS: usize> IntoIterator for Vector<T, ROWS>
where
    T: Copy,
{
    type Item = T;
    type IntoIter = VectorIterator<T, ROWS, Range<usize>>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            vector: self,
            row_iterator: 0..ROWS,
        }
    }
}

/// an iterator for reading (by reference) the elements of a `Vector`
#[derive(Debug)]
pub struct VectorReferenceIterator<'a, T, const ROWS: usize, I>
where
    T: Copy,
    I: Iterator<Item = usize>,
{
    pub(crate) vector: &'a Vector<T, ROWS>,
    pub(crate) row_iterator: I,
}

impl<'a, T, const ROWS: usize, I> Iterator for VectorReferenceIterator<'a, T, ROWS, I>
where
    T: Copy,
    I: Iterator<Item = usize>,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let index = match self.row_iterator.next() {
            Some(i) => i,
            _ => return None,
        };
        // we assume the index given is in bounds (if not, this panics)
        Some(&self.vector[index])
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.row_iterator.size_hint()
    }
}

impl<T, const ROWS: usize, I> DoubleEndedIterator for VectorReferenceIterator<'_, T, ROWS, I>
where
    T: Copy,
    I: Iterator<Item = usize> + DoubleEndedIterator,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        let index = match self.row_iterator.next_back() {
            Some(i) => i,
            None => return None,
        };
        // we assume the index given is in bounds (if not, this panics)
        Some(&self.vector[index])
    }
}

impl<T, I, const ROWS: usize> FusedIterator for VectorReferenceIterator<'_, T, ROWS, I>
where
    T: Copy,
    I: Iterator<Item = usize> + FusedIterator,
{
}

impl<T, I, const ROWS: usize> ExactSizeIterator for VectorReferenceIterator<'_, T, ROWS, I>
where
    T: Copy,
    I: Iterator<Item = usize> + ExactSizeIterator,
{
}

impl<'a, T, const ROWS: usize> IntoIterator for &'a Vector<T, ROWS>
where
    T: Copy,
{
    type Item = &'a T;
    type IntoIter = VectorReferenceIterator<'a, T, ROWS, Range<usize>>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            vector: self,
            row_iterator: 0..ROWS,
        }
    }
}
