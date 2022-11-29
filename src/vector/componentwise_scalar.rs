use crate::ops::componentwise::{AssignComponentwiseOp, ComponentwiseOp};
use crate::vector::Vector;
use std::array::from_fn;

impl<Component, Input, OutputComponent, const ROWS: usize>
    ComponentwiseOp<Component, Input, Input, OutputComponent> for Vector<Component, ROWS>
where
    Component: Copy,
    Input: Copy,
    OutputComponent: Copy,
{
    type Output = Vector<OutputComponent, ROWS>;

    fn componentwise_op(
        self,
        input: Input,
        op: fn(Component, Input) -> OutputComponent,
    ) -> Self::Output {
        Vector::from_array(from_fn(move |row| op(self[row], input)))
    }
}

impl<Component, Input, const ROWS: usize> AssignComponentwiseOp<Component, Input, Input>
    for Vector<Component, ROWS>
where
    Component: Copy,
    Input: Copy,
{
    fn assign_componentwise_op(&mut self, input: Input, op: fn(&mut Component, Input)) {
        for row in 0..ROWS {
            op(&mut self[row], input);
        }
    }
}
