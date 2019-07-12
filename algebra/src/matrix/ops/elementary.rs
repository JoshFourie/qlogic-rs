//! Docs: InProgress, view src.

use crate::matrix;

use matrix::interface;

use std::ops;

impl<T: Copy> interface::ElementaryRow<T, usize> for matrix::Matrix<T>
where
    T: ops::Add<Output=T> 
    + ops::Mul<Output=T> 
    + num::One
{
    type Output = Self;

    #[inline]
    fn row_swap(mut self, r1: usize, r2: usize) -> Self::Output {
        (&mut self).row_swap(r1, r2);
        self
    }

    #[inline]
    fn row_add(mut self, scalar: Option<T>, lhs: usize, rhs: usize) -> Self::Output {
        (&mut self).row_add(scalar, lhs, rhs);
        self
    }

    #[inline]
    fn row_mul(mut self, scal: T, r: usize) -> Self::Output {
        (&mut self).row_mul(scal, r);
        self
    }
   
}

impl<'a, T: Copy> interface::ElementaryRow<T,usize> for &'a matrix::Matrix<T>
where
    T: ops::Add<Output=T> 
    + ops::Mul<Output=T> 
    + num::One
{
    type Output = crate::matrix::Matrix<T>;

    fn row_swap(self, r1: usize, r2: usize) -> Self::Output
    {
        let mut mat: crate::matrix::Matrix<T> = self.clone();
        mat[r1][..self.col].clone_from_slice(&self[r2][..self.col]);
        mat[r2][..self.col].clone_from_slice(&self[r1][..self.col]);
        mat
    }

    fn row_add(self, scalar: Option<T>, lhs: usize, rhs: usize) -> Self::Output
    {
        let mut mat: crate::matrix::Matrix<T> = self.clone();
        let scal: T = match scalar {
            Some(s) => s,
            None => T::one()
        };
        for c in 0..self.col {
            mat[lhs][c] = scal * self[lhs][c] + self[rhs][c];
        }
        mat
    }

    fn row_mul(self, scal: T, r: usize) -> Self::Output
    {
        let mut mat: crate::matrix::Matrix<T> = self.clone();
        for c in 0..self.col {
            mat[r][c] = scal * mat[r][c];   
        }
        mat
    }
}


impl<'a, T: Copy> interface::ElementaryRow<T, usize> for &'a mut matrix::Matrix<T>
where
    T: ops::Add<Output=T> 
    + ops::Mul<Output=T> 
    + num::One
{
    type Output = ();

    fn row_swap(mut self, r1: usize, r2: usize) -> Self::Output
    {
        for col in 0..self.col {
            let mem_cpy_item: T = self[r1][col];
            self[r1][col] = std::mem::replace(&mut self[r2][col], mem_cpy_item);
        }
    }

    fn row_add(mut self, scalar: Option<T>, lhs: usize, rhs: usize) -> Self::Output
    {
        let scal: T = match scalar {
            Some(s) => s,
            None => T::one()
        };
        for c in 0..self.col {
            self[lhs][c] = scal * self[lhs][c] + self[rhs][c];
        }
    }

    fn row_mul(mut self, scal: T, r: usize) -> Self::Output
    {
        for c in 0..self.col {
            self[r][c] = scal * self[r][c];   
        }
    }
}

#[cfg(test)] mod tests {

    use crate::matrix;

    use matrix::Matrix;

    use matrix::interface::ElementaryRow;

    #[test] fn test_elementary_row_swap_for_matrix()
    {
        let T: Matrix<_> = Matrix {
            inner: vec![
                1.0,3.0,
                5.0,7.0,
                9.0,11.0
            ],
            row: 3,
            col: 2
        };
        let T1: _ = T.row_swap(0,1);
        let E1: Matrix<_> = Matrix {
            inner: vec![
                5.0,7.0,
                1.0,3.0,
                9.0,11.0
            ],
            row: 3,
            col: 2
        };
        assert_eq!(T1,E1);
    }

    #[test] fn test_elementary_row_add_for_matrix()
    {
        let T: Matrix<_> = Matrix {
            inner: vec![
                1.0,3.0,
                5.0,7.0,
                9.0,11.0
            ],
            row: 3,
            col: 2
        };
        let T1: _ = (&T).row_add(None, 0,1);
        let E1: Matrix<_> = Matrix {
            inner: vec![
                6.0,10.0,
                5.0,7.0,
                9.0,11.0
            ],
            row: 3,
            col: 2
        };
        assert_eq!(E1, T1);
        let T2: _ = T.row_add(Some(2.0), 0, 1);
        let E2: Matrix<_> = Matrix {
            inner: vec![
                7.0,13.0,
                5.0,7.0,
                9.0,11.0
            ],
            row: 3,
            col: 2
        };
        assert_eq!(E2, T2);
    }

    #[test] fn test_elemntary_row_mul_for_matrix()
    {
        let T: Matrix<_> = Matrix {
            inner: vec![
                1.0,3.0,
                5.0,7.0,
                9.0,11.0
            ],
            row: 3,
            col: 2
        };
        let T1: _ = (&T).row_mul(2.0, 2);
        let E1: Matrix<_> = Matrix {
            inner: vec![
                1.0,3.0,
                5.0,7.0,
                18.0,22.0
            ],
            row: 3,
            col: 2
        };   
        assert_eq!(E1, T1);
    }
}