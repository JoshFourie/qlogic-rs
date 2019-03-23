// We only consider square matrices for quantum mechanics.
// Conventinoally matrices are denoted with capitals: A,B,C...
#![allow(non_snake_case)] 

/***** Imports ********/

use num_integer::Roots;
use std::ops::{Div, Rem, Sub, Add, Mul};
use num::Complex;
use super::{ QuantumUnit, QuantumReal, ComplexMatrixAlgebra, VectorAlgebra, MatrixAlgebra };
use super::error::{MatrixError};
use std::result::Result;

/***** Struct ********/

#[derive(Debug, PartialEq, Clone)]
pub struct Matrix<T>
{
    inner: Vec<T>,
    dim: usize,
}

pub type ComplexMatrix<T> = Matrix<Complex<T>>;

/***** Impl ********/

// row major iteration.
impl<T> IntoIterator for Matrix<T>
{
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter
    {
        self.inner.into_iter()
    }
}

impl<T:QuantumUnit> MatrixAlgebra<T> for Matrix<T>
{
    type Inner = Vec<T>;
    type Error = MatrixError;

    // dimension getter function.
    fn dim(&self) -> usize { self.dim }

    fn update_dim(&mut self) { self.dim=self.inner.len().sqrt(); }

    // inner getter function. 
    fn into_inner(self) -> Vec<T> { self.inner } 

    // append to inner val.
    fn push(&mut self, val: T) {self.inner.push(val);}

    // row-major permutation.
    fn permute_rows(self) -> std::vec::IntoIter<T> {self.into_iter()}

    // col-major permutation.
    fn permute_cols(self) -> std::vec::IntoIter<T>
    {
        let mut scratch = Vec::new();
        for col_index in 0..self.dim
        {   
            for row_index in 0..self.dim
            {
                scratch.push( self.inner[row_index*self.dim + col_index]);
            }
        }
        scratch.into_iter()
    }

    // pull out a copy of a row at a given index.
    fn extract_row(&self, r: usize) -> Result<Self::Inner,Self::Error>
    {
        let mut v: Vec<T> = Vec::new();
        for c in 0..self.dim() {
            let val = self.get(r,c)?;
            v.push(val)
        }
        Ok(v)
    }

    // pull out a copy of a col at a given index.
    fn extract_col(&self, c: usize) -> Result<Self::Inner,Self::Error>
    {
        let mut v: Vec<T> = Vec::new();
        for r in 0..self.dim() {
            let val = self.get(r,c)?;
            v.push(val)
        }
        Ok(v)
    }

    // retrieve value at an index. May fail.
    fn get(&self, row: usize, col: usize) -> Result<T, Self::Error>
    {
        let index = row*self.dim +col;
        match index < self.inner.len()
        {
            true => Ok(self.inner[index]),
            false => MatrixError::invalid_index(row, col, self.dim).as_result(),
        }   
    }

    // set a value at an index. May fail.
    fn set(&mut self, row: usize, col: usize, val: T) -> Result<(), Self::Error>
    {
        match row+col<self.inner.len()
        {
            true => {
                self.inner[row*self.dim+col] = val;
                Ok(())
            },
            false => MatrixError::invalid_index(row, col, self.dim).as_result(),
        }
    }

    // (A(x)B)ij = (a)|(i-1)/p|+1,|(j-1)/q|+1 dot (b)(i-1)%p,(j-1)%q+1
    // where (A(x)B) repr. output at a position i,j
    // a & b repr. elements in matrices 
    // p & q repr. dimension of B
    // rust zero-indexes vecs but the above indexes at 1, we sub 1 after
    // casting to usize to circumvent the limitation.
    // we are also dealing with square matrices so p & q are identical. 
    fn kronecker(&self, rhs: &Self) -> Result<Self,Self::Error>
    {
        let mut new = Self::from(Vec::new());
        let dim = self.dim*rhs.dim;
        let pq =rhs.dim as f64;
        for i in 1..=dim {
            let i = i as f64;
            for j in 1..=dim {
                let j = j as f64;              
                let a = self.get(
                    (f64::floor( (i-1.0).div(pq)+1.0 )) as usize -1,
                    (f64::floor( (j-1.0).div(pq)+1.0 )) as usize -1,
                )?;
                let b = rhs.get(
                    ((i-1.0).rem(pq)+1.0) as usize -1,
                    ((j-1.0).rem(pq)+1.0) as usize -1,
                )?;
                new.push(a*b);
            }
        }
        new.dim=dim;
        Ok(new)
    }

    // transposes by col permutation set as rows of the new matrix.
    fn transpose(self) -> Self
    {
        self.permute_cols().collect::<Vec<_>>().into()
    }

    // scalar multiplication.
    fn scalar(self, rhs: T) -> Self
    {
        Self::from( 
            self.into_iter()
            .map(|val| val*rhs)
            .collect::<Vec<T>>()   
        )
    }

    // standard matrix multiplication
    fn cross(&self, rhs: &Self) -> Result<Self,Self::Error>
    {
        assert_eq!(self.dim,rhs.dim);
        let len = self.dim;
        let mut new = Self::from(Vec::new());
        for i in 0..len {
            for j in 0..len {
                let mut sigma = T::zero();
                for k in 0..len
                {
                    let aik = self.get(i,k)?;
                    let bkj = rhs.get(k,j)?; 
                    sigma += aik*bkj;
                }
            new.push(sigma)
            } 
        }
        new.dim = len;
        Ok(new)
    }


    // vector_product to allow us to skip a for loop.
    fn vector_product<V: VectorAlgebra<T>>(self, rhs: V) -> Result<V,Self::Error>
    where
        Self::Error: From<V::Error>
    {
        let mut new = V::from(Vec::new());
        for i in 0..self.dim {
            let mut sigma = T::zero();
            for k in 0..self.dim {
                let aik = self.get(i,k)?;
                let b = rhs.get(k)?;
                sigma += aik*b;
            }
            new.push(sigma);
        }
        Ok(new)
    } 

    // returns the diagonal of the Matrix.
    fn diagonal(&self) -> Result<Self::Inner,Self::Error>
    {
        let mut d = Vec::new();
        for j in 0..self.dim {
            d.push(self.get(j, j)?)
        }
        Ok(d)
    }

    // trace is the sum of the diagonals.
    fn trace(self) -> Result<T,Self::Error>
    {
        let mut sigma = T::zero();
        for val in self.diagonal()?
            .into_iter()
        {
            sigma += val;
        }
        Ok(sigma)
    }

    // matrix addition.
    fn addition(self, rhs: Self) -> Self
    {
        Self::from( 
            self.permute_rows()
            .zip(rhs.permute_rows())
            .map(|(lhs,rhs)| lhs+rhs)
            .collect::<Vec<_>>()
        )
    }

    fn subtraction(self, rhs: Self) -> Self
    {
        assert_eq!(self.dim(), rhs.dim());
        self.permute_rows()
            .zip(rhs.permute_rows())
            .map(|(lhs,rhs)| lhs-rhs)
            .collect::<Vec<_>>()
            .into()
    }

    fn apply_to_each<F: Fn(T)->T>(self, action: F) -> Self
    {
        self.into_iter()
            .map(|x| action(x))
            .collect::<Vec<_>>()
            .into()
    }

    fn identity(&self) -> Result<Self,Self::Error>
    {
        let mut id: Self = vec![T::zero(); self.dim*self.dim].into();
        for j in 0..self.dim {
            id.set(j,j,T::one())?
        }
        Ok(id)
    }    

    default fn hessenberg<W>(&self) -> Result<(Self,Self),Self::Error> 
    where
        W: VectorAlgebra<T>
    {
        MatrixError::specialisation("decomposing matrix: did not match function to appropriate primitive").as_result()
    }

    fn determinant<X>(&self) -> Result<T,Self::Error>
    where 
        X: VectorAlgebra<T>,
        Self::Error: From<X::Error>
    {
        let (_,R) = self.hessenberg::<X>()?;
        let det = R.cross(self)?
            .diagonal()?
            .into_iter()
            .fold(T::one(), |acc,t| acc*t);
        Ok(det)
    }

    fn eigen_values<Y>(&self) -> Result<Vec<T>, Self::Error>
    where 
        Y: VectorAlgebra<T>,
        Self::Error: From<Y::Error>
    {
        let (_,R) = self.hessenberg::<Y>()?;
        let eigen = R.cross(self)?
            .diagonal()?;
        Ok(eigen)
    }
}

// default impl picks up the complex case.
impl<T: QuantumReal> MatrixAlgebra<T> for Matrix<T>
{
    /* 
    any matrix m x n can be decomposed into the product of Q: orthoganl matrix && and R: upper right triangle
    input: matrix
    process: 
        1.  let the Householder H = I - (2/v^T v)vv^T, 
            where v = u / u1 
            u = a + sign(a1) ∥a∥2 e1,
            a = col vector,
            e1 = vector::[1, 0 .. 0]^T,
        2. applying H to the Matrix A zeroes the sub-diagonal elements for the col.
        3. the alg. needs to move onto the next col without disrupting the previous calculations.
        4. take the minor of the matrix at the required position.
        5. reapply H.
    */

    fn hessenberg<W>(&self) -> Result<(Self,Self),Self::Error> 
    where 
        W: VectorAlgebra<T>,
        Self::Error: From<W::Error>
    {
        Ok( real_qr_decomposition::<T,Self,W>(self)? )
    } 
} 

impl MatrixAlgebra<Complex<f64>> for Matrix<Complex<f64>>
{
    fn hessenberg<W>(&self) -> Result<(Self,Self),Self::Error> 
    where 
        W: VectorAlgebra<Complex<f64>>,
        Self::Error: From<W::Error>
    {
       Ok(complex_qr_decomposition::<f64,Self,W>(self)?)
    } 
}

impl MatrixAlgebra<Complex<f32>> for Matrix<Complex<f32>>
{
    fn hessenberg<W>(&self) -> Result<(Self,Self),Self::Error> 
    where 
        W: VectorAlgebra<Complex<f32>>,
        Self::Error: From<W::Error>
    {
        Ok(complex_qr_decomposition::<f32,Self,W>(self)?)
    } 
}

impl<T:QuantumUnit> ComplexMatrixAlgebra<T> for ComplexMatrix<T>
where
    Self: MatrixAlgebra<Complex<T>>,
    Complex<T>: QuantumUnit
{
    fn complex_conjugate(self) -> Self
    {
        Self::from(
            self.permute_rows()
            .map(|c| c.conj() )
            .collect::<Vec<_>>()
        )
    }

    fn hermitian_conjugate(self) -> Self
    {
        Self::from(self.permute_cols()
            .map(|c| c.conj() )
            .collect::<Vec<_>>()   
        )
    }
}

impl<T> From<Vec<T>> for Matrix<T>
{
    fn from(inner: Vec<T>) -> Self {
        Self {
            dim: inner.len().sqrt(),
            inner: inner
        }
    }
}

fn real_qr_decomposition<T,M,V>(A: &M) -> Result<(M,M), M::Error>
where
    T: QuantumReal,
    M: MatrixAlgebra<T>,
    V: VectorAlgebra<T> + From<M::Inner>,
    M::Error: From<V::Error>
{
    let mut M: M = A.clone();
    let mut _Q: Vec<M> = Vec::new();
    let mut _R: Vec<M> = Vec::new();

    // for k in 0..M.dim() {
    for k in 0..M.dim()-1 {
        let x: V = M.extract_col(k)?.into();
        let alpha: T = x.get(k+1)?
            .signum()
            .mul(x.eucl_norm());
        let epsilon: V = {
            let mut _e = vec![T::zero(); M.dim()];
            _e[k]=T::one();
            _e.into()
        };
        let mu: V = x.subtraction(epsilon.scalar(alpha));
        let mu_norm: T = mu.eucl_norm();
        let I = M.identity()?;
        let vvT: M = mu.clone().kronecker(mu);
        let Qk: M = I.subtraction(vvT.scalar( (T::one()+T::one()).div(mu_norm*mu_norm) ));

        let mut Q = Qk.cross(&M)?;
        for i in 0..Q.dim() {
            Q.set(k,i,T::zero())?;
            Q.set(i,k,T::zero())?;
        }

        _R.push(Qk.clone());
        _Q.push(Qk.transpose());
        
        Q.set(k,k,T::one())?;
        M = Q;
    }
    let R: M = _R.into_iter()
        .rev()
        .fold(M.identity()?, |acc,q| acc.cross(&q).unwrap());       
    let Q: M = _Q.into_iter()
        .fold(M.identity()?, |acc,q| acc.cross(&q).unwrap());
    Ok((Q,R))
}

use num_traits::identities::{ One, Zero};

fn complex_qr_decomposition<T,M,V>(A: &M) -> Result<(M,M),M::Error>
where
    T: num_traits::Float,
    M: MatrixAlgebra<Complex<T>>,
    M::Error: From<V::Error>,
    V: VectorAlgebra<Complex<T>>
    + From<M::Inner>,
{
    let mut M: M = A.clone();
    let mut _Q: Vec<M> = Vec::new();
    let mut _R: Vec<M> = Vec::new();

    // for k in 0..M.dim() {
    for k in 0..M.dim()-1 {
        let x: V = M.extract_col(k)?.into();
        let arg: T = x.get(k+1)?.arg();
        let alpha: Complex<T> = -( Complex::<T>::i() * arg ).exp() * x.eucl_norm();
        let epsilon: V = {
            let mut _e = vec![Complex::zero(); M.dim()];
            _e[k]=Complex::one();
            _e.into()
        };
        let mu: V = x.subtraction(epsilon.scalar(alpha));
        let mu_norm: Complex<T> = mu.eucl_norm();
        let I = M.identity()?;
        let vvT: M = mu.clone().kronecker(mu);
        let Qk: M = I.subtraction(vvT.scalar( (Complex::<T>::one()+Complex::one()).div(mu_norm*mu_norm) ));

        let mut Q = Qk.cross(&M)?;
        for i in 0..Q.dim() {
            Q.set(k,i,Complex::zero())?;
            Q.set(i,k,Complex::zero())?;
        }

        _R.push(Qk.clone());
        _Q.push(Qk);
        
        Q.set(k,k,Complex::one())?;
        M = Q;
    }
    let R: M = _R.into_iter()
        .rev()
        .fold(M.identity()?, |acc,q| acc.cross(&q).unwrap());       
    let Q: M = _Q.into_iter()
        .fold(M.identity()?, |acc,q| acc.cross(&q.transpose()).unwrap());
    Ok((Q,R))
}

fn francis_double_step<T,M,V>(A: &M) -> Result<M,M::Error>
where
    T: QuantumReal + std::fmt::Debug,
    M: MatrixAlgebra<T>,
    M::Error: From<V::Error>,
    V: VectorAlgebra<T>
    + From<M::Inner>,
{
    use std::ops::RangeInclusive;

    let (Q,R) = real_qr_decomposition::<T,M,V>(A)?;
    let mut H = R.cross(A)?.cross(&Q)?;
    let p = A.dim();
    let n = A.dim();

    let reflect = |P: V, alpha: RangeInclusive<usize>, beta: RangeInclusive<usize>| -> Result<(),M::Error> 
    {
        let phi = Vec::new();
        for i in alpha {
            for j in beta {
                phi.push(
                    H.get(i,j)?
                )
            }
        }
        let rho = P.matrix_product(phi.into())?
            .into_iter()
            .collect::<Vec<_>>();
        let mut l = 0;
        for i in alpha {
            for j in beta {
                let delta = rho[l];
                H.set(i,j,delta)?;
            }
        }
        Ok(())
    };

    // reduce all indexing by 1. !!!!!!!!!!!!!!!!!!!!
    while p > 1 
    {
        let q: usize = p.sub(1);
        let s: T = H.get(q,q)?
            .add(H.get(p,p)?);
        let t: T = H.get(q,q)?
            .mul(H.get(p,p)?)
            .sub( H.get(q,p)?.mul(H.get(p,q)?) );
        let mut x: T = H.get(0,0)?
            .pow64(2.0)
            .add( H.get(0,1)?.mul( H.get(1,0)? ) )
            .sub(s.mul(H.get(0,0)?))
            .add(t);
        let mut y: T = H.get(1,0)?
            .mul( H.get(0,0)?.add(H.get(1,1)?).sub(s) );
        let mut z: T = H.get(1,0)?
            .mul(H.get(2,1)?);
        let mut hh_reflector: V = vec![x,y,z].into();
        let mut givens_reflector: V = vec![x,y].into();
        for k in 0..p.sub(4)
        {
            if k > 1 { 
                reflect(hh_reflector, k.add(1)..=k.add(3), k..=n )? 
            } else {
                reflect(hh_reflector, 2..=4, 1..=n)?
            };

            if k.add(4) < p { 
                reflect(hh_reflector, 1..=k, k.add(1)..=k.add(3) )?
            } else {
                reflect(hh_reflector, 1..=p, p.add(1)..=p.add(3) )?
            };

            x = H.get(k.add(2),k.add(1))?;
            y = H.get(k.add(3), k.add(1))?;
            if k < p.sub(3) { 
                z = H.get(k.add(4), k.add(1))?; 
            }
        }
        reflect(givens_reflector, q..=p, p.sub(2)..=n)?;

        // if the enclosure accepts a function this can be cleaner.
        let phi = Vec::new();
        for i in 1..=p {
            for j in p.sub(1)..p {
                phi.push(
                    H.get(i,j)?
                )
            }
        }
        let rho = H.vector_product(givens_reflector)?
            .into_iter()
            .collect::<Vec<_>>();
        let mut l = 0;
        for i in 1..=p {
            for j in p.sub(1)..p {
                let delta = rho[l];
                H.set(i,j,delta)?;
            }
        }
        // There's some convergence checking here.
    };
    Ok(H)
}