//! iterating over matrices

use crate::matrix::{Matrix, MatrixCoordinate};
use std::iter::FusedIterator;

/// An iterator that iterates coordinates of a matrix given start and end points,
/// iterating each element in a column, then moving to the next column.
#[derive(Debug)]
pub struct MatrixAreaIterator {
    start: MatrixCoordinate,
    row_size: usize,
    size: usize,
    state: Option<(usize, usize)>,
}

impl MatrixAreaIterator {
    /// construct a new MatrixAreaIterator given start and end coordinates
    #[must_use]
    pub fn new(start: MatrixCoordinate, end: MatrixCoordinate) -> Self {
        Self {
            start,
            row_size: (end.row - start.row),
            size: (end.column - start.column) * (end.row - end.column),
            state: Some((0, 0)),
        }
    }
}

impl Iterator for MatrixAreaIterator {
    type Item = MatrixCoordinate;

    fn next(&mut self) -> Option<Self::Item> {
        let next = match self.state {
            Some((front, _)) => MatrixCoordinate::new(
                front / self.row_size + self.start.column,
                front % self.row_size + self.start.row,
            ),
            None => return None, // fused
        };
        self.state = match self.state {
            Some((front, back)) if self.size - back > front => Some((front + 1, back)),
            Some(_) => None, // trip fuse, the whole matrix has been iterated
            None => None,    // fused
        };
        Some(next)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.size, Some(self.size))
    }
}

impl DoubleEndedIterator for MatrixAreaIterator {
    fn next_back(&mut self) -> Option<Self::Item> {
        let next = match self.state {
            Some((_, back)) => {
                let back_index = self.size - 1 - back;
                MatrixCoordinate::new(back_index / self.row_size, back_index % self.row_size)
            }
            None => return None, // fused
        };
        self.state = match self.state {
            Some((front, back)) if self.size - back > front => Some((front, back + 1)),
            Some(_) => None, // trip fuse, the whole matrix has been iterated
            None => None,    // fused
        };
        Some(next)
    }
}

impl FusedIterator for MatrixAreaIterator {}

impl ExactSizeIterator for MatrixAreaIterator {}

#[test]
fn matrix_area_iterator_iteration_order() {
    MatrixAreaIterator::new(MatrixCoordinate::new(0, 0), MatrixCoordinate::new(3, 3))
        .enumerate()
        .for_each(|(i, coord)| {
            assert_eq!(coord, MatrixCoordinate::new(i / 3, i % 3));
        });
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
