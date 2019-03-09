use std::ops::{ Mul, AddAssign, Add };
use std::collections::HashMap;
use std::result::Result;
use num::Zero;
use num_integer::Roots;
use super::vector::{ Vector };

// TODO: 
// [ ] impl iterator for Matrix
//      > ran into difficulties with the hashmap
//      > solved with Deref but f32/64 do not impl the trait

#[derive(Debug)]
pub enum MatrixErr
{
    Multiplication(MatrixMulErr),
}

#[derive(Debug)]
pub enum MatrixMulErr
{
    Dimensions
}

pub trait Matrices<T>
{
    fn kronecker_product(self, rhs: Self) -> Self;
}

impl<T: Copy> Matrices<T> for Matrix<T> 
where
    T: Mul<Self,Output=Self>
{
    // (A(x)B)i,j = a|(i-1/p)|+1,|(j-1)/q|+1 b(i-1)%p+1, (j-1)%q+1
    // where |..| repr. floor
    // % repr. a remainder op
    // a & b repr. elements in A and B
    // i & j repr. element positions in the output
    // p & q repr. dimensions of B
    fn kronecker_product(self, rhs: Self) -> Self
    {
               
    }
}

pub type MatrixInner<T> = HashMap<(usize,usize), T>;

#[derive(Debug, PartialEq, Clone)]
pub struct Matrix<T>
{
    pub inner: MatrixInner<T>,
    pub dim: usize,
}

impl<T: Copy> Matrix<T>
{
    pub fn new_empty(dim: usize) -> Self { Self { inner: MatrixInner::new(), dim: dim  } }

    pub fn new_with_inner(data: Vec<T>) -> Self 
    {
        let dim=data.len().sqrt();
        let mut m = Self::new_empty( dim );   
        for i in 0..dim
        {
            for j in 0..dim
            {
                m.inner.insert((i, j), data[i*dim+j]);
            }
        }
        m.dim = dim;
        m
    }

    fn permute_rows<'r>(self) -> Vec<&'r T>
    {
        let mut rows = Vec::new();
        for i in 0..self.dim
        {
            for j in 0..self.dim
            {
                rows.push( self.inner.get( &(i,j)).unwrap() );
            }
        }
        rows
    }

    fn permute_cols<'q>(self) -> Vec<&'q T>
    {
        let mut cols = Vec::new();
        for i in 0..self.dim
        {
            for j in 0..self.dim
            {
                cols.push( self.inner.get( &(i,j) ).unwrap() );
            }
        }   
        cols
    }

    fn dimensions_check(&self, rhs: &Self) -> Result<(),MatrixErr>
    {
        match self.dim==rhs.dim
        {
            true =>  { },
            false => return Err( MatrixErr::Multiplication( MatrixMulErr::Dimensions ) )
        };
        match self.inner.len()==self.dim*self.dim
        {
            true => { },
            false => return Err( MatrixErr::Multiplication( MatrixMulErr::Dimensions ) )
        };
        match self.inner.len()== rhs.inner.len()
        {
            true => { },
            false => return Err( MatrixErr::Multiplication( MatrixMulErr::Dimensions ) )
        };
        Ok(())
    }
}

impl<T> Add<Self> for Matrix<T>
where
    for <'d>
    &'d T: Add<&'d T,Output=T>,
    T: Copy
{
    type Output=Result<Self,MatrixErr>;
    
    fn add(self, rhs: Self) -> Self::Output
    {
        match self.dimensions_check(&rhs)
        {
            Ok(_) => {
                let dim=self.dim;
                let mut c = Self::new_empty(dim);
                for i in 0..dim
                {
                    for j in 0..dim
                    {
                        let val = self.inner.get(&(i,j)).unwrap()+rhs.inner.get(&(i,j)).unwrap();
                        c.inner.insert((i,j), val);
                    }
                }
                Ok(c)
            },
            Err(e) => Err(e),
        }
    }
}

impl<T: Copy> Mul<T> for Matrix<T>
where   
    T: Mul<T,Output=T>
{
    type Output=Self;
    fn mul(self, rhs: T) -> Self
    {
        let mut c = Self::new_empty(self.dim);
        c.inner = self.inner.into_iter()
            .map(|((i, i2), x)| ((i, i2), rhs*x))
            .collect::<MatrixInner<T>>();
        c
    }
}

impl<T: Copy> Mul<&T> for Matrix<T>
where   
    for <'k> 
    &'k T: Mul<T,Output=T>
{
    type Output=Self;
    fn mul(self, rhs: &T) -> Self
    {
        let mut c = Self::new_empty(self.dim);
        c.inner = self.inner.into_iter()
            .map(|((i, i2), x)| ((i, i2), rhs*x))
            .collect::<MatrixInner<T>>();
        c
    }
}

// O(n^3) matrix multiplication.
impl<T> Mul<Self> for Matrix<T>
where
    for <'a> 
    &'a T: Mul<&'a T, Output=T>,
    T: Copy
    + Zero
    + AddAssign
{
    type Output = Result<Self, MatrixErr>;

    fn mul(self, rhs: Self) -> Self::Output
    {   
        match self.dimensions_check(&rhs)
        {
            Ok(_) => {
                let dim=self.dim;
                let mut c = Self::new_empty(dim);
                for i in 0..dim
                {
                    for j in 0..dim
                    { 
                        let mut val = T::zero();
                        for k in 0..dim
                        {
                            val += self.inner.get( &(i,k) ).unwrap() * rhs.inner.get( &(k,j) ).unwrap();
                        }
                        c.inner.insert( (i,j), val );
                    } 
                }
                Ok(c)
            },
            Err(e) => Err(e)
        }

    }
}

impl<T> Mul<Vector<T>> for Matrix<T>
where
    for <'b>
    &'b T: Mul<&'b T,Output=T>,
    T: Zero
    + AddAssign,
{
    type Output = Result<Vector<T>, MatrixErr>;

    fn mul(self, rhs: Vector<T>) -> Self::Output
    {
        let mut c: Vector<T> = Vector::new_empty();
        for i in 0..self.dim
        {
            let mut val = T::zero();
            for k in 0..self.dim
            {
                val += self.inner.get( &(i,k) ).unwrap() * &rhs.inner[k];
            }
            c.inner.push(val); 
        }
        Ok(c)
    }
}

#[cfg(test)]
mod tests
{
    use num::Complex;
    use super::*;
    #[test]
    fn test_matrix_dot_self() 
    {

        let inner = vec![ 1.0, 2.0, 3.0, 4.0, 1.0, 2.0, 3.0, 4.0, 1.0 ].into_iter()
            .map(|n| Complex::from(n))
            .collect::<Vec<_>>();
        let A = Matrix::new_with_inner(inner);
                
        let inner = vec![ 1.0, 2.0, 3.0, 4.0, 1.0, 2.0, 3.0, 4.0, 1.0 ].into_iter()
            .map(|n| Complex::from(n))
            .collect::<Vec<_>>();        
        let B = Matrix::new_with_inner(inner);

        let inner = vec![ 18.0, 16.0, 10.0, 14.0, 17.0, 16.0, 22.0, 14.0, 18.0 ].into_iter()
            .map(|n| Complex::from(n))
            .collect::<Vec<_>>();
            
        let exp = Matrix::new_with_inner(inner);

        assert_eq!(exp, (B*A).unwrap());
    }
}