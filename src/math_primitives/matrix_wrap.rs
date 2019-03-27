use super::matrix::Matrix;
use super::matrix_err::MathError;
use super::{ MatrixAlgebra, QuantumReal, QuantumUnit };
use num::integer::Roots;
use std::ops::{ Mul, Add };

#[derive(Debug, PartialEq, Clone)]
pub struct Vector<T>
{
    pub(crate) inner: Vec<T>,
    pub(crate) len: Option<usize>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct SquareMatrix<T>
{
    pub(crate) inner: Vec<T>,
    pub(crate) dim: Option<usize>,
}

impl<T> From<Vector<T>> for Matrix<T>
{
    fn from(v: Vector<T>) -> Self 
    {
        Self {
            col: Some(1),
            row: Some(v.inner.len()),
            dim: Some(v.inner.len()),
            inner: v.inner,   
        }
    }
}

impl<'a, T: Clone> From<&'a Vector<T>> for Matrix<T>
{
    fn from(v: &'a Vector<T>) -> Self
    {
        return Matrix {
            col: Some(1),
            row: Some(v.inner.len()),
            dim: Some(v.inner.len()),
            inner: v.inner.clone(),   
        }
    }
}

// TEMP UNWRAP()
impl<T> From<SquareMatrix<T>> for Matrix<T>
{
    fn from(sq: SquareMatrix<T>) -> Self 
    {
        Self {
            col: sq.dim,
            row: sq.dim,
            dim: Some( sq.dim.unwrap().mul(sq.dim.unwrap()) ),
            inner: sq.inner,   
        }
    }
}

impl<'a, T: Clone> From<&'a SquareMatrix<T>> for Matrix<T>
{
    fn from(sq: &'a SquareMatrix<T>) -> Self 
    {
        Self {
            col: sq.dim,
            row: sq.dim,
            dim: Some( sq.dim.unwrap().mul(sq.dim.unwrap()) ),
            inner: sq.inner.clone(),   
        }
    }
} 

impl<T> From<Vec<T>> for Vector<T>
{
    fn from(v: Vec<T>) -> Self {
        Self {
            len: Some( v.len() ),
            inner: v
        }
    }
}

impl<T> From<Vec<T>> for SquareMatrix<T>
{
    fn from(v: Vec<T>) -> Self {
        Self {
            dim: Some( v.len().sqrt() ),
            inner: v
        }
    }
}

impl<T: QuantumUnit> MatrixAlgebra<T> for Vector<T>
{
    type Error = MathError;

    fn dim(&self) -> Option<usize> { self.len }

    fn col_dim(&self) -> Option<usize> { Some(1) }

    fn row_dim(&self) -> Option<usize> { self.len }

    // Should probably be an Option on the col.
    fn update(self, row: Option<usize>, col: Option<usize>) -> Result<Self,Self::Error>
    { 
        match (row, col) {
            (None, None) => Ok( self.inner.into() ),
            _ => MathError::bad_op("Invalid update: cannot over-ride the auto-generated dimensions of a Vector").as_result()
        }
    }

    fn into_inner(&self) -> Vec<T> { self.inner.clone() }

    fn push(&mut self, val: T) { self.inner.push(val); }

    fn get(&self, row: Option<usize>, col: Option<usize>) -> Result<T,Self::Error>
    {
        match (row, col) 
        {
            (Some(index), None) => Ok(self.inner[index]),
            (None, Some(index)) => Ok(self.inner[index]),
            _ => MathError::invalid_index(row?, col?, self.len?, 1).as_result(), 
        }
        
    }

    fn set(&mut self, row: Option<usize>, col: Option<usize>, val:T) -> Result<(),Self::Error>
    {
        match (row, col) 
        {
            (Some(index), None) => {
                self.inner[index] = val;
                Ok(())
            },
            (None, Some(index)) => {
                self.inner[index] = val;
                Ok(())
            },
            _ => Err(MathError::invalid_index(row?, col?, self.len?, 1)), 
        }
    }

    fn hessenberg(&self) -> Result<(Self,Self),Self::Error> {
        MathError::bad_op("Invalid decomposition: cannot decompose a Vector with this operation.").as_result()
    }
}

impl<T: QuantumReal> MatrixAlgebra<T> for SquareMatrix<T>
{
    type Error = MathError;

    fn dim(&self) -> Option<usize> { self.dim }

    fn col_dim(&self) -> Option<usize> { self.dim }

    fn row_dim(&self) -> Option<usize> { self.dim }

    fn update(self, row: Option<usize>, col: Option<usize>) -> Result<Self,Self::Error>
    { 
        let mut N: Self = self.into_inner().into();
        match (row,col) {
            (Some(_), None) => { N.dim = row; },
            (None, Some(_)) => { N.dim = col; },
            (None, None) => { N.dim = Some(self.into_inner().len().sqrt()) }
            (Some(r), Some(c)) => {
                if r==c { 
                    N.dim = row 
                } else { 
                    return MathError::bad_op("Invalid Dimensions: rows must be equivalent to cols when force-updating a square matrix.").as_result() 
                }
            }
        }
        Ok(N)
    }

    fn into_inner(&self) -> Vec<T> { self.inner.clone() }

    fn push(&mut self, val: T) { self.inner.push(val); }

    fn get(&self, row: Option<usize>, col: Option<usize>) -> Result<T,Self::Error>
    {
        let index = row?
            .mul(self.dim?)
            .add(col?);
        Ok(self.inner[index])
    }

    fn set(&mut self, row: Option<usize> , col: Option<usize>, val:T) -> Result<(),Self::Error>
    {
        let index = row?
            .mul(self.dim?)
            .add(col?);
        self.inner[index] = val;
        Ok(())
    }

    fn hessenberg(&self) -> Result<(Self,Self),Self::Error> 
    {
        super::eigen::real_hessenberg(self)
    }
}
