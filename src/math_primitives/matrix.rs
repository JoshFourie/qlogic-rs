#![allow(non_snake_case)]

/***** Imports ********/
use std::ops::{Sub, Add, Mul};
use super::{ QuantumReal, MatrixAlgebra };
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
        let mut M: Self = self.clone();
        let mut Q_store: Vec<Self> = Vec::new();
        let mut R_store: Vec<Self> = Vec::new();
        let col_dim = M.col_dim();
        let row_dim = M.row_dim();

        for k in 0..col_dim?.sub(1) {
            let x: Self = Self::from(M.extract_col(k)?)
                .update(row_dim, Some(1))?;
            let alpha: T = x.get( Some(k.add(1)), Some(0))?
                .signum()
                .mul( x.eucl_norm() );
            let epsilon: Self = {
                let mut e = vec![T::zero(); col_dim?];
                e[k] = T::one();
                Self::from(e).update(row_dim, Some(1))?
            };
            let mu: Self = x.subtraction(&epsilon.scalar(alpha)?)?;
            let mu_norm: T = mu.eucl_norm();
            let I: Self = M.identity()?;
            let vvT: Self = mu.kronecker(&mu)?;
            let Qk: Self = I.subtraction(
                &vvT.scalar( T::one().add(T::one()).div(mu_norm.mul(mu_norm)) )?
            )?;
            let mut Q: Self = Qk.cross(&M)?;
            for i in 0..row_dim? {
                Q.set(Some(k), Some(i), T::zero())?;
                Q.set(Some(i), Some(k), T::zero())?;
            }
            Q_store.push(Qk.transpose()?);
            R_store.push(Qk);

            Q.set(Some(k), Some(k), T::one())?;
            M = Q;
        }
        // we have to unwrap here because acc.
        let R: Self = R_store.into_iter()
            .rev()
            .fold(M.identity()?, |acc,q| acc.cross(&q).unwrap());
        let Q: Self = Q_store.into_iter()
            .fold(M.identity()?, |acc,q| acc.cross(&q).unwrap());
        Ok((Q,R))
    }
}