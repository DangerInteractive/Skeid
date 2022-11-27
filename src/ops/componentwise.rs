pub trait ComponentwiseOp<T, Rhs = T, RC = Rhs> {
    type OutputComponent;
    type Output;

    fn componentwise_op(self, rhs: Rhs, op: fn(T, RC) -> Self::OutputComponent) -> Self::Output;
}

pub trait AssignComponentwiseOp<T, Rhs = T, RC = Rhs> {
    fn assign_componentwise_op(&mut self, rhs: Rhs, op: fn(&mut T, RC));
}
