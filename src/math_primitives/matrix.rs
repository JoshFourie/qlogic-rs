use num_integer::Roots;
use std::ops::{Div, Rem};
use num::Complex;
use crate::math_primitives::interface::{ QuantumUnit, ComplexMatrixAlgebra, VectorAlgebra, MatrixAlgebra };
use crate::math_primitives::error::MatrixError;


/************** WARNING!!! *******************/
// We only consider square matrices for quantum mechanics.
#[derive(Debug, PartialEq, Clone)]
pub struct Matrix<T>
{
    inner: Vec<T>,
    dim: usize,
}

pub type ComplexMatrix<T> = Matrix<Complex<T>>;

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

    fn dim(&self) -> usize { self.dim }

    fn into_inner(self) -> Vec<T> { self.inner } 

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

    fn get(&self, row: usize, col: usize) -> Result<T, MatrixError>
    {
        let index = row*self.dim +col;
        match index < self.inner.len()
        {
            true => Ok(self.inner[index]),
            false => MatrixError::invalid_index(row, col, self.dim).as_result(),
        }   
    }

    fn set(&mut self, row: usize, col: usize, val: T) -> Result<(), MatrixError>
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
    fn kronecker(self, rhs: Self) -> Self
    {
        let mut new = Self::from(Vec::new());
        let dim = self.dim*rhs.dim;
        let pq =rhs.dim as f32;
        for i in 1..=dim {
            let i = i as f32;
            for j in 1..=dim {
                let j = j as f32;              
                let a = self.get(
                    (f32::floor( (i-1.0).div(pq)+1.0 )) as usize -1,
                    (f32::floor( (j-1.0).div(pq)+1.0 )) as usize -1,
                ).unwrap();
                let b = rhs.get(
                    ((i-1.0).rem(pq)+1.0) as usize -1,
                    ((j-1.0).rem(pq)+1.0) as usize -1,
                ).unwrap();
                new.push(a*b);
            }
        }
        new.dim=dim;
        new
    }

    fn transpose(self) -> Self
    {
        Self::from(
            self.permute_cols().collect::<Vec<_>>()
        )
    }

    fn scalar(self, rhs: T) -> Self
    {
        Self::from( 
            self.into_iter()
            .map(|val| val*rhs)
            .collect::<Vec<T>>()   
        )
    }

    fn cross(self, rhs: Self) -> Self
    {
        assert_eq!(self.dim,rhs.dim);
        let len = self.dim;
        let mut new = Self::from(Vec::new());
        for i in 0..len {
            for j in 0..len {
                let mut sigma = T::zero();
                for k in 0..len
                {
                    let aik = self.get(i,k).unwrap();
                    let bkj = rhs.get(k,j).unwrap(); 
                    sigma += aik*bkj;
                }
            new.push(sigma)
            } 
        }
        new.dim = len;
        new
    }

    fn vector_product<V: VectorAlgebra<T>>(self, rhs: V) -> V
    {
        let mut new = V::from(Vec::new());
        for i in 0..self.dim {
            let mut sigma = T::zero();
            for k in 0..self.dim {
                let aik = self.get(i,k).unwrap();
                let b = rhs.get(k).unwrap();
                sigma += aik*b;
            }
            new.push(sigma);
        }
        new
    } 
}
 
impl<T:QuantumUnit> ComplexMatrixAlgebra for ComplexMatrix<T>
where
    Self: MatrixAlgebra<Complex<T>>,
{
    fn complex_conjugate(self) -> Self
    {
        Self::from(self.into_iter()
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