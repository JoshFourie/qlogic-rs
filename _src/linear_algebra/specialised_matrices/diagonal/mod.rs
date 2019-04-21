/******** Imports ********/
use num::Num;
use std::fmt::Debug;
use crate::linear_algebra::*;
use crate::linear_algebra::matrix_iter::MatrixIter;
use crate::linear_algebra::matrix_err::MathError;
use crate::linear_algebra::square::*;

/******* Exports ******/
pub mod tridiagonal;
// #[cfg(test)] mod test;

pub trait Tridiagonal<T: Num + Copy + Debug>: Sized
where
    for <'a> &'a Self: IntoIterator<Item=T>,
    Self: From<Matrix<T>> + Into<Matrix<T>>,
{
    
}

/******** Notes for Dev. ************
 *  Only implementing for squares at the moment.
 *  Intention is to use tridiagonal matrix for specialisation.
 *  Optimise for QR algorithm etc.
 *  Implemented: CoreMatrix, BasicTransform, Square and Tridiagonal
 * TODO: Optimisations.
************************************/
