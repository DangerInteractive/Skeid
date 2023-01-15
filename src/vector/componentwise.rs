use crate::ops::componentwise::{AssignComponentwise, Componentwise};
use crate::vector::Vector;
use std::array::from_fn;

/// Operate on two vectors of the same dimensions
impl<Component, InputComponent, OutputComponent, const ROWS: usize>
    Componentwise<Component, Vector<InputComponent, ROWS>, InputComponent, OutputComponent>
    for Vector<Component, ROWS>
where
    Component: Copy,
    InputComponent: Copy,
    OutputComponent: Copy,
{
    type Output = Vector<OutputComponent, ROWS>;

    fn componentwise<Op: FnMut(Component, InputComponent) -> OutputComponent>(
        self,
        input: Vector<InputComponent, ROWS>,
        mut op: Op,
    ) -> Self::Output {
        Vector::from_array(from_fn(|row| op(self[row], input[row])))
    }
}

/// Operate on two vectors of the same dimensions, in place on left-hand-side vector
impl<Component, InputComponent, const ROWS: usize>
    AssignComponentwise<Component, Vector<InputComponent, ROWS>, InputComponent>
    for Vector<Component, ROWS>
where
    Component: Copy,
    InputComponent: Copy,
{
    fn assign_componentwise<Op: FnMut(&mut Component, InputComponent)>(
        &mut self,
        input: Vector<InputComponent, ROWS>,
        mut op: Op,
    ) {
        for row in 0..ROWS {
            op(&mut self[row], input[row]);
        }
    }
}
