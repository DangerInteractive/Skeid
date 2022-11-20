pub trait Cross<Rhs = Self> {
    type Output;

    fn cross(self, rhs: Rhs) -> Self::Output;
}

pub trait CrossAssign<Rhs = Self> {
    fn cross_assign(&mut self, rhs: Rhs);
}
