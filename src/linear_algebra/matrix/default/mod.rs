#![allow(non_snake_case)]

/******** TODO ******
 * column extraction by indexing: M[None][3].
********************/

/**** Imports *****/

use std::ops::{Mul, Add, Sub, Index, IndexMut, Range};
use super::{LinearAlgebraError, ErrorKind};
use num_integer::Roots;
use num::{One, Zero};

/**** Exports *****/

mod iter;
mod algebra;
#[cfg(test)] mod tests;

/**** Structs ******/

#[derive(Default, Debug, PartialEq, Clone)]
pub struct Matrix<T>
{
    inner: Vec<T>,
    row: usize,
    col: usize
}

/********** From **********/

impl<T> From<Vec<T>> for Matrix<T> 
{
    fn from(v: Vec<T>) -> Self {
        let l: usize = v.len().sqrt();
        Matrix {
            inner: v,
            row: l,
            col: l,
        }
    }
}

/********** Into **********/

impl<T> Into<Vec<T>> for Matrix<T> 
{
    fn into(self) -> Vec<T> { 
        self.inner 
    } 
}

macro_rules! impl_into_vec_for_matrix {
    ($id:ty) => {
        impl<'a, T:Copy> Into<Vec<T>> for $id 
        { 
            fn into(self) -> Vec<T> { 
                self.inner.clone() 
            } 
        }
    }
}

impl_into_vec_for_matrix!(&'a Matrix<T>);
impl_into_vec_for_matrix!(&'a mut Matrix<T>);

/******* Index & IndexMut ********/

impl<T> Index<(usize,usize)> for Matrix<T>
{
    type Output = T;

    fn index<'a>(&'a self, idx: (usize,usize)) -> &'a T {
        let (r,c): (usize,usize) = idx;
        let i = r.mul(self.col).add(c);
        &self.inner[i]
    }
}

impl<T> IndexMut<(usize,usize)> for Matrix<T>
{
    fn index_mut<'a>(&'a mut self, idx:(usize,usize)) -> &'a mut T {
        let (r,c): (usize,usize) = idx;
        let i = r.mul(self.col).add(c);
        &mut self.inner[i]   
    }    
}

// row extraction by indexing.
impl<T> Index<usize> for Matrix<T>
{
    type Output = [T];

    fn index<'a>(&'a self,idx:usize) -> &'a Self::Output {
        let i0: usize = idx.mul(self.col);
        let ter: usize = i0.add(self.col);
        let i: Range<usize> = i0..ter;
        &self.inner[i]
    }
}

impl<T> IndexMut<usize> for Matrix<T>
{
    fn index_mut<'a>(&'a mut self, idx:usize) -> &'a mut Self::Output {
        let i0: usize = idx.mul(self.col);
        let ter: usize = i0.add(self.col);
        let i: Range<usize> = i0..ter;
        &mut self.inner[i]
    }
}

/******** Unchecked Standard Operations ***********/
pub trait CheckedMul<RHS> {

    type Output;

    fn checked_mul(self, rhs: RHS) -> Self::Output;

}

macro_rules! impl_matrix_mul {
    ($id:ty) => {
        impl<'a, T: Copy> Mul<T> for $id
        where
            T: Mul<Output=T>,
        {
            type Output = Matrix<T>;

            fn mul(self, rhs: T) -> Matrix<T> { 
                let inner: Vec<T> = self.into_iter()
                    .map(|l| l * rhs)
                    .collect();
                let l: usize = inner.len();
                Matrix {
                    inner: inner,
                    row: l,
                    col: l
                }
            }
        }
    };
    ($lhs:ty, $rhs:ty) => {
        impl<'a, T: Copy> Mul<$rhs> for $lhs
        where
            T: Mul<Output=T> + Zero
        {
            type Output = Matrix<T>;

            fn mul(self, rhs: $rhs) -> Self::Output {
                let mut C: Vec<T> = Vec::new();
                for i in 0..self.row {
                    for j in 0..rhs.col {
                        let mut sigma: T = T::zero();
                        for k in 0..rhs.row
                        {
                            sigma = sigma + self[i][k] * rhs[k][j];
                        }
                        C.push(sigma);
                    }
                }
                Matrix {
                    inner: C,
                    row: self.row,
                    col: self.col
                }
            }
        }
    
        impl<'a, T: Copy> CheckedMul<$rhs> for $lhs
        where
            T: Mul<Output=T> + Zero
        {
            type Output = Result<Matrix<T>, LinearAlgebraError>;

            fn checked_mul(self, rhs: $rhs) -> Self::Output 
            {
                if self.col != rhs.row {
                    Err(LinearAlgebraError::from(ErrorKind::MatrixStructure))
                } else { 
                    Ok(self * rhs)
                }
            }
        } 
    }
}

impl_matrix_mul!(Matrix<T>);
impl_matrix_mul!(&'a Matrix<T>);
impl_matrix_mul!(&'a mut Matrix<T>);
impl_matrix_mul!(Matrix<T>, Matrix<T>);
impl_matrix_mul!(Matrix<T>, &'a Matrix<T>);
impl_matrix_mul!(&'a Matrix<T>, Matrix<T>);
impl_matrix_mul!(&'a Matrix<T>, &'a Matrix<T>);
impl_matrix_mul!(Matrix<T>, &'a mut Matrix<T>);
impl_matrix_mul!(&'a mut Matrix<T>, Matrix<T>);
impl_matrix_mul!(&'a mut Matrix<T>, &'a mut Matrix<T>);

pub trait CheckedAdd<RHS> {

    type Output;
    
    fn checked_add(self, rhs: RHS) -> Self::Output;

}

pub trait CheckedSub<RHS> {

    type Output;

    fn checked_sub(self, rhs: RHS) -> Self::Output;

}

macro_rules! impl_matrix_add_or_sub {
    ($lhs:ty, $rhs:ty, $unchecked:ident, $func:ident, $checked:ident, $checked_func:ident) => {
        impl<'a, T: Copy> $unchecked<$rhs> for $lhs 
        where
            T: $unchecked<T,Output=T>
        {
            type Output = Matrix<T>;

            fn $func(self, rhs: $rhs) -> Self::Output {
                let (r,c): (usize,usize) = (self.row,self.col);
                let C: Vec<T> = self.into_iter()
                    .zip(rhs.into_iter())
                    .map(|(l,r)| l.$func(r))
                    .collect();
                Matrix {
                    inner: C,
                    row: r,
                    col: c
                }
            }
        }  

        impl<'a, T: Copy> $checked<$rhs> for $lhs 
        where
            T: $unchecked<T,Output=T>
        {
            type Output = Result<Matrix<T>, LinearAlgebraError>;

            fn $checked_func(self, rhs: $rhs) -> Self::Output {
                if self.col == rhs.col && self.row == self.col {
                    Ok(self.$func(rhs))
                } else { 
                    Err(LinearAlgebraError::from(ErrorKind::MatrixStructure))
                }
            }
        }
    } 
}

impl_matrix_add_or_sub!(Matrix<T>, Matrix<T>, Add, add, CheckedAdd, checked_add);
impl_matrix_add_or_sub!(&'a Matrix<T>, Matrix<T>, Add, add, CheckedAdd, checked_add);
impl_matrix_add_or_sub!(Matrix<T>, &'a Matrix<T>, Add, add, CheckedAdd, checked_add);
impl_matrix_add_or_sub!(&'a Matrix<T>, &'a Matrix<T>, Add, add, CheckedAdd, checked_add);

impl_matrix_add_or_sub!(Matrix<T>, Matrix<T>, Sub, sub, CheckedSub, checked_sub);
impl_matrix_add_or_sub!(&'a Matrix<T>, Matrix<T>, Sub, sub, CheckedSub, checked_sub);
impl_matrix_add_or_sub!(Matrix<T>, &'a Matrix<T>, Sub, sub, CheckedSub, checked_sub);
impl_matrix_add_or_sub!(&'a Matrix<T>, &'a Matrix<T>, Sub, sub, CheckedSub, checked_sub);

impl_matrix_add_or_sub!(&'a mut Matrix<T>, Matrix<T>, Add, add, CheckedAdd, checked_add);
impl_matrix_add_or_sub!(Matrix<T>, &'a mut Matrix<T>, Add, add, CheckedAdd, checked_add);
impl_matrix_add_or_sub!(&'a mut Matrix<T>, &'a mut Matrix<T>, Add, add, CheckedAdd, checked_add);

impl_matrix_add_or_sub!(&'a mut Matrix<T>, Matrix<T>, Sub, sub, CheckedSub, checked_sub);
impl_matrix_add_or_sub!(Matrix<T>, &'a mut Matrix<T>, Sub, sub, CheckedSub, checked_sub);
impl_matrix_add_or_sub!(&'a mut Matrix<T>, &'a mut Matrix<T>, Sub, sub, CheckedSub, checked_sub);


/******* MULTIPLICATIVE IDENTITY ********/
pub trait Identity {
    type Output;
    fn identity(self) -> Self::Output;
}

macro_rules! impl_matrix_identity {
    ($s:ty) => {
        impl<'a, T: Clone> Identity for $s 
        where
            T: Zero + One
        {
            type Output = Matrix<T>;

            fn identity(self) -> Self::Output {
                let mut I: Matrix<T> = Matrix {
                    inner: vec![T::zero(); self.row * self.col],
                    row: self.row,
                    col: self.col
                };
                for i in 0..self.row {
                    I[i][i] = T::one();
                }
                I
            }
        }
    }
}

impl_matrix_identity!(Matrix<T>);
impl_matrix_identity!(&'a Matrix<T>);
impl_matrix_identity!(&'a mut Matrix<T>);