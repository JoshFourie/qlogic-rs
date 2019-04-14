/****** Imports *******/
pub(crate) use std::ops::{Range, Rem, Mul, Add, Sub, Div};
pub(crate) use std::fmt::Debug;
pub(crate) use num::{Num, Complex, Signed, Float};
pub(crate) use super::ancillary_algorithms::{eigenvalues as eigen};

/****** Exports *******/
pub mod matrix;
pub mod matrix_err;
pub mod matrix_iter;

pub use matrix::*;

/***** Interfaces ********/
pub trait CoreMatrix<T: Copy>
where
    for <'a> &'a Self: IntoIterator<Item=T>, 
    Self: From<Vec<T>> + std::fmt::Debug,
    T: Num
{
    type Error: std::fmt::Debug
    + From<std::option::NoneError>;

    fn dim(&self) -> Option<usize>;

    fn col_dim(&self) -> Option<usize>;

    fn row_dim(&self) -> Option<usize>;
    
    fn update(self, row: Option<usize>, col: Option<usize>) -> Result<Self,Self::Error>;

    fn into_inner(&self) -> Vec<T>;

    fn push(self, val: T) -> Result<Self, Self::Error>;

    fn permute_rows<'a>(&self) -> std::vec::IntoIter<T>
    {
        self.into_inner().into_iter()
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

    // fn destruct(self) {  } // the lib. requires a destructor because there are a lot of references to ensure looping.

}

pub trait BasicTransform<T: Copy>
where
    for <'a> &'a Self: IntoIterator<Item=T>, 
    Self: CoreMatrix<T>,
    T: Num
{
    fn get_sub_matrix(
        &self, 
        alpha: Option<Range<usize>>, 
        beta: Option<Range<usize>>
    ) -> Result<Self, Self::Error>
    {
        let mut A: Self = Vec::new().into();
        for i in alpha? {
            for j in beta.clone()? {
                A.push(self.get(Some(i), Some(j))?);
            }
        }
        Ok(A)
    }

    fn set_sub_matrix(
        &mut self,
        alpha: Option<Range<usize>>, 
        beta: Option<Range<usize>>,
        delta: Vec<T>
    ) -> Result<(), Self::Error>
    {
        let mut l: usize = 0;
        for i in alpha? {
            for j in beta.clone()? {
                self.set(Some(i), Some(j), delta[l])?;
                l += 1;
            }
        }
        Ok(())
    }

    fn transpose(&self) -> Result<Self, Self::Error> {
        let M = Self::from( self
            .permute_cols()?
            .collect::<Vec<T>>()
        ).update(self.row_dim(),self.col_dim())?;
        Ok(M)
    }

    fn eucl_norm(&self) -> T  
    { 
        let y = self.into_iter()
            .fold(T::zero(), |acc,x| acc + num::pow(x, 2) );
        num::pow(y, 1.div(2) as usize)       
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
                    sigma = sigma + self.get(Some(i), Some(k))?.mul( rhs.get(Some(k), Some(j))?);
                }

                M.push(sigma);
            }
        }
        Ok( Self::from(M).update(m, n)? )
    }

    fn identity(&self) -> Result<Self,Self::Error>
    {
        let col_dim = self.col_dim();
        let mut id: Self = Self::from( vec![T::zero(); self.dim()?] )
            .update(col_dim, col_dim)?; 
        for i in 0..col_dim? {
            id.set(Some(i), Some(i), T::one())?;
        }
        Ok(id)
    }

    fn trace(&self) -> Result<T,Self::Error>
    {
        let mut sigma = T::zero();
        for n in self.diagonal()?
            .into_iter()
        {
            sigma = sigma + n;
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
            .permute_rows()
            .zip(rhs.permute_rows())
            .map(|(l,r)| l+r)
            .collect::<Vec<T>>()
        ).update(self.row_dim(), self.col_dim())?;
        Ok(M)
    }

    fn subtraction(&self, rhs: &Self) ->  Result<Self, Self::Error> 
    {
        let M: Self = Self::from(self
            .permute_rows()
            .zip(rhs.permute_rows())
            .map(|(l,r)| l-r)
            .collect::<Vec<T>>()
        ).update(self.row_dim(), self.col_dim())?;
        Ok(M)
    }
}

pub trait ComplexTransform<T: Copy>
where
    for <'a> &'a Self: IntoIterator<Item=T>,
    Self: BasicTransform<T>,
    Complex<T>: Num,
    T: Num
{    
    fn complex_conjugate(&self) -> Result<Self, Self::Error>;

    fn hermitian_conjugate(&self) -> Result<Self, Self::Error>;
}

pub trait EigenValueDecomposition<T: Copy>
where
    for <'a> &'a Self: IntoIterator<Item=T>, 
    Self: BasicTransform<T>,
    T: Num
{
    fn decomposition(&self) -> Result<(Self,Self),Self::Error>;

    // CONFIRM THIS AS NEEDING DECOMPOSITION?
    // Error on test.
    fn determinant(&self) -> Result<T,Self::Error>
    {
        let (_,R) = self.decomposition()?;
        let det = R.diagonal()?
            .into_iter()
            .fold(T::one(), |acc,t| acc.mul(t));
        Ok(det)
    }

    fn eigen_values(&self) -> Result<Vec<T>, Self::Error>
    {
        let (Q,R) = self.decomposition()?;
        let mut X = R.cross(&Q)?
            .decomposition()?;
        for _ in 0..R.dim()?
        {
            let (Y,Z) = X;
            X = Z.cross(&Y)?
                .decomposition()?;
        }
        let (_,A) = X;
        Ok( A.diagonal()? )
    }
}

pub trait ElementaryRowOperations<T: Copy + Debug>
where
    for <'a> &'a Self: IntoIterator<Item=T>, 
    Self: CoreMatrix<T>,
    T: Num
{
    fn row_swap(&self, r1: Option<usize>, r2: Option<usize>) -> Result<Self, Self::Error>
    {
        let mut M: Self = self.into_inner().into();
        for c in 0..M.dim()?
        {
            let sigma = M.get(r1, Some(c))?;
            let omega = M.get(r2, Some(c))?;
            M.set(r1, Some(c), omega)?;
            M.set(r2, Some(c), sigma)?;
        }
        Ok(M)
    }

//    fn row_scalar(&self, scalar: T) -> Result<Self, Self::Error> { }
}

/* 
pub trait GuassianElimination<T: Copy> 
where 
    for <'a> &'a Self: IntoIterator<Item=T>, 
    Self: BasicTransform<T>,
    T: Float
{
    fn gaussian_elimination(&self) // -> Result<Self, Self::Error>
    {
        match self.forward_elimination() {
            Some(k) => {

            }
        }
    }

    fn forward_elimination(&self) -> Result<Option<usize>, Self::Error>
    {
        assert_eq!(self.row_dim()?.add(1), self.col_dim()?);
        let N: usize = self.row_dim()?;
        let mut M: Self = self.into_inner().into();
        for k in 0..N {
            let max_index = k;
            let max_val = M.get(Some(max_index), Some(k))?;
            
            for i in k.add(1)..N {
                let xi = M.get(Some(i), Some(k))?; 
                if xi.abs() > max_val {
                    max_val = xi;
                    max_index = i;
                }
            }
            if M.get(Some(k), Some(max_index))? == T::zero() { return Ok(Some(k)) } 
            if max_index != k { M.row_swap(Some(k), Some(max_index))?; }

            for i in k.add(1)..N {
                let f: T = M.get(Some(i), Some(k))?
                    .div(M.get(Some(k), Some(k))?); 
                for j in k.add(1)..=N {
                    let sigma = M.get(Some(k), Some(j))?
                        .mul(f);
                    let omega = M.get(Some(i), Some(j))?;
                    M.set(Some(i), Some(j), omega.sub(sigma))?;
                }
                M.set(Some(i), Some(k), T::zero())?;
            }
        }
        Ok(None)
    }

    fn backwards_substitution(&self) -> Result<Self, Self::Error>
    {
        let mut M: Self = vec![T::zero(); self.row_dim()?].into();
        
        for i in self.row_dim()?.sub(1)..0 {
            M.set(Some)
        }
        Ok(M)
    }
}
*/