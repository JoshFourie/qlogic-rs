// We only consider square matrices for quantum mechanics.

/***** Imports ********/

use num_integer::Roots;
use std::ops::{Div, Rem};
use num::Complex;
use super::{ QuantumUnit, QuantumScalar, ComplexMatrixAlgebra, VectorAlgebra, MatrixAlgebra };
use super::error::MatrixError;

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
    fn extract_row(&self, r: usize) -> Self::Inner
    {
        let mut v: Vec<T> = Vec::new();
        for c in 0..self.dim() {
            let val = self.get(r,c).unwrap();
            v.push(val)
        }
        v
    }

    // pull out a copy of a col at a given index.
    fn extract_col(&self, c: usize) -> Self::Inner
    {
        let mut v: Vec<T> = Vec::new();
        for r in 0..self.dim() {
            let val = self.get(r,c).unwrap();
            v.push(val)
        }
        v
    }

    // retrieve value at an index. May fail.
    fn get(&self, row: usize, col: usize) -> Result<T, MatrixError>
    {
        let index = row*self.dim +col;
        match index < self.inner.len()
        {
            true => Ok(self.inner[index]),
            false => MatrixError::invalid_index(row, col, self.dim).as_result(),
        }   
    }

    // set a value at an index. May fail.
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


    // vector_product to allow us to skip a for loop.
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

    // returns the diagonal of the Matrix.
    fn diagonal(&self) -> Self::Inner
    {
        let mut d = Vec::new();
        for j in 0..self.dim {
            d.push(self.get(j, j).unwrap())
        }
        d
    }

    // trace is the sum of the diagonals.
    fn trace(self) -> T
    {
        let mut sigma = T::zero();
        for val in self.diagonal()
            .into_iter()
        {
            sigma += val;
        }
        sigma
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

    /* fn eigen_value(self) -> T
    {
        
    } */
 
    fn apply_to_each<F: Fn(T)->T>(self, action: F) -> Self
    {
        self.into_iter()
            .map(|x| action(x))
            .collect::<Vec<_>>()
            .into()
    }

    fn minor(&mut self, m: &Self, d: usize)
    {
        for i in 0..d { self.set(i,i,T::one()).unwrap(); }
        for i in d..m.dim() {
            for j in d..m.dim() {
                let x = m.get(i,j).unwrap();
                self.set(i,j,x).unwrap(); 
            }
        }
    }

    fn identity(&self) -> Self
    {
        let mut id: Self = vec![T::zero(); self.dim*self.dim].into();
        for j in 0..self.dim {
            id.set(j,j,T::one()).unwrap()
        }
        id
    }    

    // TODO
    default fn decompose<W>(&self) -> Self 
    where
        W: VectorAlgebra<T>
    {
        vec![T::zero(); self.dim()].into()
    }
}

// default impl picks up the complex case.
impl<T: QuantumScalar> MatrixAlgebra<T> for Matrix<T>
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

    fn decompose<W>(&self) -> Self
    where 
        W: VectorAlgebra<T>,
    {
        // we will rip away apart when we delete cols and rows.
        let mut A: Self = self.clone();

        // B is our matrix that we are returning.
        let mut B: Vec<T> = Vec::new();// vec![T::zero(); self.dim()*self.dim()];

        // we are operating on every column.
        // we need a new iteration each time because we are working with a different matrix.
        for i in 0..self.dim() {
            //  pulling out the a1, a and e1 values.
            let alpha: W = self.extract_col(i).into();
            let mut _e = vec![T::zero(); self.dim()];
            _e[0] = T::one();
            let e1: W = _e.into();
            let a1 = alpha.get(0).unwrap();
            // building u
            let norm = alpha.eucl_norm();
            let mu = alpha.addition(e1.scalar(norm * a1.signum()));
            let mu1 = mu.eucl_norm();
            // transforming u -> V.
            let phi = mu.apply_to_each(|c| c.div(mu1)) ;
            // pulling together to generate H. We don't need to transpose because it is meaningless for 
            // the machine when we are calculating the outer product of a vector
            let H: Self = A
                .identity()
                .subtraction( phi
                    .clone()
                    .kronecker::<Self>(phi)
                    .scalar(T::one()+T::one())
                );
            // we need to apply H to A and permute through the rows of the minor.    
            let mut b = vec![T::zero(); i];
            B.append(&mut b);
            B.append( &mut H.cross(A.clone()).into_inner() );
        } 
        // B.transpose()
        Self::from(B).transpose()
    } 
} 

impl<T:QuantumUnit> ComplexMatrixAlgebra for ComplexMatrix<T>
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

pub fn qr_decomposition<M,T,V>(A: &M) -> (M,M)
where
    T: QuantumScalar + std::fmt::Debug,
    M: MatrixAlgebra<T> + std::fmt::Debug,
    V: VectorAlgebra<T> + std::fmt::Debug
    + From<M::Inner>
{
    let mut M: M = A.clone();
    let mut _Q: Vec<M> = Vec::new();
    let mut _R: Vec<M> = Vec::new();

    // for k in 0..M.dim() {
    for k in 0..M.dim()-1 {
            
            println!("M: 
            {:?}", M);
        let x: V = M.extract_col(k).into();
        let alpha: T = x.get(k+1).unwrap().signum() * x.eucl_norm();
            println!("alpha: 
            {:?}", alpha);
        let epsilon: V = {
            let mut _e = vec![T::zero(); M.dim()];
            _e[k]=T::one();
            _e.into()
        };
            println!("epsilon: 
            {:?}", epsilon);
            println!("x: 
            {:?}", x);
        let mu: V = x.subtraction(epsilon.scalar(alpha));
            println!("mu: 
            {:?}", mu);
        let mu_norm: T = mu.eucl_norm();
            println!("mu_norm: 
            {:?}", mu_norm);
        // let phi: V = mu.apply_to_each(|u| u.div(mu_norm));
        let I = M.identity();
            println!("I: 
            {:?}", I);
        let vvT: M = mu.clone().kronecker(mu);
            println!("vvT: 
            {:?}", vvT);
        let Qk: M = I.subtraction(vvT.scalar( (T::one()+T::one()).div(mu_norm*mu_norm) ));             
            println!("Q1: 
            {:?}", Qk);
        _R.push(Qk.clone());
        _Q.push(Qk.clone().transpose());
              
        
        let mut Q = Qk.cross(M.clone());
            println!("Q: 
            {:?}", Q);
        // correct until we pull the minor.

        for i in 0..Q.dim() {
            Q.set(k,i,T::zero()).unwrap();
            Q.set(i,k,T::zero()).unwrap();
        }
        Q.set(k,k,T::one()).unwrap();
        M = Q;
            println!("M: 
            {:?}",M);
            println!("
            
            
            ");
    }
    // } 
    /* let k=1;
            println!("M: 
            {:?}", M);
        let x: V = M.extract_col(k).into();
        let alpha: T =   x.eucl_norm();
            println!("alpha: 
            {:?}", alpha);
        let epsilon: V = {
            let mut _e = vec![T::zero(); M.dim()];
            _e[k]=T::one();
            _e.into()
        };
            println!("epsilon: 
            {:?}", epsilon);
            println!("x: 
            {:?}", x);
        let mu: V = x.subtraction(epsilon.scalar(alpha));
            println!("mu: 
            {:?}", mu);
        let mu_norm: T = mu.eucl_norm();
            println!("mu_norm: 
            {:?}", mu_norm);
        // let phi: V = mu.apply_to_each(|u| u.div(mu_norm));
        let I = M.identity();
            println!("I: 
            {:?}", I);
        let vvT: M = mu.clone().kronecker(mu);
            println!("vvT: 
            {:?}", vvT);
        let Qk: M = I.subtraction(vvT.scalar( (T::one()+T::one()).div(mu_norm*mu_norm) ));             
            println!("Q2: 
            {:?}", Qk);
        _R.push(Qk.clone());
        _Q.push(Qk.clone().transpose());


        let mut Q = Qk.cross(M.clone());
            println!("Q: 
            {:?}", Q);
        // correct until we pull the minor.
        for i in 0..Q.dim() {
            Q.set(k,i,T::zero()).unwrap();
            Q.set(i,k,T::zero()).unwrap();
        }
        Q.set(k,k,T::one()).unwrap();
        M = Q;
            println!("M: 
            {:?}",M);
            println!("
            
            
            ");
    */ 

    let R: M = _R.into_iter()
        .rev()
        .fold(M.identity(), |acc,q| acc.cross(q));       
        println!("R: 
        {:?}", R);

    let Q: M = _Q.into_iter()
        .fold(M.identity(), |acc,q| acc.cross(q));
        println!("Q: 
        {:?}", Q);
        
    (Q,R)
}

#[test]
fn test_decomp()
{
    let M: Matrix<f32> = vec![12.0, -51.0, 4.0, 6.0, 167.0, -68.0, -4.0, 24.0, -41.0].into();
    let (Q,R): (Matrix<f32>, Matrix<f32>) = qr_decomposition::<Matrix<f32>, f32, super::Vector<f32>>(&M);
    let A: Matrix<f32> =  vec![14.0, 21.0, -14.0, 0.0, 175.0, -70.0, 0.0, 0.0, -35.0].into();
    // let test = Q.cross(R);
    
    assert_eq!(A,R.cross(M));
}