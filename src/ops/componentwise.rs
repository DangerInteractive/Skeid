pub trait ComponentwiseOp<
    Component,
    Input = Component,
    InputComponent = Input,
    OutputComponent = Component,
>
{
    type Output;

    fn componentwise_op(
        self,
        input: Input,
        op: fn(Component, InputComponent) -> OutputComponent,
    ) -> Self::Output;
}

pub trait AssignComponentwiseOp<Component, Input = Component, InputComponent = Input> {
    fn assign_componentwise_op(&mut self, input: Input, op: fn(&mut Component, InputComponent));
}
