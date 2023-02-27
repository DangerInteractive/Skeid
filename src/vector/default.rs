use crate::vector::Vector;

impl<T, const ROWS: usize> Default for Vector<T, ROWS>
where
    T: Copy + Default,
{
    fn default() -> Self {
        Vector::from_value(Default::default())
    }
}
