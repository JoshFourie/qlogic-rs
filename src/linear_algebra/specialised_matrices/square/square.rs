use super::*;

impl<T: Copy + Debug> CoreMatrix<T> for SquareMatrix<T>
where
    T: Num
{
    type Error = MathError;

    fn dim(&self) -> Option<usize> {
        Some(self.dim?.mul(self.dim?))
    }

    fn col_dim(&self) -> Option<usize> { self.dim }

    fn row_dim(&self) -> Option<usize> { self.dim }

    // BAD FUNCTION
    fn update(self, row: Option<usize>, col: Option<usize>) -> Result<Self,Self::Error>
    { 
        let mut N: Self = self.into_inner().into();
        match (row,col) {
            (Some(_), None) => { N.dim = row },
            (None, Some(_)) => { N.dim = col },
            (None, None) => { N.dim = Some(self.into_inner().len().pow(1/2)) }
            (Some(r), Some(c)) => {
                if r==c { 
                    N.dim = Some(r);
                } else { 
                    return MathError::bad_op("Invalid Dimensions: rows must be equivalent to cols when force-updating a square matrix.").as_result() 
                }
            }
        }
        Ok(N)
    }

    fn into_inner(&self) -> Vec<T> { self.inner.clone() }
    
    fn push(&mut self, val: T) -> Result<&Self, Self::Error> { 
        self.inner.push(val); 
        Ok(self)
    }

    fn get(&self, row: Option<usize>, col: Option<usize>) -> Result<T,Self::Error>
    {
        let index = row?
            .mul(self.col_dim()?)
            .add(col?);
        Ok(self.inner[index])
    }

    fn set(&mut self, row: Option<usize> , col: Option<usize>, val:T) -> Result<(),Self::Error>
    {
        let index = row?
            .mul(self.col_dim()?)
            .add(col?);
        self.inner[index] = val;
        Ok(())
    }
}

impl<T: Num + Copy + Debug> BasicTransform<T> for SquareMatrix<T> { }

impl<T: Copy + Debug> EigenValueDecomposition<T> for SquareMatrix<T> 
where 
    T: Float + Signed
{
    fn decomposition(&self) -> Result<(Self,Self),Self::Error> 
    {
        super::eigen::qr_transform(self)
    }
}

// TEMP UNWRAP()
impl<T> From<SquareMatrix<T>> for Matrix<T>
{
    fn from(sq: SquareMatrix<T>) -> Self 
    {
        Self {
            col: sq.dim,
            row: sq.dim,
            dim: Some( sq.dim.unwrap().mul(sq.dim.unwrap()) ),
            inner: sq.inner,   
        }
    }
}

impl<'a, T: Clone> From<&'a SquareMatrix<T>> for Matrix<T>
{
    fn from(sq: &'a SquareMatrix<T>) -> Self 
    {
        Self {
            col: sq.dim,
            row: sq.dim,
            dim: Some( sq.dim.unwrap().mul(sq.dim.unwrap()) ),
            inner: sq.inner.clone(),   
        }
    }
} 

impl<T> From<Vec<T>> for SquareMatrix<T>
{
    fn from(v: Vec<T>) -> Self {
        Self {
            dim: Some( (v.len() as f64).powf(0.5) as usize  ),
            inner: v
        }
    }
}

impl<T> From<Matrix<T>> for SquareMatrix<T>
{
    fn from(mat: Matrix<T>) -> Self
    {
        let row = mat.row
            .expect("temp err on From<Matrix> for SquareMatrix: None on row value.");
        let col = mat.col
            .expect("temp err on From<Matrix> for SquareMatrix: None on col value.");
        assert_eq!(row, col);
        Self {
            dim: mat.col,
            inner: mat.inner
        }
    }
}


impl<T: Copy> IntoIterator for SquareMatrix<T>
{
    type Item = T;
    type IntoIter = MatrixIter<Matrix<T>>;
    fn into_iter(self) -> Self::IntoIter
    {
        Matrix::from(self).into_iter()
    }
}

impl <'a, T: Copy> IntoIterator for &'a SquareMatrix<T>
{
    type Item = T;
    type IntoIter = MatrixIter<Matrix<T>>;
    fn into_iter(self) -> Self::IntoIter
    {
        Matrix::from(self).into_iter()
    }
}
