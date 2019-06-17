#![allow(non_snake_case)]
//! A module containing the Matrix structure, interface and implementations.
//! 
//! The `Matrix` structure is a mono-structure containing all the interface implementations
//! which have been separated for modularity and readibility. Every implementation is
//! contained in the `Interface` module.
//! 
//! It is not a high-performance implementation and aims to approximate answers with an error 
//! bound of 0.0001.
//! 
//! # Using the Module
//! ``` 
//! // import the modules.
//! use crate::algebra::{matrix, vector};
//! 
//! // import any trait interfaces that are needed.
//! use matrix::interface::ERO;
//! 
//! use vector::interface::Length;
//! 
//! // An Operator is a Matrix that acts on a vector representing a quantum bit.
//! #[derive(Debug, PartialEq)] struct Operator<T>(matrix::Matrix<T>); 
//! 
//! #[derive(Debug, PartialEq)] struct QBit<T>(vector::Vector<T>);
//! 
//! impl<T: Copy> Operator<T>
//! where
//!    T: num::Num
//! {
//!     // a CNOT gate flips a bit iff the control bit is set
//!     fn cnot(bit: QBit<T>) -> QBit<T> {
//!         
//!         let len: usize = (&bit.0).len();    
//! 
//!         let mut gate: matrix::Matrix<T> = vec![T::zero(); len*len].into();
//!         
//!         for i in 0..gate.row {
//!             gate[i][i] = T::one()
//!         }
//! 
//!         gate = gate.row_swap(2,3);
//! 
//!         QBit(gate * bit.0)
//!     }
//! }
//! 
//! fn main() 
//! {
//!     let bit: QBit<num::Complex<f64>> = QBit(vector::Vector::from(vec![num::Complex::from(1.0), num::Complex::from(0.0), num::Complex::from(1.0), num::Complex::from(0.0)]));   
//!     
//!     let exp: QBit<num::Complex<f64>> = QBit(vector::Vector::from(vec![num::Complex::from(1.0), num::Complex::from(0.0), num::Complex::from(0.0), num::Complex::from(1.0)]));
//!     
//!     let test: QBit<num::Complex<f64>> = Operator::cnot(bit);
//!  
//!     assert_eq!(exp, test);
//! }
//! ```

use num::integer::Roots;

mod macro_core;

mod matrix_ops;

pub mod interface;

mod iter;

/// A structure representing a Matrix.
/// 
/// The `Matrix` stores the data in a `Vec<T>` structure and 
/// maintains a record of the row and col values. It is the
/// responsibility of a function that constructs or modifies 
/// the `Matrix` to update these values as required.
/// 
/// # Using `Matrix`
/// ```
/// use crate::algebra::matrix;
/// 
/// fn main() {
///     // A constructor using the `.into()` method call. The `row` and `col` 
///     // are set to construct a square matrix such that row = col.
///     let matrix: matrix::Matrix<usize> = vec![0,1,2,3,4,5,6,7,8].into();
/// 
///     // A constructor that is a safer alternative than `.into()`.
///     let matrix: matrix::Matrix<usize> = matrix::Matrix::from(vec![0,1,2,3,4,5,6,7,8]);
/// 
///     // A constructor to set a `Matrix` as non-square.
///     let matrix: matrix::Matrix<usize> = matrix::Matrix::new(vec![0,1,2,3,4,5,6], 3, 2);
/// }
/// ```

#[derive(Default, Debug, PartialEq, Clone)]
pub struct Matrix<T>
{
    inner: Vec<T>,
    pub row: usize,
    pub col: usize
}

impl<T> Matrix<T> 
{
    /// A constructor function for a `Matrix` used for constructing
    /// a non-square structure.
    pub fn new(inner: Vec<T>, row: usize, col: usize) -> Self {
        Self { inner, row, col }
    }
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