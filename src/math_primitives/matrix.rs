use num_integer::Roots;
use std::fmt::Debug;
use std::ops::{ Mul, AddAssign, Div, Rem };
use num::{ Float, Zero };

#[derive(Debug)]
enum MatrixError
{
    InvalidIndex(String),
    InvalidDimension(String),
}

impl MatrixError
{
    fn as_result<T>(self) -> Result<T, Self> { Err(self) } 
    fn invalid_index<T: Debug>(row:T, col:T, len:T) -> Self { MatrixError::InvalidIndex(format!("index ({:?},{:?}) exceeds max dimension of ({:?},{:?})", row, col, len, len)) }
    fn invalid_dimension<T: Debug>(dim1:T, dim2:T) -> Self { MatrixError::InvalidDimension(format!("cannot multiply inequal dimensions: {:?} != {:?}", dim1, dim2)) }

}

// We only consider square matrices.
#[derive(Debug, PartialEq)]
struct Matrix<T>
{
    inner: Vec<T>,
    dim: usize,
}

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

impl<T: Copy> Matrix<T>
where
    T: Mul<T,Output=T>
{
    fn new(inner: Vec<T>) -> Self 
    {
        Self {
            dim: inner.len().sqrt(),
            inner: inner,
        }
    }

    fn push(&mut self, val: T) {
        self.inner.push(val);
    }

    // row-major permutation.
    fn permute_rows(self) -> std::vec::IntoIter<T>
    {
        self.into_iter()
    }

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
    fn kronecker_product(self, rhs: Self) -> Self
    {
        let mut new = Self::new(Vec::new());
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
}

impl<T: Copy> Mul<T> for Matrix<T>
where
    T: Mul<T,Output=T>
{
    type Output=Self;
    fn mul(self, rhs: T) -> Self
    {
        Self {
            dim: self.dim,
            inner: self.into_iter()
                .map(|val| val*rhs)
                .collect::<Vec<T>>()            
        }
    }
}

impl<T: Copy> Mul<Self> for Matrix<T>
where   
    T: Mul<T,Output=T>
    + AddAssign
    + Zero
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self
    {
        assert_eq!(self.dim,rhs.dim);
        let len = self.dim;
        let mut new = Self::new(Vec::new());
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
}

#[cfg(test)]
mod tests 
{
    //                                  0 1 2 
    //  0, 1, 2, 3, 4, 5, 6, 7, 8 -->   3 4 5 
    //                                  6 7 8 

    use super::*;

    #[test]
    fn test_column_permutation()
    {
        let exp = vec![0, 3, 6, 1, 4, 7, 2, 5, 8].into_iter();
        let test = Matrix::new(vec![0, 1, 2, 3, 4, 5, 6, 7, 8]).permute_cols();
        for (exp, test) in exp.into_iter()
            .zip( test )
        {
            assert_eq!(exp, test);
        }
    }

    #[test]
    fn test_matrix_get()
    {   
        let test = Matrix::new(vec![0, 1, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(test.get(1,1).unwrap(), 4);
        assert_eq!(test.get(1,2).unwrap(), 5);
        assert_eq!(test.get(2,1).unwrap(), 7);
        match test.get(2,8) {
            Err(_) => { },
            _ => panic!("MatrixError was not returned as expected")
        }
    }

    #[test]
    fn test_scalar_mul()
    {
        let test = Matrix::new(vec![0, 1, 2, 3, 4, 5, 6, 7, 8]);
        let exp = Matrix::new(vec![0, 3, 6, 9, 12, 15, 18, 21, 24]);
        assert_eq!(test*3, exp);
    }

    #[test]
    fn test_matrix_dot_product()
    {
        let test = Matrix::new(vec![0, 1, 2, 3, 4, 5, 6, 7, 8])*Matrix::new(vec![0, 1, 2, 3, 4, 5, 6, 7, 8]);
        let exp = Matrix::new(vec![15, 18, 21, 42, 54, 66, 69, 90, 111]);
        assert_eq!(test, exp);
    }

    #[test]
    fn test_kronecker_product()
    {
        let test = Matrix::new(vec![1,2,3,4]).kronecker_product(Matrix::new(vec![0,5,6,7]));
        let exp = Matrix::new(vec![0,5,0,10,6,7,12,14,0,15,0,20,18,21,24,28]);
        assert_eq!(test,exp);
    }
}