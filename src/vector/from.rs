use crate::ops::from::From;
use crate::vector::Vector;
use std::array::from_fn;
use std::convert::From as StdFrom;

impl<TFrom, TTo, const ROWS: usize> From<Vector<TFrom, ROWS>> for Vector<TTo, ROWS>
where
    TFrom: Copy,
    TTo: Copy + StdFrom<TFrom>,
{
    fn from(value: Vector<TFrom, ROWS>) -> Self {
        Vector::from_array(from_fn(|i| TTo::from(value.0[i])))
    }
}
