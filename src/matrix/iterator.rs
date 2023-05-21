use crate::matrix::{Matrix, MatrixCoordinate};

impl<T, const ROWS: usize, const COLUMNS: usize> IntoIterator for Matrix<T, ROWS, COLUMNS>
where
    T: Copy,
{
    type Item = T;
    type IntoIter = MatrixIterator<T, ROWS, COLUMNS>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            matrix: self,
            coordinate: MatrixCoordinate::new(0, 0),
        }
    }
}

#[derive(Debug)]
pub struct MatrixIterator<T: Copy, const ROWS: usize, const COLUMNS: usize> {
    matrix: Matrix<T, ROWS, COLUMNS>,
    coordinate: MatrixCoordinate,
}

impl<T, const ROWS: usize, const COLUMNS: usize> Iterator for MatrixIterator<T, ROWS, COLUMNS>
where
    T: Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let value = match self.coordinate {
            coordinate if coordinate.row < ROWS && coordinate.column < COLUMNS => {
                self.matrix[coordinate]
            }
            _ => return None,
        };
        self.coordinate = self.coordinate.increment_by_row(ROWS);
        Some(value)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let cells = ROWS * COLUMNS;
        (cells, Some(cells))
    }
}

impl<T, const ROWS: usize, const COLUMNS: usize> ExactSizeIterator
    for MatrixIterator<T, ROWS, COLUMNS>
where
    T: Copy,
{
}

impl<'a, T, const ROWS: usize, const COLUMNS: usize> IntoIterator for &'a Matrix<T, ROWS, COLUMNS>
where
    T: Copy,
{
    type Item = &'a T;
    type IntoIter = MatrixRefIterator<'a, T, ROWS, COLUMNS>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            matrix: self,
            coordinate: MatrixCoordinate::new(0, 0),
        }
    }
}

#[derive(Debug)]
pub struct MatrixRefIterator<'a, T: Copy, const ROWS: usize, const COLUMNS: usize> {
    matrix: &'a Matrix<T, ROWS, COLUMNS>,
    coordinate: MatrixCoordinate,
}

impl<'a, T, const ROWS: usize, const COLUMNS: usize> Iterator
    for MatrixRefIterator<'a, T, ROWS, COLUMNS>
where
    T: Copy,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let value = match self.coordinate {
            coordinate if coordinate.row < ROWS && coordinate.column < COLUMNS => {
                &self.matrix[coordinate]
            }
            _ => return None,
        };
        self.coordinate = self.coordinate.increment_by_row(ROWS);
        Some(value)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let cells = ROWS * COLUMNS;
        (cells, Some(cells))
    }
}

impl<T, const ROWS: usize, const COLUMNS: usize> ExactSizeIterator
    for MatrixRefIterator<'_, T, ROWS, COLUMNS>
where
    T: Copy,
{
}
