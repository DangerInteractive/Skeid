pub trait Componentwise<
    Component,
    Input = Component,
    InputComponent = Input,
    OutputComponent = Component,
>
{
    type Output;

    fn componentwise(
        self,
        input: Input,
        op: fn(Component, InputComponent) -> OutputComponent,
    ) -> Self::Output;
}

pub trait AssignComponentwise<Component, Input = Component, InputComponent = Input> {
    fn assign_componentwise(&mut self, input: Input, op: fn(&mut Component, InputComponent));
}
