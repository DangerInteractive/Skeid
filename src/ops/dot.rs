pub trait Dot<Rhs = Self> {
    type Output;

    fn dot(self, rhs: Rhs) -> Self::Output;
}

pub trait DotAssign<Rhs = Self> {
    fn dot_assign(&mut self, rhs: Rhs);
}
