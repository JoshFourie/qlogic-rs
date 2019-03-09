use std::iter::IntoIterator;
use super::matrix::Matrix;
use std::ops::{ AddAssign, Mul };
use num::Zero;

pub trait Vectors<T> { }

impl<T> Vectors<T> for Vector<T> { }

// TODO: impl ErrorHandling
#[derive(Debug)]
pub enum VectorErr
{
    Multiplication(VectorMulErr)
} 

#[derive(Debug)]
pub enum VectorMulErr
{

}

#[derive(Debug, PartialEq)]
pub struct Vector<T>
{
    pub inner: Vec<T>
}

impl<T> IntoIterator for Vector<T>
{
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter
    {
        self.inner.into_iter()
    }
} 

impl<T> Mul<Matrix<T>> for Vector<T>
where
    for <'c>
    &'c T: Mul<&'c T,Output=T>, 
    T: Zero
    + AddAssign
{
    type Output=Result<Self,VectorErr>;

    fn mul(self, rhs: Matrix<T>) -> Self::Output
    {
        let mut c = Vector::new_empty();
        for i in 0..rhs.dim
        {
            let mut val = T::zero();
            for k in 0..rhs.dim
            {
                val += &self.inner[k] * rhs.inner.get( &(i, k) ).unwrap();
            }
            c.inner.push(val);
        }
        Ok(c)
    }
}

impl<T> Vector<T>
{
    pub fn new_empty() -> Self { Self{inner: Vec::new()} }

    pub fn new_with_inner(inner: Vec<T>) -> Self { Self{ inner } }
}

#[cfg(test)]
mod tests
{
    use super::super::matrix::Matrix;
    use super::*;

    #[test]
    fn test_matrix_dot_vector()
    {
        let inner = vec![ 1.0, 2.0, 3.0, 4.0, 1.0, 2.0, 3.0, 4.0, 1.0, 2.0, 3.0, 4.0, 1.0, 2.0, 3.0, 4.0 ];
        let A = Matrix::new_with_inner(inner);

        let inner = vec![ 1.0, 2.0, 3.0, 4.0 ];
        let B = Vector::new_with_inner(inner);

        let inner = vec![ 30.0, 30.0, 30.0, 30.0 ];
        let exp = Vector::new_with_inner(inner);

        assert_eq!((B*A).unwrap(), exp);
    }
}