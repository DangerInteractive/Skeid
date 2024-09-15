use crate::matrix::Matrix;
use crate::ops::from::From;
use std::convert::From as StdFrom;

impl<TFrom, TTo, const ROWS: usize, const COLUMNS: usize> From<Matrix<TFrom, ROWS, COLUMNS>>
    for Matrix<TTo, ROWS, COLUMNS>
where
    TFrom: Copy,
    TTo: Copy + StdFrom<TFrom>,
{
    fn from(value: Matrix<TFrom, ROWS, COLUMNS>) -> Self {
        Matrix::from_fn(move |coord| TTo::from(value[coord]))
    }
}
