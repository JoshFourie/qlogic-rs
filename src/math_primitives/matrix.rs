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
impl<T: QuantumUnit> MatrixAlgebra<T> for Matrix<T>
{
    type Error = MathError;

    fn dim(&self) -> Option<usize> { self.dim }

    fn col_dim(&self) -> Option<usize> { self.col }

    fn row_dim(&self) -> Option<usize> { self.row }

    fn update(self, row: Option<usize>, col: Option<usize>) -> Result<Self,Self::Error>
    { 
        match (row,col) {
            (Some(r),Some(c)) => {
                let mut N: Self = self.inner.into();
                N.row = row;
                N.col = col;
                N.dim = Some(r.mul(c));
                return Ok(N)
            },
            (_,_) => MathError::bad_input("MatrixAlgebra::update() requires (Some(),Some()) input.").as_result()
        }
    }

    fn into_inner(&self) -> Vec<T> { self.inner.clone() }

    fn push(&mut self, val: T) { self.inner.push(val); }

    fn get(&self, row: Option<usize>, col: Option<usize>) -> Result<T,Self::Error>
    {
        if self.row? > row? && self.col? > col? 
        {
            let i = row?
                .mul(self.col_dim()?)
                .add(col?);
            return Ok(self.inner[i])
        } else { MathError::invalid_index(row?, col?, self.row?, self.col?).as_result() }
    }

    fn set(&mut self, row: Option<usize> , col: Option<usize>, val:T) -> Result<(),Self::Error>
    {
        let i = row?
            .mul(self.col_dim()?)
            .add(col?);
                self.inner[i] = val;
        Ok(())
    }

    default fn hessenberg(&self) -> Result<(Self,Self),Self::Error> 
    {
        MathError::bad_spec("temp").as_result()
    }
}

impl<T: QuantumReal> MatrixAlgebra<T> for Matrix<T>
{
    // numerically unstable, error is unacceptable. 
    fn hessenberg(&self) -> Result<(Self,Self),Self::Error> 
    {
        super::eigen::real_hessenberg(self)
    }
}

impl MatrixAlgebra<Complex<f32>> for Matrix<Complex<f32>>
{
    fn hessenberg(&self) -> Result<(Self,Self),Self::Error> 
    {
        super::eigen::complex_hessenberg(self)
    }
}

impl MatrixAlgebra<Complex<f64>> for Matrix<Complex<f64>>
{
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