pub mod hessenberg;

use hessenberg::*;
use super::*;

impl<T: Copy + Debug> CoreMatrix<T> for HessenbergMatrix<T>
where
    T: Num
{
    type Error = MathError;

    // BAD FUNCTIONO
    fn dim(&self) -> Option<usize> { self.dim }

    fn col_dim(&self) -> Option<usize> { self.dim }

    fn row_dim(&self) -> Option<usize> { self.dim }

    fn update(self, row: Option<usize>, col: Option<usize>) -> Result<Self,Self::Error>
    { 
        let mut N: Self = self.into_inner().into();
        match (row,col) {
            (Some(_), None) => { N.dim = row },
            (None, Some(_)) => { N.dim = col },
            (None, None) => { N.dim = Some(self.into_inner().len().pow(1/2)) }
            (Some(r), Some(c)) => {
                if r==c { 
                    N.dim = Some(r.mul(c));
                } else { 
                    return MathError::bad_op("Invalid Dimensions: rows must be equivalent to cols when force-updating a square matrix.").as_result() 
                }
            }
        }
        Ok(N)
    }

    fn into_inner(&self) -> Vec<T> { self.inner.clone() }

    fn push(&mut self, val: T) { self.inner.push(val); }

    fn get(&self, row: Option<usize>, col: Option<usize>) -> Result<T,Self::Error>
    {
        let index = row?
            .mul(self.dim?)
            .add(col?);
        Ok(self.inner[index])
    }

    fn set(&mut self, row: Option<usize> , col: Option<usize>, val:T) -> Result<(),Self::Error>
    {
        let index = row?
            .mul(self.dim?)
            .add(col?);
        self.inner[index] = val;
        Ok(())
    }
}

impl<T: Num + Copy + Debug> BasicTransform<T> for HessenbergMatrix<T> { }


// TEMP UNWRAP()
impl<T> From<HessenbergMatrix<T>> for Matrix<T>
{
    fn from(sq: HessenbergMatrix<T>) -> Self 
    {
        Self {
            col: sq.dim,
            row: sq.dim,
            dim: Some( sq.dim.unwrap().mul(sq.dim.unwrap()) ),
            inner: sq.inner,   
        }
    }
}

impl<'a, T: Clone> From<&'a HessenbergMatrix<T>> for Matrix<T>
{
    fn from(sq: &'a HessenbergMatrix<T>) -> Self 
    {
        Self {
            col: sq.dim,
            row: sq.dim,
            dim: Some( sq.dim.unwrap().mul(sq.dim.unwrap()) ),
            inner: sq.inner.clone(),   
        }
    }
} 

impl<T> From<Vec<T>> for HessenbergMatrix<T>
{
    fn from(v: Vec<T>) -> Self {
        Self {
            dim: Some( v.len().pow(1/2) ),
            inner: v
        }
    }
}

impl<T> From<Matrix<T>> for HessenbergMatrix<T>
{
    fn from(mat: Matrix<T>) -> Self
    {
        let row = mat.row
            .expect("temp err on From<Matrix> for HessenbergMatrix: None on row value.");
        let col = mat.col
            .expect("temp err on From<Matrix> for HessenbergMatrix: None on col value.");
        assert_eq!(row, col);
        Self {
            dim: mat.dim,
            inner: mat.inner
        }
    }
}


impl<T: Copy> IntoIterator for HessenbergMatrix<T>
{
    type Item = T;
    type IntoIter = MatrixIter<Matrix<T>>;
    fn into_iter(self) -> Self::IntoIter
    {
        Matrix::from(self).into_iter()
    }
}

impl <'a, T: Copy> IntoIterator for &'a HessenbergMatrix<T>
{
    type Item = T;
    type IntoIter = MatrixIter<Matrix<T>>;
    fn into_iter(self) -> Self::IntoIter
    {
        Matrix::from(self).into_iter()
    }
}
