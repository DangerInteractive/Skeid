use crate::matrix::Matrix;
use std::ops::{AddAssign, Mul};

impl<LhsT, RhsT, const LHS_COLUMNS: usize, const LHS_ROWS: usize, const RHS_COLUMNS: usize>
    Mul<Matrix<RhsT, LHS_COLUMNS, RHS_COLUMNS>> for Matrix<LhsT, LHS_ROWS, RHS_COLUMNS>
where
    LhsT: Copy + Mul<RhsT>,
    RhsT: Copy,
    <LhsT as Mul<RhsT>>::Output: AddAssign + Copy + Default,
{
    type Output = Matrix<<LhsT as Mul<RhsT>>::Output, LHS_ROWS, RHS_COLUMNS>;

    fn mul(self, rhs: Matrix<RhsT, LHS_COLUMNS, RHS_COLUMNS>) -> Self::Output {
        Matrix::from_fn(|row, column| {
            let mut sum = Default::default();
            for k in 0..LHS_COLUMNS {
                sum += self[(row, k)] * rhs[(k, column)]
            }
            sum
        })
    }
}
