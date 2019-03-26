#![allow(non_snake_case)]

/***** Imports ********/
use num_integer::Roots;
use std::ops::{Div, Rem, Sub, Add, Mul};
use num::Complex;
use super::{ QuantumUnit, QuantumReal,  MatrixAlgebra };
use std::result::Result;

/***** Struct ********/

#[derive(Debug, PartialEq, Clone)]
pub struct Matrix<T>
{
    inner: Vec<T>,
    row: Option<usize>,
    col: Option<usize>,
    dim: Option<usize>,
}

#[derive(Debug)]
pub enum MathError
{
    NoneVal(std::option::NoneError),
    BadIndex(String),
}

/***** Impl ********/

// row major iteration.
impl<'a, T> IntoIterator for Matrix<T>
{
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter { self.inner.into_iter() }
}

impl<'a, T> From<Vec<T>> for Matrix<T>
{
    fn from(v: Vec<T>) -> Self {
        Matrix {
            inner: v,
            row: None,
            col: None,
            dim: None,
        }
    }
}

impl From<std::option::NoneError> for MathError
{
    fn from(e: std::option::NoneError) -> Self { MathError::NoneVal(e) }
}

// every call to dim should incorporate the zero-indexing
impl<'a, T: QuantumReal> MatrixAlgebra<T> for Matrix<T>
where
    Self: Clone
{
    type Error = MathError;

    fn dim(&self) -> Option<usize> { self.dim }

    fn col_dim(&self) -> Option<usize> { self.col }

    fn row_dim(&self) -> Option<usize> { self.row }

    fn update(self, row: usize, col: usize) -> Result<Self,Self::Error>
    { 
        let mut N: Self = self.inner.into();
        N.row = Some(row);
        N.col = Some(col);
        N.dim = Some(row.mul(col));
        Ok(N)
    }

    fn permute_cols(&self) -> Result<std::vec::IntoIter<T>,Self::Error>
    {
        let mut scratch: Vec<T> = Vec::new();
        for i in 0..self.row? {
            for j in 0..self.col? {
                scratch.push( self.inner[ j.mul(self.row?).add(i) ]);
            }
        }
        Ok(scratch.into_iter())
    }

    fn permute_rows(&self) -> Result<std::vec::IntoIter<T>,Self::Error>
    {
        let mut scratch: Vec<T> = Vec::new();
        for i in 0..self.col? {
            for j in 0..self.row? {
                scratch.push( self.inner[ i.mul(self.row?).add(j) ]);
            }
        }
        Ok(scratch.into_iter())
    }

    fn into_inner(self) -> Vec<T> { self.inner }

    fn push(&mut self, val: T) { self.inner.push(val); }

    fn get(&self, row:usize, col:usize) -> Result<T,Self::Error>
    {
        let index = row
            .mul(self.col?)
            .add(col);
        Ok(self.inner[index])
    }

    fn set(&mut self, row:usize, col:usize, val:T) -> Result<(),Self::Error>
    {
        let index = row
            .mul(self.col?)
            .add(col);
        self.inner[index] = val;
        Ok(())
    }

    fn eucl_norm(&self) -> T  
    { 
        self.inner
            .iter()
            .fold(T::zero(), |acc,x| acc + x.pow64(2.0))
            .sqroot()
    }

    fn scalar(&self, rhs: T) -> Result<Self, Self::Error>
    {
        let M = Self::from(self
            .inner
            .iter()
            .map(|n| n.mul(rhs))
            .collect::<Vec<T>>()
        ).update(self.row_dim()?, self.col_dim()?)?;
        Ok(M)
    }

    fn hessenberg(&self) -> Result<(Self,Self),Self::Error>
    {
        let mut M: Self = self.clone();
        let mut Q_store: Vec<Self> = Vec::new();
        let mut R_store: Vec<Self> = Vec::new();
        let col_dim = M.col_dim()?;
        let row_dim = M.row_dim()?;

        for k in 0..col_dim.sub(1) {
            let x: Self = Self::from(M.extract_col(k)?)
                .update(row_dim, 1)?;
            let alpha: T = x.get(k.add(1),0)?
                .signum()
                .mul( x.eucl_norm() );
            let epsilon: Self = {
                let mut e = vec![T::zero(); col_dim];
                e[k] = T::one();
                Self::from(e).update(row_dim, 1)?
            };
            let mu: Self = x.subtraction(&epsilon.scalar(alpha)?)?;
            let mu_norm: T = mu.eucl_norm();
            let I: Self = M.identity()?;
            let vvT: Self = mu.kronecker(&mu)?;
            let Qk: Self = I.subtraction(
                &vvT.scalar( T::one().add(T::one()).div(mu_norm.mul(mu_norm)) )?
            )?;
            let mut Q: Self = Qk.cross(&M)?;
            for i in 0..row_dim {
                Q.set(k,i,T::zero())?;
                Q.set(i,k,T::zero())?;
            }
            Q_store.push(Qk.transpose()?);
            R_store.push(Qk);

            Q.set(k,k,T::one())?;
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

impl MathError 
{
    pub fn invalid_index(c: usize, r: usize, max_r: usize, max_c: usize) -> Self {
        MathError::BadIndex(format!("Invalid Index: indexed at {},{}, but the maximum input is {},{}",r,c,max_r,max_c))
    }  
}