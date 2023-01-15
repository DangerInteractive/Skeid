pub trait Componentwise<
    Component,
    Input = Component,
    InputComponent = Input,
    OutputComponent = Component,
>
{
    type Output;

    fn componentwise<Op: FnMut(Component, InputComponent) -> OutputComponent>(
        self,
        input: Input,
        op: Op,
    ) -> Self::Output;
}

pub trait AssignComponentwise<Component, Input = Component, InputComponent = Input> {
    fn assign_componentwise<Op: FnMut(&mut Component, InputComponent)>(
        &mut self,
        input: Input,
        op: Op,
    );
}
