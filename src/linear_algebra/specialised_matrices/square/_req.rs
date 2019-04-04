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
            dim: Some( v.len().sqrt() ),
            inner: v
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
