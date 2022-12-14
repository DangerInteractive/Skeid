use crate::ops::componentwise::{AssignComponentwiseOp, ComponentwiseOp};
use crate::vector::Vector;
use std::array::from_fn;

/// Operate on two vectors of the same dimensions
impl<Component, InputComponent, OutputComponent, const ROWS: usize>
    ComponentwiseOp<Component, Vector<InputComponent, ROWS>, InputComponent, OutputComponent>
    for Vector<Component, ROWS>
where
    Component: Copy,
    InputComponent: Copy,
    OutputComponent: Copy,
{
    type Output = Vector<OutputComponent, ROWS>;

    fn componentwise_op(
        self,
        input: Vector<InputComponent, ROWS>,
        op: fn(Component, InputComponent) -> OutputComponent,
    ) -> Self::Output {
        Vector::from_array(from_fn(move |row| op(self[row], input[row])))
    }
}

/// Operate on two vectors of the same dimensions, in place on left-hand-side vector
impl<Component, InputComponent, const ROWS: usize>
    AssignComponentwiseOp<Component, Vector<InputComponent, ROWS>, InputComponent>
    for Vector<Component, ROWS>
where
    Component: Copy,
    InputComponent: Copy,
{
    fn assign_componentwise_op(
        &mut self,
        input: Vector<InputComponent, ROWS>,
        op: fn(&mut Component, InputComponent),
    ) {
        for row in 0..ROWS {
            op(&mut self[row], input[row]);
        }
    }
}
