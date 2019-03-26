#![allow(non_snake_case)]

use std::ops::{ Add, Mul, Sub, Div, Rem };

pub mod matrix;

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

pub trait MatrixAlgebra<T: QuantumUnit>
where
    Self: IntoIterator<Item=T> 
    + From<Vec<T>>
    + Clone
{
    type Error: std::fmt::Debug
    + From<std::option::NoneError>;

    fn dim(&self) -> Option<usize>;

    fn col_dim(&self) -> Option<usize>;

    fn row_dim(&self) -> Option<usize>;
    
    fn update(self, row: usize, col: usize) -> Result<Self,Self::Error>;

    fn into_inner(self) -> Vec<T>;

    fn push(&mut self, val: T);

    fn permute_rows(&self) -> Result<std::vec::IntoIter<T>,Self::Error>;

    fn permute_cols(&self) -> Result<std::vec::IntoIter<T>,Self::Error>;

    fn apply_to_each<F: Fn(T)->T>(self, action: F) -> Self
    {
        self.into_iter()
            .map(|x| action(x))
            .collect::<Vec<T>>()
            .into()
    }

    fn extract_row(&self, r: usize) -> Result<Vec<T>,Self::Error>
    {
        let mut v: Vec<T> = Vec::new();
        for c in 0..self.col_dim()? {
            let val = self.get(r,c)?;
            v.push(val)
        }
        Ok(v)
    }

    fn extract_col(&self, c: usize) -> Result<Vec<T>,Self::Error>
    {
        let mut v: Vec<T> = Vec::new();
        for r in 0..self.row_dim()? {
            let val = self.get(r,c)?;
            v.push(val)
        }
        Ok(v)
    }

    fn get(&self, row:usize, col:usize) -> Result<T,Self::Error>;
    
    fn set(&mut self, row:usize, col:usize, val:T) -> Result<(),Self::Error>;

    fn transpose(&self) -> Result<Self, Self::Error> {
        let M = Self::from( self
            .permute_cols()?
            .collect::<Vec<_>>()
        ).update(self.row_dim()?,self.col_dim()?)?;
        Ok(M)
    }

    fn eucl_norm(&self) -> T;    

    // index error stopped when we switch i,j in N and then transpose?
    fn kronecker(&self, rhs: &Self) -> Result<Self,Self::Error>
    {   
        let n = self.col_dim()?; 
        let q = rhs.col_dim()?; 
        let m = self.row_dim()?;
        let p = rhs.row_dim()?; 
        
        let mut N: Self = Self::from(vec![T::zero(); m.mul(n).mul(p).mul(q)])
            .update(
                m.mul(p),
                n.mul(q)
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
                
                let alpha = self.get(a1,a2)?;
                let beta = rhs.get(b1,b2)?;
                let delta = alpha.mul(beta);
                
                N.set(i as usize - 1, j as usize - 1, delta)?;
            }
        }
        Ok( N )  
    }

    fn scalar(&self, rhs: T) -> Result<Self, Self::Error>;

    fn cross(&self, rhs: &Self) -> Result<Self,Self::Error>
    {
        let m = self.row_dim()?;
        let n = rhs.col_dim()?;

        let mut M: Vec<T> = Vec::new();
        for i in 0..m {
            for j in 0..n {
                let mut sigma = T::zero();
                for k in 0..rhs.row_dim()? 
                {
                    sigma += self.get(i,k)?.mul(rhs.get(k,j)?);
                }

                M.push(sigma);
            }
        }
        Ok( Self::from(M).update(m, n)? )
    }

    // fn eigen_values(self) -> Result<Vec<T>, Self::Error>;

    fn identity(&self) -> Result<Self,Self::Error>
    {
        let col_dim = self.col_dim()?;
        let mut id: Self = Self::from( vec![T::zero(); self.dim()?] )
            .update(col_dim, col_dim)?;
        for i in 0..col_dim {
            id.set(i,i,T::one())?
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
            diag.push(self.get(i,i)?);
        }
        Ok(diag)
    }

    fn addition(&self, rhs: &Self) -> Result<Self, Self::Error>
    {
        let row_dim = self.row_dim()?;
        let col_dim = self.col_dim()?;
        let M: Self = Self::from(self
            .permute_rows()?
            .zip(rhs.permute_rows()?)
            .map(|(l,r)| l+r)
            .collect::<Vec<T>>()
        ).update(row_dim, col_dim)?;
        Ok(M)
    }

    fn subtraction(&self, rhs: &Self) ->  Result<Self, Self::Error> 
    {
        let row_dim = self.row_dim()?;
        let col_dim = self.col_dim()?;
        let M: Self = Self::from(self
            .permute_rows()?
            .zip(rhs.permute_rows()?)
            .map(|(l,r)| l-r)
            .collect::<Vec<T>>()
        ).update(row_dim, col_dim)?;
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