#![allow(non_snake_case)]

use std::ops::{ Add, Mul, Sub, Div, Rem };
use std::iter::Iterator;
use num::Complex;

pub mod matrix;
mod matrix_iter;
mod matrix_wrap;
mod matrix_err;
mod eigen;

/***** Interfaces ********/
pub trait QuantumUnit: num::Num
    + std::ops::Neg<Output=Self>
    + std::ops::AddAssign
    + Copy
{
    // these are not implemented as a trait in Rust.
    fn pow64(self, rhs: f64) -> Self;
    fn sqroot(self) -> Self;
}

// trait for scalar units that have traits like PartialOrd<Self> & Signum
// the QuantumUnit is only useful for allowing specialisation.
pub trait QuantumReal: QuantumUnit + num_traits::real::Real { }

/*
Required methods:
    - dim()
    - col_dim()
    - row_dim()
    - update()
    - into_inner()
    - push()
    - hessenberg()
The rest are provided with the trait.
*/
pub trait MatrixAlgebra<T: QuantumUnit>
where
    for <'a> &'a Self: IntoIterator<Item=T>, 
    Self: From<Vec<T>>
    + Clone
{
    type Error: std::fmt::Debug
    + From<std::option::NoneError>;

    fn dim(&self) -> Option<usize>;

    fn col_dim(&self) -> Option<usize>;

    fn row_dim(&self) -> Option<usize>;
    
    fn update(self, row: Option<usize>, col: Option<usize>) -> Result<Self,Self::Error>;

    fn into_inner(&self) -> Vec<T>;

    fn push(&mut self, val: T);

    fn permute_rows<'a>(&self) -> Result<std::vec::IntoIter<T>,Self::Error>
    {
        let mut scratch: Vec<T> = Vec::new();
        let row = self.row_dim()?;
        let col = self.col_dim()?;
        let inner = self.into_inner();
        for i in 0..col{
            for j in 0..row {
                scratch.push( inner[ i.mul(row).add(j) ]);
            }
        }
        Ok(scratch.into_iter())
    }

    fn permute_cols<'a>(&self) -> Result<std::vec::IntoIter<T>,Self::Error>
    {
        let mut scratch: Vec<T> = Vec::new();
        let row = self.row_dim()?;
        let inner = self.into_inner();
        for i in 0..row {
            for j in 0..self.col_dim()? {
                scratch.push( inner[ j.mul(row).add(i) ]);
            }
        }
        Ok(scratch.into_iter())
    }

    fn apply_to_each<F: Fn(T)->T>(&self, action: F) -> Result<Self, Self::Error>
    {
        Ok( Self::from(
            self.into_iter()
            .map(|x| action(x))
            .collect::<Vec<T>>()
        ).update(self.row_dim(), self.col_dim())?)
    }

    fn extract_row(&self, r: usize) -> Result<Vec<T>,Self::Error>
    {
        let mut v: Vec<T> = Vec::new();
        for c in 0..self.col_dim()? {
            let val = self.get(Some(r), Some(c))?;
            v.push(val)
        }
        Ok(v)
    }

    fn extract_col(&self, c: usize) -> Result<Vec<T>,Self::Error>
    {
        let mut v: Vec<T> = Vec::new();
        for r in 0..self.row_dim()? {
            let val = self.get(Some(r), Some(c))?;
            v.push(val)
        }
        Ok(v)
    }

    fn get(&self, row: Option<usize>, col: Option<usize>) -> Result<T,Self::Error>;
    
    fn set(&mut self, row: Option<usize>, col: Option<usize>, val:T) -> Result<(),Self::Error>;

    fn transpose(&self) -> Result<Self, Self::Error> {
        let M = Self::from( self
            .permute_cols()?
            .collect::<Vec<T>>()
        ).update(self.row_dim(),self.col_dim())?;
        Ok(M)
    }

    fn eucl_norm(&self) -> T  
    { 
        self.into_iter()
            .fold(T::zero(), |acc,x| acc + x.pow64(2.0))
            .sqroot()
    }

    // index error stopped when we switch i,j in N and then transpose?
    fn kronecker(&self, rhs: &Self) -> Result<Self,Self::Error>
    {   
        let n = self.col_dim()?; 
        let q = rhs.col_dim()?; 
        let m = self.row_dim()?;
        let p = rhs.row_dim()?; 
        
        let mut N: Self = Self::from(vec![T::zero(); m.mul(n).mul(p).mul(q)])
            .update(
                Some(m.mul(p)),
                Some(n.mul(q))
            )?;
        for i in 1..=m.mul(p) as usize { 
            for j in 1..=n.mul(q) as usize 
            {
                let i = i as f64;
                let j = j as f64;
                
                let a1 = i.sub(1.0).div(p as f64).floor().add(1.0) as usize - 1;
                let a2 = j.sub(1.0).div(q as f64).floor().add(1.0) as usize - 1;
                let b1 = i.sub(1.0).rem(p as f64).add(1.0) as usize - 1;
                let b2 = j.sub(1.0).rem(q as f64).add(1.0) as usize - 1;
                
                let alpha = self.get(Some(a1), Some(a2))?;
                let beta = rhs.get(Some(b1), Some(b2))?;
                let delta = alpha.mul(beta);
                
                N.set( Some(i as usize - 1), Some(j as usize - 1), delta)?;
            }
        }
        Ok( N )  
    }

    fn scalar(&self, rhs: T) -> Result<Self, Self::Error>
    {
        let M = Self::from(self
            .into_iter()
            .map(|n| n.mul(rhs))
            .collect::<Vec<T>>()
        ).update(self.row_dim(), self.col_dim())?;
        Ok(M)
    }

    fn cross(&self, rhs: &Self) -> Result<Self,Self::Error>
    {
        let m = self.row_dim();
        let n = rhs.col_dim();

        let mut M: Vec<T> = Vec::new();
        for i in 0..m? {
            for j in 0..n? {
                let mut sigma = T::zero();
                for k in 0..rhs.row_dim()? 
                {
                    sigma += self.get(Some(i), Some(k))?.mul( rhs.get(Some(k), Some(j))?);
                }

                M.push(sigma);
            }
        }
        Ok( Self::from(M).update(m, n)? )
    }

    // fn eigen_values(self) -> Result<Vec<T>, Self::Error>;

    fn identity(&self) -> Result<Self,Self::Error>
    {
        let col_dim = self.col_dim();
        let mut id: Self = Self::from( vec![T::zero(); self.dim()?] )
            .update(col_dim, col_dim)?;
        for i in 0..col_dim? {
            id.set(Some(i), Some(i), T::one())?
        }
        Ok(id)
    }

    fn trace(&self) -> Result<T,Self::Error>
    {
        let mut sigma = T::zero();
        for n in self.diagonal()?
            .into_iter()
        {
            sigma += n;
        }
        Ok(sigma)
    }

    fn diagonal(&self) -> Result<Vec<T>,Self::Error>
    {
        let mut diag: Vec<T> = Vec::new();
        for i in 0..self.col_dim()? {
            diag.push(self.get(Some(i), Some(i))?);
        }
        Ok(diag)
    }

    fn addition(&self, rhs: &Self) -> Result<Self, Self::Error>
    {
        let M: Self = Self::from(self
            .permute_rows()?
            .zip(rhs.permute_rows()?)
            .map(|(l,r)| l+r)
            .collect::<Vec<T>>()
        ).update(self.row_dim(), self.col_dim())?;
        Ok(M)
    }

    fn subtraction(&self, rhs: &Self) ->  Result<Self, Self::Error> 
    {
        let M: Self = Self::from(self
            .permute_rows()?
            .zip(rhs.permute_rows()?)
            .map(|(l,r)| l-r)
            .collect::<Vec<T>>()
        ).update(self.row_dim(), self.col_dim())?;
        Ok(M)
    }

    fn hessenberg(&self) -> Result<(Self,Self),Self::Error>;
    
    fn determinant(&self) -> Result<T,Self::Error>
    {
        let (_,R) = self.hessenberg()?;
        let det = R.cross(self)?
            .diagonal()?
            .into_iter()
            .fold(T::one(), |acc,t| acc.mul(t));
        Ok(det)
    }
}

pub trait ComplexMatrixAlgebra<T>: Sized
{
    type Error;
    
    fn complex_conjugate(&self) -> Result<Self, Self::Error>;

    fn hermitian_conjugate(&self) -> Result<Self, Self::Error>;
}

/***** Impls ********/
impl QuantumUnit for isize { 
    fn pow64(self, rhs: f64) -> Self { self.pow(rhs as u32) }
    fn sqroot(self) -> Self { (self as f64).sqrt() as isize }
}
impl QuantumUnit for f32 {
    fn pow64(self, rhs: f64) -> Self { self.powf(rhs as  f32) }     
    fn sqroot(self) -> Self { self.sqrt() }
}
impl QuantumUnit for f64 {     
    fn pow64(self, rhs: f64) -> Self { self.powf(rhs) }     
    fn sqroot(self) -> Self { self.sqrt() }
}

impl QuantumUnit for num::Complex<f32> {
    fn pow64(self, rhs: f64) -> Self { self.powf(rhs as f32) }     
    fn sqroot(self) -> Self { self.sqrt() }
}
impl QuantumUnit for num::Complex<f64> {
    fn pow64(self, rhs: f64) -> Self { self.powf(rhs) }     
    fn sqroot(self) -> Self { self.sqrt() }
}

impl QuantumReal for f32 { }

impl QuantumReal for f64 { }