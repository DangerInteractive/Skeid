use crate::vector::Vector;

impl<T, const ROWS: usize> IntoIterator for Vector<T, ROWS>
where
    T: Copy,
{
    type Item = T;
    type IntoIter = VectorIterator<T, ROWS>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            vector: self,
            row: 0,
        }
    }
}

#[derive(Debug)]
pub struct VectorIterator<T: Copy, const ROWS: usize> {
    vector: Vector<T, ROWS>,
    row: usize,
}

impl<T, const ROWS: usize> Iterator for VectorIterator<T, ROWS>
where
    T: Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let value = match self.row {
            // TODO: simplify this when exclusive range patterns are stable
            row if row < ROWS => self.vector[self.row],
            _ => return None,
        };
        self.row += 1;
        Some(value)
    }
}

impl<'a, T, const ROWS: usize> IntoIterator for &'a Vector<T, ROWS>
where
    T: Copy,
{
    type Item = &'a T;
    type IntoIter = VectorRefIterator<'a, T, ROWS>;

    fn into_iter(self) -> Self::IntoIter {
        todo!()
    }
}

#[derive(Debug)]
pub struct VectorRefIterator<'a, T: Copy, const ROWS: usize> {
    vector: &'a Vector<T, ROWS>,
    row: usize,
}

impl<'a, T, const ROWS: usize> Iterator for VectorRefIterator<'a, T, ROWS>
where
    T: Copy,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let value = match self.row {
            // TODO: simplify this when exclusive range patterns are stable
            row if row < ROWS => &self.vector[row],
            _ => return None,
        };
        self.row += 1;
        Some(value)
    }
}
