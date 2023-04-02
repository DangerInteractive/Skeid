//! operations for interacting with two equivalently-sized
//! and compatibly-shaped collections iteratively in a componentwise manner
//! (applying some operation to one element from both collections at a time,
//! where both elements are in the same relative location in each collection)

/// interact with another type of equivalent size and compatible shape in a componentwise manner
/// (applying some operation to one element from both collections at a time,
/// where both elements are in the same relative location in each collection),
/// returning a new value
pub trait Componentwise<
    Component,
    Input = Component,
    InputComponent = Input,
    OutputComponent = Component,
>
{
    /// the type returned by [Componentwise::componentwise](Componentwise::componentwise)
    type Output;

    /// interact with another type of equivalent size and compatible shape in a componentwise manner
    /// (applying some operation to one element from both collections at a time,
    /// where both elements are in the same relative location in each collection),
    /// returning a new value
    #[must_use]
    fn componentwise<Op: FnMut(Component, InputComponent) -> OutputComponent>(
        self,
        input: Input,
        op: Op,
    ) -> Self::Output;
}

/// interact with another type of equivalent size and compatible shape in a componentwise manner
/// (applying some operation to one element from both collections at a time,
/// where both elements are in the same relative location in each collection),
/// performing the operation in place
pub trait AssignComponentwise<Component, Input = Component, InputComponent = Input> {
    /// interact with another type of equivalent size and compatible shape in a componentwise manner
    /// (applying some operation to one element from both collections at a time,
    /// where both elements are in the same relative location in each collection),
    /// performing the operation in place
    fn assign_componentwise<Op: FnMut(&mut Component, InputComponent)>(
        &mut self,
        input: Input,
        op: Op,
    );
}
