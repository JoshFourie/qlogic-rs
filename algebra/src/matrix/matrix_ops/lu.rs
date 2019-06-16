use crate::matrix;

use crate::matrix::interface;

use interface::{Identity, Column};

use std::ops;

impl<T: Copy> interface::LU for matrix::Matrix<T>
where
    T: num::Zero 
    + num::One
    + num::Signed
    + ops::AddAssign<T>
    + PartialOrd<T>
{
    type Output = (matrix::Matrix<T>, matrix::Matrix<T>, matrix::Matrix<T>);

    /// The LU algorithm for an owned Matrix structure.
    fn lu(mut self) -> Self::Output
    {
        let cached_rows = self.row;
        let cached_cols = self.col;

        let mut L: Self = (&self).identity();
        let mut U: Self = (&self) * T::zero();

        let mut P: Self = 
        {
            let mut val: T = T::zero();
            let mut inner: Vec<T> = Vec::new();

            for _ in 0..cached_cols {
                val += T::one();
                inner.push(val)
            }

            Self {
                inner: inner,
                row: 1,
                col: cached_cols
            }
        };
        let mut r: usize = 0;

        for k in 0..cached_cols {

            let piv: usize = 
            {
                let mut max_idx: usize = 0;
                let mut max_val: T = T::zero();

                for (curr_idx, curr_val) in (&self).get_col(k)
                    .into_iter()
                    .enumerate()
                {
                    let buf = curr_val.abs();
                    if buf > max_val {
                        max_idx = curr_idx;
                        max_val = buf
                    }
                }
                max_idx
            };

            if self[piv][k] != T::zero() {

                for col in k..cached_cols {
                    let buf: T = self[r][col];
                    self[r][col] = self[piv][col];
                    self[piv][col] = buf;
                }

                for col in 0..r {
                    let buf: T = L[r][col];
                    L[r][col] = L[piv][col];
                    L[piv][col] = buf;
                }
                
                P.inner.swap(piv,r);        

                for idx in r+1..cached_rows { L[idx][r] = self[idx][k] / self[r][k] }

                for idx in k..cached_cols { U[r][idx] = self[r][idx] }

                for row in r..cached_rows {
                    for col in k..cached_cols
                    {
                        self[row][col] = self[row][col] - (L[row][r] * U[r][col])
                    }
                }

                r += 1;
            }
        }

        (P,L,U)
    }
}

impl<'a, T: Copy> interface::LU for &'a matrix::Matrix<T>
where
    T: num::Zero 
    + num::One
    + num::Signed
    + ops::AddAssign<T>
    + PartialOrd<T>
{
    type Output = (matrix::Matrix<T>, matrix::Matrix<T>, matrix::Matrix<T>);

    #[inline]
    fn lu(self) -> Self::Output 
    {  
        self.clone().lu()
    }
}

#[cfg(test)] use interface::LU;

#[test] fn test_lu_decomposition()
{
    let A: matrix::Matrix<f32> = vec![
        1.0, 1.0, 2.0, 2.0, 
        2.0, 2.0, 4.0, 6.0, 
        -1.0, -1.0, -1.0, 1.0, 
        1.0, 1.0, 3.0, 1.0
    ].into();

    let LE: matrix::Matrix<f32> = vec![
        1.0, 0.0, 0.0, 0.0,
        -0.5, 1.0, 0.0, 0.0,
        0.5, 1.0, 1.0, 0.0,
        0.5, 0.0, 0.16666666667, 1.0 
    ].into();

    let UE: matrix::Matrix<f32> = vec![
        2.0, 2.0, 4.0, 6.0,
        0.0, 0.0, 1.0, 4.0,
        0.0, 0.0, 0.0, -6.0,
        0.0, 0.0, 0.0, 0.0
    ].into();

    let (_,L,U) = A.lu(); 

    assert_eq!(LE,L);

    assert_eq!(UE,U);
}

#[ignore]#[test] fn test_lu_decomposition_two()
{
    let A: matrix::Matrix<f32> = vec![
        1.0, -2.0, 3.0,
        2.0, -5.0, 12.0,
        0.0, 2.0, 10.0
    ].into();

    let LE: matrix::Matrix<f32> = vec![
        1.0, 0.0, 0.0,
        2.0, 1.0, 0.0,
        0.0, -2.0, 1.0 
    ].into();

    let UE: matrix::Matrix<f32> = vec![
        1.0, -2.0, 3.0,
        0.0, -1.0, 6.0,
        0.0, 0.0, 2.0
    ].into();

    let (_,L,U) = A.lu(); 

    assert_eq!(LE,L);

    assert_eq!(UE,U);
}