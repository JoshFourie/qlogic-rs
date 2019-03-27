#![allow(non_snake_case)]

/***** Imports ********/
use std::ops::{ Add, Mul};
use super::{ QuantumReal, QuantumUnit, MatrixAlgebra, ComplexMatrixAlgebra, Complex };
use super::matrix_err::MathError;

/***** Struct ********/

#[derive(Debug, PartialEq, Clone)]
pub struct Matrix<T>
{
    pub(crate) inner: Vec<T>,
    pub(crate) row: Option<usize>,
    pub(crate) col: Option<usize>,
    pub(crate) dim: Option<usize>,
}

/***** Std Impl ********/
impl<'a, T> From<Vec<T>> for Matrix<T>
{
    fn from(v: Vec<T>) -> Self {
        Self {
            inner: v,
            row: None,
            col: None,
            dim: None,
        }
    }
}

/***** Trait Impl ********/
// every call to dim should incorporate the zero-indexing
impl<'a, T: QuantumReal> MatrixAlgebra<T> for Matrix<T>
{
    type Error = MathError;

    fn dim(&self) -> Option<usize> { self.dim }

    fn col_dim(&self) -> Option<usize> { self.col }

    fn row_dim(&self) -> Option<usize> { self.row }

    fn update(self, row: Option<usize>, col: Option<usize>) -> Result<Self,Self::Error>
    { 
        let mut N: Self = self.inner.into();
        N.row = row;
        N.col = col;
        N.dim = Some(row?.mul(col?));
        Ok(N)
    }

    fn into_inner(&self) -> Vec<T> { self.inner.clone() }

    fn push(&mut self, val: T) { self.inner.push(val); }

    fn get(&self, row: Option<usize>, col: Option<usize>) -> Result<T,Self::Error>
    {
        let index = row?
            .mul(self.col?)
            .add(col?);
        Ok(self.inner[index])
    }

    fn set(&mut self, row: Option<usize> , col: Option<usize>, val:T) -> Result<(),Self::Error>
    {
        let index = row?
            .mul(self.col?)
            .add(col?);
        self.inner[index] = val;
        Ok(())
    }

    fn hessenberg(&self) -> Result<(Self,Self),Self::Error> 
    {
        super::eigen::real_hessenberg(self)
    }
}

impl<T: num::Float> MatrixAlgebra<Complex<T>> for Matrix<Complex<T>>
where
    Complex<T>: QuantumUnit
{
    type Error = MathError;

    fn dim(&self) -> Option<usize> { self.dim }

    fn col_dim(&self) -> Option<usize> { self.col }

    fn row_dim(&self) -> Option<usize> { self.row }

    fn update(self, row: Option<usize>, col: Option<usize>) -> Result<Self,Self::Error>
    { 
        let mut N: Self = self.inner.into();
        N.row = row;
        N.col = col;
        N.dim = Some(row?.mul(col?));
        Ok(N)
    }

    fn into_inner(&self) -> Vec<Complex<T>> { self.inner.clone() }

    fn push(&mut self, val: Complex<T>) { self.inner.push(val); }

    fn get(&self, row: Option<usize>, col: Option<usize>) -> Result<Complex<T>,Self::Error>
    {
        let index = row?
            .mul(self.col?)
            .add(col?);
        Ok(self.inner[index])
    }

    fn set(&mut self, row: Option<usize> , col: Option<usize>, val: Complex<T>) -> Result<(),Self::Error>
    {
        let index = row?
            .mul(self.col?)
            .add(col?);
        self.inner[index] = val;
        Ok(())
    }

    fn hessenberg(&self) -> Result<(Self,Self),Self::Error> 
    {
        super::eigen::complex_hessenberg(self)
    }
}

impl<T: QuantumUnit> ComplexMatrixAlgebra<Complex<T>> for Matrix<Complex<T>> 
where
    Self: MatrixAlgebra<Complex<T>>,
    Complex<T>: QuantumUnit,
{
    type Error = <Self as MatrixAlgebra<Complex<T>>>::Error;

    fn complex_conjugate(&self) -> Result<Self, Self::Error> 
    {
        self.apply_to_each(|c| c.conj() )
    }

    fn hermitian_conjugate(&self) -> Result<Self, Self::Error>
    {
        self.apply_to_each(|c| c.conj() )?
            .transpose()
    }
}