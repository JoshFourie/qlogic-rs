//! Docs: InProgress, view src.

use crate::matrix;

use crate::matrix::interface;

use num::integer;

use crate::error;

use std::ops;

/// A trait implementation of [`Kronecker`] that consumes `self` and `rhs`
/// to construct a `Matrix<T>` with updated `inner`,`row` and `col`.
/// The function allocates an additional `Vec<T>` on the heap, but then destructs
/// `self` and `rhs`.
/// 
/// See: `internal` for more implementation details.
/// 
/// `internal`: ../fn._kronecker_internal.html
impl<T: Copy> interface::Kronecker<Self> for matrix::Matrix<T>
where
    T: ops::Mul<Output=T>
{
    type Output = Self;

    fn kronecker(self, rhs: Self) -> Self::Output
    {
        let new_row = self.row * rhs.row;
        let new_col = self.col * rhs.col;

        matrix::Matrix {
            inner: _kronecker_internal(&self, &rhs, new_row, new_col),
            row: new_row,
            col: new_col
        }
    }
}

/// A trait implementation of [`Kronecker`] that constructs a `Matrix<T>` 
/// using a reference to `self` and `rhs` without consuming either object.
/// The function allocates an additional `Vec<T>` on the heap.
/// 
/// See: `internal` for more implementation details.
/// 
/// `internal`: ../fn._kronecker_internal.html
impl<'a, T: Copy> interface::Kronecker<Self> for &'a matrix::Matrix<T>
where
    T: ops::Mul<Output=T>
{
    type Output = matrix::Matrix<T>;

    fn kronecker(self, rhs: Self) -> Self::Output
    {
        let new_row = self.row * rhs.row;
        let new_col = self.col * rhs.col;

        matrix::Matrix {
            inner: _kronecker_internal(self, rhs, new_row, new_col),
            row: new_row,
            col: new_col
        }
    }
}

/// A trait implementation of [`Kronecker`] that replaces the `self` object with
/// the constructed `Matrix<T>` structure. The function allocates a `Vec<T>` on the heap
/// that is assigned to `self.inner`.
/// 
/// See: `internal` for more implementation details.
/// 
/// `internal`: ../fn._kronecker_internal.html
impl<'a, T: Copy> interface::Kronecker<Self> for &'a mut matrix::Matrix<T>
where
    T: ops::Mul<Output=T>
{
    type Output = ();

    fn kronecker(self, rhs: Self)
    {        
        let new_row = self.row * rhs.row;
        let new_col = self.col * rhs.col;

        self.inner = _kronecker_internal(self, rhs, new_row, new_col);
        self.row = new_row;
        self.col = new_col;
    }
}

/// An internal function that applies a `kronecker product` routine.
/// 
/// The definition `c[a][b] = a[i][j] * b[k][l]` from WolframAlpha is 
/// equivalent to the following from Wikipedia:
/// ``` rust.ignore
///     c[i][j] = a[floor((i-1/p))+1][floor((j-1)/q)+1] * b[(i-1)%p+1][(j-1)%q+1]
/// ```
/// This translates to the following when incorporating `zero_indexing`:
/// ```rust.ignore
///     let a0 = integer::div_floor(i, rhs.row);
///     let a1 = integer::div_floor(j, rhs.col);
///     let b0 = i % rhs.row;
///     let b1 = j % rhs.col;
/// ```
/// We can therefore declare that: 
/// ```rust.ignore
///     c[i][j] = a[a0][a1] * b[b0][b1]
/// ```
fn _kronecker_internal<T: Copy>(
    lhs: &matrix::Matrix<T>, 
    rhs: &matrix::Matrix<T>,
    new_row: usize,
    new_col: usize
) -> Vec<T>
where
    T: ops::Mul<Output=T>
{
    let mut buf: Vec<T> = Vec::new();

    for i in 0..new_row {
        for j in 0..new_col
        {
            let a0 = integer::div_floor(i, rhs.row);
            let a1 = integer::div_floor(j, rhs.col);

            let b0 = i % rhs.row;
            let b1 = j % rhs.col;

            buf.push(lhs[a0][a1] * rhs[b0][b1]);
        }  
    }
    buf
}

use interface::Kronecker;

/// A trait implementation of `SafeKronecker` that wraps around a `Kronecker` 
/// implementation. It will check that the `Matrix` structure is well-formed before
/// calling the `.kronecker(/* args */)` method.
/// 
/// # Error
/// The trait will return an `Err` result if the `row` and `col` fields incorrectly 
/// declare the length of the data.
macro_rules! safe_kronecker {
    
    ($id: ty, $result: ty) => {
        impl<'a, T: Copy> interface::SafeKronecker<Self> for $id
        where
            T: ops::Mul<Output=T>
        {
            type Output = interface::Result<$result>;

            #[inline]
            fn safe_kronecker(self, rhs: Self) -> Self::Output
            {
                if !self.inner.len() == self.col * self.row || !rhs.inner.len() == rhs.col * rhs.row {
                    Err(error::AlgebraError::from(error::ErrorKind::MatrixStructure))
                } else {
                    Ok(self.kronecker(rhs))
                }
            }
        }
    }
}

safe_kronecker!(matrix::Matrix<T>, Self);
safe_kronecker!(&'a matrix::Matrix<T>, matrix::Matrix<T>);
safe_kronecker!(&'a mut matrix::Matrix<T>, ());

#[test] fn test_kronecker()
{
    use interface::{SafeKronecker, Kronecker};

    let T1A: matrix::Matrix<_> = matrix::Matrix {
        inner: vec![2.0, 4.0, 6.0, 8.0],
        row: 2,
        col: 2
    };
    let T1B: matrix::Matrix<_> = matrix::Matrix {
        inner: vec![1.0,3.0,5.0,7.0,9.0,11.0],
        row: 3,
        col: 2
    };

    let T1A_ref = &T1A;

    let T1B_ref = &T1B;

    let T1_ref = &T1A_ref.kronecker(T1B_ref);

    let T1_ref_safe = &T1A_ref.safe_kronecker(T1B_ref).unwrap();

    let T1: _ = T1A.kronecker(T1B);

    let E1: matrix::Matrix<_> = matrix::Matrix {
        inner: vec![
            2.0,    6.0,    4.0,    12.0,
            10.0,   14.0,   20.0,   28.0,
            18.0,   22.0,   36.0,   44.0,
            6.0,    18.0,   8.0,    24.0,
            30.0,   42.0,   40.0,   56.0,
            54.0,   66.0,   72.0,   88.0
        ],
        row: 6,
        col: 4
    };

    let E1_ref = &E1;

    assert_eq!(T1_ref, E1_ref);

    assert_eq!(T1_ref_safe, E1_ref);

    assert_eq!(E1, T1);
}