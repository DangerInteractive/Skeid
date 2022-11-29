use crate::matrix::Matrix;
use crate::ops::componentwise::{AssignComponentwiseOp, ComponentwiseOp};
use std::array::from_fn;

/// Operate on two matrices of the same dimensions
impl<Component, InputComponent, OutputComponent, const ROWS: usize, const COLUMNS: usize>
    ComponentwiseOp<
        Component,
        Matrix<InputComponent, ROWS, COLUMNS>,
        InputComponent,
        OutputComponent,
    > for Matrix<Component, ROWS, COLUMNS>
where
    Component: Copy,
    InputComponent: Copy,
    OutputComponent: Copy,
{
    type Output = Matrix<OutputComponent, ROWS, COLUMNS>;

    fn componentwise_op(
        self,
        input: Matrix<InputComponent, ROWS, COLUMNS>,
        op: fn(Component, InputComponent) -> OutputComponent,
    ) -> Self::Output {
        Matrix::from_array(from_fn(move |column| {
            from_fn(move |row| op(self[(column, row)], input[(column, row)]))
        }))
    }
}

/// Operate on two matrices of the same dimensions, in place on left-hand-side matrix
impl<Component, InputComponent, const ROWS: usize, const COLUMNS: usize>
    AssignComponentwiseOp<Component, Matrix<InputComponent, ROWS, COLUMNS>, InputComponent>
    for Matrix<Component, ROWS, COLUMNS>
where
    Component: Copy,
    InputComponent: Copy,
{
    fn assign_componentwise_op(
        &mut self,
        input: Matrix<InputComponent, ROWS, COLUMNS>,
        op: fn(&mut Component, InputComponent),
    ) {
        for column in 0..COLUMNS {
            for row in 0..ROWS {
                op(&mut self[(column, row)], input[(column, row)]);
            }
        }
    }
}
