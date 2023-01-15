use crate::matrix::Matrix;
use crate::ops::componentwise::{AssignComponentwise, Componentwise};
use std::array::from_fn;

/// Operate on two matrices of the same dimensions
impl<Component, InputComponent, OutputComponent, const ROWS: usize, const COLUMNS: usize>
    Componentwise<Component, Matrix<InputComponent, ROWS, COLUMNS>, InputComponent, OutputComponent>
    for Matrix<Component, ROWS, COLUMNS>
where
    Component: Copy,
    InputComponent: Copy,
    OutputComponent: Copy,
{
    type Output = Matrix<OutputComponent, ROWS, COLUMNS>;

    fn componentwise<Op: FnMut(Component, InputComponent) -> OutputComponent>(
        self,
        input: Matrix<InputComponent, ROWS, COLUMNS>,
        mut op: Op,
    ) -> Self::Output {
        Matrix::from_array(from_fn(|column| {
            from_fn(|row| op(self[(column, row)], input[(column, row)]))
        }))
    }
}

/// Operate on two matrices of the same dimensions, in place on left-hand-side matrix
impl<Component, InputComponent, const ROWS: usize, const COLUMNS: usize>
    AssignComponentwise<Component, Matrix<InputComponent, ROWS, COLUMNS>, InputComponent>
    for Matrix<Component, ROWS, COLUMNS>
where
    Component: Copy,
    InputComponent: Copy,
{
    fn assign_componentwise<Op: FnMut(&mut Component, InputComponent)>(
        &mut self,
        input: Matrix<InputComponent, ROWS, COLUMNS>,
        mut op: Op,
    ) {
        for column in 0..COLUMNS {
            for row in 0..ROWS {
                op(&mut self[(column, row)], input[(column, row)]);
            }
        }
    }
}
