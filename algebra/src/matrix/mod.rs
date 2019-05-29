#![allow(non_snake_case)]

use num_integer::Roots;

mod macro_core;

mod iter;

#[cfg(test)] mod tests;

/// A structure representing a Matrix.
#[derive(Default, Debug, PartialEq, Clone)]
pub struct Matrix<T>
{
    inner: Vec<T>,
    pub row: usize,
    pub col: usize
}

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

impl<T> Into<Vec<T>> for Matrix<T> 
{
    fn into(self) -> Vec<T> { 
        self.inner 
    } 
}

impl<'a, T> std::ops::Index<usize> for &'a crate::matrix::Matrix<T>
{
    type Output = [T];

    fn index<'b>(&'b self,idx:usize) -> &'b Self::Output {
        let i0: usize = idx * self.col;
        let ter: usize = i0 + self.col;
        let i: std::ops::Range<usize> = i0..ter;
        &self.inner[i]
    }
}