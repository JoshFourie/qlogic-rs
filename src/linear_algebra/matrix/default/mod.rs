/******** TODO ******
 * column extraction by indexing: M[None][3].
********************/

/**** Imports *****/

use std::ops::{Mul, Add, Sub, Index, IndexMut, Range};
use num_integer::Roots;
use num::{Num, One, Zero};

/**** Exports *****/

mod iter;
// mod algebra;
#[cfg(test)] mod tests;

/**** Structs ******/

#[derive(Default, Debug, Clone)]
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

impl<'a, T:Copy> Into<Vec<T>> for &'a Matrix<T> 
{ 
    fn into(self) -> Vec<T> { 
        self.inner.clone() 
    } 
}

/******* PartialEq ********/

impl<T> PartialEq<Self> for Matrix<T>
where
    T: PartialEq<T>
{
    fn eq(&self, rhs: &Self) -> bool 
    {
        self.inner == rhs.inner
    }
}

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
macro_rules! impl_matrix_mul {
    
    ($lhs:ty, $rhs:ty, $out:ty) => {
        impl<'a, T: Copy> Mul<$rhs> for $lhs
        where
            T: Mul<Output=T> + Zero
        {
            type Output = $out;

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
                C.into()
            }
        }
    };
}

impl_matrix_mul!(Matrix<T>, Matrix<T>, Matrix<T>);
impl_matrix_mul!(Matrix<T>, &'a Matrix<T>, Matrix<T>);
impl_matrix_mul!(&'a Matrix<T>, Matrix<T>, Matrix<T>);
impl_matrix_mul!(&'a Matrix<T>, &'a Matrix<T>, Matrix<T>);

macro_rules! impl_matrix_scalar_mul {
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
    }
}

impl_matrix_scalar_mul!(Matrix<T>);
impl_matrix_scalar_mul!(&'a Matrix<T>);


/* 
impl<T: Copy> Mul<Self> for Matrix<T>
where
    T: Zero + Mul<Output=T>
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
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
        C.into()
    }
}

impl<'a, T: Copy> Mul<Self> for &'a Matrix<T>
where
    T: Zero + Mul<Output=T>
{
    type Output = Matrix<T>;

    fn mul(self, rhs: Self) -> Matrix<T> {
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
        C.into()
    }
}
*/

impl<T: Copy> Add<Self> for Matrix<T>
where
    T: Add<Output=T>
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let mut C: Vec<T> = Vec::new();
        for (l,r) in self.into_iter()
            .zip(rhs.into_iter())
        {
            C.push(l + r);
        }
        C.into()
    }
}

impl<T: Copy> Sub<Self> for Matrix<T>
where
    T: Sub<Output=T>
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let mut C: Vec<T> = Vec::new();
        for (l,r) in self.into_iter()
            .zip(rhs.into_iter())
        {
            C.push(l - r);
        }
        C.into()
    }
}

/******* MULTIPLICATIVE IDENTITY ********/
// impl<T> One for Matrix<T>