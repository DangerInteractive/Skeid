use crate::ops::componentwise::{AssignComponentwise, Componentwise};
use crate::vector::Vector;
use std::array::from_fn;

impl<Component, Input, OutputComponent, const ROWS: usize>
    Componentwise<Component, Input, Input, OutputComponent> for Vector<Component, ROWS>
where
    Component: Copy,
    Input: Copy,
    OutputComponent: Copy,
{
    type Output = Vector<OutputComponent, ROWS>;

    fn componentwise<Op: FnMut(Component, Input) -> OutputComponent>(
        self,
        input: Input,
        op: Op,
    ) -> Self::Output {
        Vector::from_array(from_fn(|row| op(self[row], input)))
    }
}

impl<Component, Input, const ROWS: usize> AssignComponentwise<Component, Input, Input>
    for Vector<Component, ROWS>
where
    Component: Copy,
    Input: Copy,
{
    fn assign_componentwise<Op: FnMut(&mut Component, Input)>(&mut self, input: Input, op: Op) {
        for row in 0..ROWS {
            op(&mut self[row], input);
        }
    }
}
