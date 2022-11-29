use crate::matrix::Matrix;
use crate::ops::componentwise::{AssignComponentwiseOp, ComponentwiseOp};
use std::array::from_fn;

impl<Component, Input, OutputComponent, const ROWS: usize, const COLUMNS: usize>
    ComponentwiseOp<Component, Input, Input, OutputComponent> for Matrix<Component, ROWS, COLUMNS>
where
    Component: Copy,
    Input: Copy,
    OutputComponent: Copy,
{
    type Output = Matrix<OutputComponent, ROWS, COLUMNS>;

    fn componentwise_op(
        self,
        input: Input,
        op: fn(Component, Input) -> OutputComponent,
    ) -> Self::Output {
        Matrix::from_array(from_fn(move |column| {
            from_fn(move |row| op(self[(column, row)], input))
        }))
    }
}

impl<Component, Input, const ROWS: usize, const COLUMNS: usize>
    AssignComponentwiseOp<Component, Input, Input> for Matrix<Component, ROWS, COLUMNS>
where
    Component: Copy,
    Input: Copy,
{
    fn assign_componentwise_op(&mut self, input: Input, op: fn(&mut Component, Input)) {
        for column in 0..COLUMNS {
            for row in 0..ROWS {
                op(&mut self[(column, row)], input);
            }
        }
    }
}
