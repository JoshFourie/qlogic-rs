use crate::matrix;

use matrix::interface;

use std::ops;

/// The diagonal trait for a `.diagonal(self)` and `.trace(self)` method call.
/// See `interface` module for details. It is agnostic to the structure of the matrix.
/// 
/// # Details
/// 
/// ## Diagonal
/// A call on `.diagonal(self)` will `filter` the matrix using an iterator
/// and return a `std::vec::Vec` collection. 
/// 
/// ## Trace
/// A call on  `.trace(self)` will call `.diagonal()` 
/// and return the sum of the collection.
/// 
/// # Error
/// There are no panic or exception conditions for this method.
impl<T> interface::Diagonal<T> for matrix::Matrix<T>
where
    T: ops::Add
    + num::Zero
{
    type Output = Vec<T>;

    fn diagonal(self) -> Self::Output
    {
        let cached_col: usize = self.col;

        let mut tar_idx: usize = 0;
        let mut cur_idx: usize = 0;

        self.inner.into_iter()
            .filter(|_| {
                if cur_idx == tar_idx {
                    tar_idx += cached_col + 1;
                    cur_idx += 1;
                    true
                } else {
                    cur_idx += 1;
                    false
                }
            }).collect()
    }

    #[inline]
    fn trace(self) -> T 
    {
        let mut val: T = T::zero();
        for next_val in self.diagonal()
            .into_iter()
        {
            val = val + next_val;
        }
        val
    }
} 

impl<'a, T: Clone> interface::Diagonal<T> for &'a matrix::Matrix<T>
where 
    T: ops::Add
    + num::Zero
{
    type Output = Vec<T>;

    fn diagonal(self) -> Self::Output
    {
        let cached_col: usize = self.col;
        let cached_row: usize = self.row;

        let cycles: usize = if cached_row > cached_col {
            cached_row
        } else { cached_col };

        let mut buf: Vec<T> = Vec::new();

        for i in 0..cycles
        {            
            buf.push(self[i][i].clone())  
        }

        buf
    }

    #[inline]
    fn trace(self) ->  T
    {
        let mut val: T = T::zero();
        for next_val in self.diagonal()
            .into_iter()
        {
            val = val + next_val;
        }
        val
    }
}

/* mut impl? */

#[test] fn test_diagonal()
{
    use interface::Diagonal;

    let T: matrix::Matrix<_> = matrix::Matrix {
        inner: vec![0,1,2,3,4,5,6,7,8],
        row: 3,
        col: 3
    };

    let E: Vec<_> = vec![0,4,8];

    assert_eq!(&(&T).diagonal(), &E);
    
    assert_eq!(T.diagonal(), E);
}