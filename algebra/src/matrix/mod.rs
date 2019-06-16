#![allow(non_snake_case)]

use num::integer::Roots;

mod macro_core;

mod matrix_ops;

pub mod interface;

mod iter;

// #[cfg(test)] mod tests;

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