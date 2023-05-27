//! iterating over matrices

use crate::matrix::{Matrix, MatrixCoordinate};
use std::iter::FusedIterator;

/// An iterator that iterates coordinates of a matrix given start and end points,
/// iterating each element in a column, then moving to the next column.
#[derive(Debug)]
pub struct MatrixAreaIterator {
    start: MatrixCoordinate,
    end: MatrixCoordinate,
    front: MatrixCoordinate,
    back: MatrixCoordinate,
}

impl MatrixAreaIterator {
    /// construct a new MatrixAreaIterator given start and end coordinates
    #[must_use]
    pub fn new(start: MatrixCoordinate, end: MatrixCoordinate) -> Self {
        Self {
            start,
            end,
            front: start,
            back: end,
        }
    }
}

impl Iterator for MatrixAreaIterator {
    type Item = MatrixCoordinate;

    fn next(&mut self) -> Option<Self::Item> {
        let next = match self.front {
            coord if coord.column < self.back.column => coord,
            coord if coord.column == self.back.column && coord.row <= self.back.row => coord,
            _ => return None, // we've iterated past the back
        };
        self.front = match self.front {
            coord if coord.row < self.end.row => MatrixCoordinate::new(coord.column, coord.row + 1),
            coord => MatrixCoordinate::new(coord.column + 1, self.start.row),
        };
        Some(next)
    }
}

impl DoubleEndedIterator for MatrixAreaIterator {
    fn next_back(&mut self) -> Option<Self::Item> {
        let next = match self.back {
            coord if coord.column > self.front.column => coord,
            coord if coord.column == self.front.column && coord.row >= self.back.row => coord,
            _ => return None, // we've iterated past the front
        };
        self.back = match self.back {
            coord if coord.row > self.start.row => {
                MatrixCoordinate::new(coord.column, coord.row - 1)
            }
            coord => MatrixCoordinate::new(coord.column - 1, self.end.row),
        };
        Some(next)
    }
}

/// an iterator for consuming the elements of a `Matrix`
#[derive(Debug)]
pub struct MatrixIterator<T, const ROWS: usize, const COLUMNS: usize, I>
where
    T: Copy,
    I: Iterator<Item = MatrixCoordinate>,
{
    pub(crate) matrix: Matrix<T, ROWS, COLUMNS>,
    pub(crate) coordinate_iterator: I,
}

impl<T, const ROWS: usize, const COLUMNS: usize, I> Iterator for MatrixIterator<T, ROWS, COLUMNS, I>
where
    T: Copy,
    I: Iterator<Item = MatrixCoordinate>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let coordinate = match self.coordinate_iterator.next() {
            Some(coord) => coord,
            None => return None,
        };
        // we assume the index given is in bounds (if not, this panics)
        Some(self.matrix[coordinate])
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.coordinate_iterator.size_hint()
    }
}

impl<T, const ROWS: usize, const COLUMNS: usize, I> DoubleEndedIterator
    for MatrixIterator<T, ROWS, COLUMNS, I>
where
    T: Copy,
    I: Iterator<Item = MatrixCoordinate> + DoubleEndedIterator,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        let coordinate = match self.coordinate_iterator.next_back() {
            Some(coord) => coord,
            None => return None,
        };
        // we assume the index given is in bounds (if not, this panics)
        Some(self.matrix[coordinate])
    }
}

impl<T, const ROWS: usize, const COLUMNS: usize, I> FusedIterator
    for MatrixIterator<T, ROWS, COLUMNS, I>
where
    T: Copy,
    I: Iterator<Item = MatrixCoordinate> + FusedIterator,
{
}

impl<T, const ROWS: usize, const COLUMNS: usize, I> ExactSizeIterator
    for MatrixIterator<T, ROWS, COLUMNS, I>
where
    T: Copy,
    I: Iterator<Item = MatrixCoordinate> + ExactSizeIterator,
{
}

impl<T, const ROWS: usize, const COLUMNS: usize> IntoIterator for Matrix<T, ROWS, COLUMNS>
where
    T: Copy,
{
    type Item = T;
    type IntoIter = MatrixIterator<T, ROWS, COLUMNS, MatrixAreaIterator>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            matrix: self,
            coordinate_iterator: MatrixAreaIterator::new(
                MatrixCoordinate::new(0, 0),
                MatrixCoordinate::new(COLUMNS - 1, ROWS - 1),
            ),
        }
    }
}

/// an iterator for reading (by reference) the elements of a `Matrix`
#[derive(Debug)]
pub struct MatrixReferenceIterator<'a, T, const ROWS: usize, const COLUMNS: usize, I>
where
    T: Copy,
    I: Iterator<Item = MatrixCoordinate>,
{
    pub(crate) matrix: &'a Matrix<T, ROWS, COLUMNS>,
    pub(crate) coordinate_iterator: I,
}

impl<'a, T, const ROWS: usize, const COLUMNS: usize, I> Iterator
    for MatrixReferenceIterator<'a, T, ROWS, COLUMNS, I>
where
    T: Copy,
    I: Iterator<Item = MatrixCoordinate>,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let coordinate = match self.coordinate_iterator.next() {
            Some(coord) => coord,
            None => return None,
        };
        // we assume the index given is in bounds (if not, this panics)
        Some(&self.matrix[coordinate])
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.coordinate_iterator.size_hint()
    }
}

impl<T, const ROWS: usize, const COLUMNS: usize, I> FusedIterator
    for MatrixReferenceIterator<'_, T, ROWS, COLUMNS, I>
where
    T: Copy,
    I: Iterator<Item = MatrixCoordinate> + FusedIterator,
{
}

impl<T, const ROWS: usize, const COLUMNS: usize, I> ExactSizeIterator
    for MatrixReferenceIterator<'_, T, ROWS, COLUMNS, I>
where
    T: Copy,
    I: Iterator<Item = MatrixCoordinate> + ExactSizeIterator,
{
}

impl<'a, T, const ROWS: usize, const COLUMNS: usize> IntoIterator for &'a Matrix<T, ROWS, COLUMNS>
where
    T: Copy,
{
    type Item = &'a T;
    type IntoIter = MatrixReferenceIterator<'a, T, ROWS, COLUMNS, MatrixAreaIterator>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            matrix: self,
            coordinate_iterator: MatrixAreaIterator::new(
                MatrixCoordinate::new(0, 0),
                MatrixCoordinate::new(COLUMNS - 1, ROWS - 1),
            ),
        }
    }
}
