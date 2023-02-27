use crate::matrix::Matrix;

impl<T, const ROWS: usize, const COLUMNS: usize> Default for Matrix<T, ROWS, COLUMNS>
where
    T: Copy + Default,
{
    fn default() -> Self {
        Matrix::from_array([[Default::default(); ROWS]; COLUMNS])
    }
}
