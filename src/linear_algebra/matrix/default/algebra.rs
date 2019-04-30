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

macro_rules! impl_eucl_norm_for_matrix 
{
    ($id:ty) => {
        impl<'a, T: Copy> Norm<T> for $id 
        where
            T: Zero + One + Add<Output=T> 
        {
            fn eucl_norm(self) -> T
            { 
                self.into_iter()
                    .fold(T::zero(), |acc, x| acc + num::pow(x, 2))
            }
        }
    }
}

impl_eucl_norm_for_matrix!(Matrix<T>);
impl_eucl_norm_for_matrix!(&'a Matrix<T>);
impl_eucl_norm_for_matrix!(&'a mut Matrix<T>);

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
