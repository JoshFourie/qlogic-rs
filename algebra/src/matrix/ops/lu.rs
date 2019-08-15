//! Docs: InProgress, view src.

use crate::matrix;
use crate::matrix::interface;
use interface::{Identity, Row, Column};

use crate::vector;
use vector::interface::{Length};

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
    fn lu(self) -> Self::Output {
        LU_Decomposition::new(self).lu().into_tuple()
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

struct LU_Decomposition<T> {
    pi: PermutationIndex,
    mat: matrix::Matrix<T>
}

impl<T: Copy> LU_Decomposition<T> 
where
    T: num::Zero 
    + num::One
    + num::Signed
    + ops::AddAssign<T>
    + PartialOrd<T>
{
    fn new(mat: matrix::Matrix<T>) -> Self {
        Self {
            pi: PermutationIndex::new(mat.col),
            mat
        }
    }

    fn lu(mut self) -> PLU<T> {
        let mut plu: _ = PLU::new(&self.mat);

        let rows: usize = self.mat.row;
        let cols = self.mat.col;

        let mut r: usize = 0;
        for k in 0..cols {

            let piv: usize = Self::pivot((&self.mat).get_col(k).into_iter());

            if self.mat[piv][k] != T::zero() {
                
                self.exchange(&mut plu, k, r, piv);
                self.pi.swap(piv,r);    

                for idx in r+1..rows { 
                    plu.L[idx][r] = self.mat[idx][k] / self.mat[r][k] 
                }

                plu.U[r][k..cols].clone_from_slice(&self.mat[r][k..cols]);

                for row in r..rows {
                    for col in k..cols {
                        self.mat[row][col] = self.mat[row][col] - (plu.L[row][r] * plu.U[r][col])
                    }
                }

                r += 1;
            }
        }

        plu.P = Some(self.pi.into_matrix());
        plu
    }

    fn exchange(&mut self, plu: &mut PLU<T>, k: usize, r: usize, piv: usize) {
        for col in k..self.mat.col {
            let mem_cpy_item: T = self.mat[r][col];
            self.mat[r][col] = std::mem::replace(&mut self.mat[piv][col], mem_cpy_item);
        }

        for col in 0..r {
            let mem_cpy_item: T = plu.L[r][col];
            plu.L[r][col] = std::mem::replace(&mut plu.L[piv][col], mem_cpy_item);
        }
    } 

    fn pivot(col: impl Iterator<Item=T>) -> usize {
        let mut max_idx: usize = 0;
        let mut selected: T = T::zero();

        for (curr_idx, curr_val) in col.enumerate() {
            let buf = curr_val.abs();
            if buf > selected {
                max_idx = curr_idx;
                selected = buf
            }
        }
        max_idx
    }
}

struct PermutationIndex(vector::Vector<usize>);

impl PermutationIndex {
    fn new(dim: usize) -> Self {
        let mut inner: Vec<usize> = Vec::new();
        for i in 0..dim {
            inner.push(i)
        }
        PermutationIndex(vector::Vector::from(inner))
    }

    fn swap(&mut self, i: usize, j: usize) {
        self.0.swap(i, j)
    }

    fn into_matrix<T:Clone>(self) -> matrix::Matrix<T> 
    where
        T: num::Zero 
        + num::One
        + ops::Mul<T,Output=T>
    {
        let len: _ = (&self.0).len();
        let mut buf: _ = matrix::Matrix::from(vec![T::zero(); len * len]);
        for (row, col) in self.0.into_iter().enumerate(){
            buf[row][col] = T::one()
        }   
        buf    
    }
}

struct PLU<T> {
    P: Option<matrix::Matrix<T>>,
    L: matrix::Matrix<T>,
    U: matrix::Matrix<T>,
}

impl<T: Copy> PLU<T> 
where
    T: num::Zero 
    + num::One
    + ops::Mul<T, Output=T>
{
    fn new(mat: &matrix::Matrix<T>) -> Self {
        let L: _ = mat.identity();
        let U: _ = mat * T::zero();

        Self { 
            P: None,
            L,
            U 
        }
    }

    fn into_tuple(self) -> (matrix::Matrix<T>, matrix::Matrix<T>, matrix::Matrix<T>) {
        let unwrapped_P: _ = self.P.unwrap();
        (unwrapped_P, self.L, self.U)
    }
}

#[cfg(test)]
    mod tests {

    use super::*;
    use interface::{LU, ElementaryRow};

    #[test]
    fn test_permutation_vector() {
        let test: vector::Vector<usize> = PermutationIndex::new(3).0;
        let exp: _ = vector::Vector::from(vec![0, 1, 2]);
        assert_eq!(test, exp)
    }

    // Check test vectors are correct.
    #[ignore]
    #[test]
    fn test_pivot() {
        let mut A: matrix::Matrix<f32> = vec![
            1.0, 3.0, 1.0,
            2.0, 2.0, -1.0,
            2.0, -1.0, 0.0
        ].into();

        let p1: usize = LU_Decomposition::pivot(
            (&A).get_col(0).into_iter()
        );
        assert_eq!(p1, 1);

        A = A.row_swap(0,1);
        println!("{:?}", A);
        let p2: usize = LU_Decomposition::pivot(
            A.get_col(1).into_iter()
        );
        assert_eq!(p2, 2);
    }

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
            0.5, 0.0, 0.166_666_67, 1.0 
        ].into();

        let UE: matrix::Matrix<f32> = vec![
            2.0, 2.0, 4.0, 6.0,
            0.0, 0.0, 1.0, 4.0,
            0.0, 0.0, 0.0, -6.0,
            0.0, 0.0, 0.0, 0.0
        ].into();

        let PE: matrix::Matrix<f32> = vec![
            0.0, 1.0, 0.0, 0.0, 
            0.0, 0.0, 1.0, 0.0, 
            0.0, 0.0, 0.0, 1.0, 
            1.0, 0.0, 0.0, 0.0, 
        ].into();

        let (P,L,U) = A.lu(); 

        assert_eq!(LE,L);

        assert_eq!(UE,U);

        assert_eq!(PE,P);
    }

    #[test]
    fn test_permutation_index() {
        
        let A: matrix::Matrix<f32> = vec![
            1.0, 3.0, 1.0,
            2.0, 2.0, -1.0,
            2.0, -1.0, 0.0
        ].into();

        let (P, _, _) = A.lu();
        let exp_P: matrix::Matrix<f32> = vec![
            0.0, 1.0, 0.0,
            0.0, 0.0, 1.0, 
            1.0, 0.0, 0.0, 
        ].into();
        assert_eq!(P, exp_P);

        // Can be run for peace of mind.
        // let exp_L: matrix::Matrix<f32> = vec![
        //     1.0, 0.0, 0.0,
        //     1.0, 1.0, 0.0,
        //     0.5, -2.0/3.0, 1.0
        // ].into();
        // assert_eq!(L, exp_L);


        // let exp_U: matrix::Matrix<f32> = vec![
        //     2.0, 2.0, -1.0,
        //     0.0, -3.0, 1.0,
        //     0.0, 0.0, 13.0/6.0 
        // ].into();
        // assert_eq!(U, exp_U);
    }
}
