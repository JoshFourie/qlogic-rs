/***** Declarations ******/
pub mod square;
mod _special;

/******** Imports *********/
use crate::linear_algebra::*;
use crate::linear_algebra::matrix_iter::*;
use crate::linear_algebra::matrix_err::*;
use num::{Num, Float};

#[derive(Debug, PartialEq, Clone)]
pub struct SquareMatrix<T>
{
    pub(crate) inner: Vec<T>,
    pub(crate) dim: Option<usize>,
}

pub trait Square<T: Num + Copy + Debug>
where
    for<'a> &'a Self: IntoIterator<Item=T>,
    Self: CoreMatrix<T> 
    + BasicTransform<T>
    + From<Matrix<T>>
    + Into<Matrix<T>>
{

}

impl<T: Num + Copy + Debug> Square<T> for SquareMatrix<T> { }