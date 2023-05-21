use crate::matrix::{Matrix, MatrixCoordinate};
use std::ops::{Index, IndexMut};

impl<T, const ROWS: usize, const COLUMNS: usize> Index<MatrixCoordinate>
    for Matrix<T, ROWS, COLUMNS>
where
    T: Copy,
{
    type Output = T;

    fn index(&self, MatrixCoordinate { column, row }: MatrixCoordinate) -> &Self::Output {
        &self.0[column][row]
    }
}

impl<T, const ROWS: usize, const COLUMNS: usize> IndexMut<MatrixCoordinate>
    for Matrix<T, ROWS, COLUMNS>
where
    T: Copy,
{
    fn index_mut(
        &mut self,
        MatrixCoordinate { column, row }: MatrixCoordinate,
    ) -> &mut Self::Output {
        &mut self.0[column][row]
    }
}

#[test]
fn indexes_correct_column() {
    let m = Matrix::<u8, 3, 3>::from_array([[0, 0, 0], [1, 1, 1], [2, 2, 2]]);

    // first column
    assert_eq!(m[MatrixCoordinate::new(0, 0)], 0);
    assert_eq!(m[MatrixCoordinate::new(0, 1)], 0);
    assert_eq!(m[MatrixCoordinate::new(0, 2)], 0);

    // second column
    assert_eq!(m[MatrixCoordinate::new(1, 0)], 1);
    assert_eq!(m[MatrixCoordinate::new(1, 1)], 1);
    assert_eq!(m[MatrixCoordinate::new(1, 2)], 1);

    // third column
    assert_eq!(m[MatrixCoordinate::new(2, 0)], 2);
    assert_eq!(m[MatrixCoordinate::new(2, 1)], 2);
    assert_eq!(m[MatrixCoordinate::new(2, 2)], 2);
}

#[test]
fn indexes_correct_row() {
    let m = Matrix::<u8, 3, 3>::from_array([[0, 1, 2], [0, 1, 2], [0, 1, 2]]);

    // first row
    assert_eq!(m[MatrixCoordinate::new(0, 0)], 0);
    assert_eq!(m[MatrixCoordinate::new(1, 0)], 0);
    assert_eq!(m[MatrixCoordinate::new(2, 0)], 0);

    // second row
    assert_eq!(m[MatrixCoordinate::new(0, 1)], 1);
    assert_eq!(m[MatrixCoordinate::new(1, 1)], 1);
    assert_eq!(m[MatrixCoordinate::new(2, 1)], 1);

    // third row
    assert_eq!(m[MatrixCoordinate::new(0, 2)], 2);
    assert_eq!(m[MatrixCoordinate::new(1, 2)], 2);
    assert_eq!(m[MatrixCoordinate::new(2, 2)], 2);
}

#[test]
fn indexes_correct_element() {
    let m = Matrix::<u8, 3, 3>::from_array([[0, 1, 2], [3, 4, 5], [6, 7, 8]]);

    assert_eq!(m[MatrixCoordinate::new(0, 0)], 0);
    assert_eq!(m[MatrixCoordinate::new(0, 1)], 1);
    assert_eq!(m[MatrixCoordinate::new(0, 2)], 2);
    assert_eq!(m[MatrixCoordinate::new(1, 0)], 3);
    assert_eq!(m[MatrixCoordinate::new(1, 1)], 4);
    assert_eq!(m[MatrixCoordinate::new(1, 2)], 5);
    assert_eq!(m[MatrixCoordinate::new(2, 0)], 6);
    assert_eq!(m[MatrixCoordinate::new(2, 1)], 7);
    assert_eq!(m[MatrixCoordinate::new(2, 2)], 8);
}
