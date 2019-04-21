/******** TODO ******
 * column extraction by indexing: M[None][3].
********************/

/**** Imports *****/
use num_integer::Roots;

/**** Exports *****/
mod index;
mod iter;
mod inter_impl;

/**** Structs ******/
#[derive(PartialEq, Debug,Clone)]
pub struct Matrix<T>
{
    inner: Vec<T>,
    row: usize,
    col: usize
}

// defaults to square matrix
impl<T> From<Vec<T>> for Matrix<T> {
    fn from(v: Vec<T>) -> Self {
        let l: usize = v.len().sqrt();
        Matrix {
            inner: v,
            row: l,
            col: l,
        }
    }
}

impl<T> Into<Vec<T>> for Matrix<T> { fn into(self) -> Vec<T> { self.inner } }

impl<'a, T:Copy> Into<Vec<T>> for &'a Matrix<T> { fn into(self) -> Vec<T> { self.inner.clone() } }