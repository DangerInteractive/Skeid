use crate::matrix::Matrix;
use crate::ops::componentwise::{AssignComponentwise, Componentwise};
use std::array::from_fn;

impl<Component, Input, OutputComponent, const ROWS: usize, const COLUMNS: usize>
    Componentwise<Component, Input, Input, OutputComponent> for Matrix<Component, ROWS, COLUMNS>
where
    Component: Copy,
    Input: Copy,
    OutputComponent: Copy,
{
    type Output = Matrix<OutputComponent, ROWS, COLUMNS>;

    fn componentwise<Op: FnMut(Component, Input) -> OutputComponent>(
        self,
        input: Input,
        op: Op,
    ) -> Self::Output {
        Matrix::from_array(from_fn(|column| {
            from_fn(|row| op(self[(column, row)], input))
        }))
    }
}

impl<Component, Input, const ROWS: usize, const COLUMNS: usize>
    AssignComponentwise<Component, Input, Input> for Matrix<Component, ROWS, COLUMNS>
where
    Component: Copy,
    Input: Copy,
{
    fn assign_componentwise<Op: FnMut(&mut Component, Input)>(&mut self, input: Input, op: Op) {
        for column in 0..COLUMNS {
            for row in 0..ROWS {
                op(&mut self[(column, row)], input);
            }
        }
    }
}
