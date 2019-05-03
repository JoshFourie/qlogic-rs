/***** Imports ******/
use super::Matrix;
use num::{One, Zero};
use std::ops::{Add, Mul, Rem, Sub, Div};
use crate::linear_algebra::interface::*;

/***** Implements *****/

macro_rules! impl_transpose_for_matrix 
{    
    ($id:ty) => {
        impl<'a, T: Copy> Transpose for $id 
        {
            type Output = Matrix<T>;
            
            fn transpose(self) -> Self::Output {
                let mut C: Vec<T> = Vec::new();
                let (r,c): (usize,usize) = (self.row, self.col);
                for i in 0..r {
                    for j in 0..c {
                        C.push(self[j][i])
                    }
                }
                Matrix {
                    inner: C,
                    row: r,
                    col: c
                }
            }    
        }
    }
}

impl_transpose_for_matrix!(Matrix<T>);
impl_transpose_for_matrix!(&'a Matrix<T>);
impl_transpose_for_matrix!(&'a mut Matrix<T>);

macro_rules! impl_diagonal_for_matrix 
{
    ($id:ty) => {
        impl<'a, T: Copy> Diagonal<T> for $id
        where   
            T: Zero + Add<Output=T>
        {
            type Output = Vec<T>;

            fn diagonal(self) -> Self::Output
            {
                let mut diag: Vec<T> = Vec::new();
                for i in 0..self.col {
                    diag.push(self[i][i])
                }
                diag
            }

            fn trace(self) -> T
            {
                self.diagonal()
                    .into_iter()
                    .fold(T::zero(), |acc, x| acc + x)
            }
        }
    }
}

impl_diagonal_for_matrix!(Matrix<T>);
impl_diagonal_for_matrix!(&'a Matrix<T>);
impl_diagonal_for_matrix!(&'a mut Matrix<T>);

macro_rules! impl_kronecker_for_matrix 
{
    ($id:ty) => {
        impl<'a, T: Copy> Kronecker<$id> for $id
        where   
            T: Zero + Mul<Output=T>
        {
            type Output = Matrix<T>;

            fn kronecker(self, rhs: $id) -> Self::Output
            {
                let m = self.row;
                let p = rhs.row;
                let n = self.col;
                let q = rhs.col;

                let mut N: Matrix<T> = Matrix {
                    inner: vec![T::zero(); m*n*p*q],
                    row: m*p,
                    col: n*q
                };

                for i in 1..=m*p as usize {
                    for j in 1..=n*q as usize 
                    {
                        let i = i as f64;
                        let j = j as f64;

                        let a1 = i.sub(1.0).div(p as f64).floor().add(1.0) as usize - 1;
                        let a2 = j.sub(1.0).div(q as f64).floor().add(1.0) as usize - 1;
                        let b1 = i.sub(1.0).rem(p as f64).add(1.0) as usize - 1;
                        let b2 = j.sub(1.0).rem(q as f64).add(1.0) as usize - 1;
                        
                        let alpha = self[a1][a2];
                        let beta = rhs[b1][b2];
                        let delta = alpha*beta;
                        N[i as usize - 1][j as usize -1] = delta;
                    }
                }
                N
            }
        }
    }
}

impl_kronecker_for_matrix!(Matrix<T>);
impl_kronecker_for_matrix!(&'a Matrix<T>);
impl_kronecker_for_matrix!(&'a mut Matrix<T>);

macro_rules! impl_elem_row_operations
{
    ($id:ty) => 
    {
        impl<'a, T: Copy> ElementaryRowOperations<T> for $id
        where
            T: Add<Output=T> + Mul<Output=T> + One
        {
            type Output = Matrix<T>;
    
            fn row_swap(self, r1: usize, r2: usize) -> Self::Output
            {
                let mut mat: Matrix<T> = self.clone();
                for c in 0..self.col {
                    mat[r1][c] = self[r2][c];
                    mat[r2][c] = self[r1][c];
                }
                mat
            }

            fn row_add(self, scalar: Option<T>, lhs: usize, rhs: usize) -> Self::Output
            {
                let mut mat: Matrix<T> = self.clone();
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
                let mut mat: Matrix<T> = self.clone();
                for c in 0..self.col {
                    mat[r][c] = scal * mat[r][c];   
                }
                mat
            }
        }
    }
}

impl_elem_row_operations!(Matrix<T>);
impl_elem_row_operations!(&'a Matrix<T>);
impl_elem_row_operations!(&'a mut Matrix<T>);