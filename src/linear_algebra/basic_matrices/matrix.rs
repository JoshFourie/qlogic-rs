#![allow(non_snake_case)]

/***** Imports ********/
use super::matrix_err::MathError;
use super::*;

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
impl<T: Copy + Debug> CoreMatrix<T> for Matrix<T>
where
    T: Num
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
}

impl<T: Num + Copy + Debug> BasicTransform<T> for Matrix<T> { }

impl<T: Copy + Debug> EigenValueDecomposition<T> for Matrix<T>
where
    T: Float + Signed
{
    // numerically unstable, error is unacceptable. 
    fn decomposition(&self) -> Result<(Self,Self),Self::Error> 
    {
        super::eigen::real_hessenberg(self)
    }
}

// It's intuitively strange to have the default impl pick up the complex specialisation but
// without a complex trait this is the better solution.
impl<T: Copy + Debug> EigenValueDecomposition<T> for Matrix<T>
where
    T: Num
{
    default fn decomposition(&self) -> Result<(Self,Self),Self::Error> 
    {
        MathError::bad_spec("temp : no complex impl").as_result()
    }   
}

impl<T: Copy> ComplexTransform<Complex<T>> for Matrix<Complex<T>> 
where
    Complex<T>: Num,
    T: Num + Signed + Debug
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