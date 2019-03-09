impl<T: Deref> IntoIterator for &mut Matrix<T>
{
    type Item = T;
    type IntoIter = MatrixIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter
    {
        MatrixIter {
            inner: self.inner,
            row: 0,
            col: 0,
            dim: self.dim
        }
    }
}

impl<T: Deref> IntoIterator for &Matrix<T>
{
    type Item = T;
    type IntoIter = MatrixIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter
    {
        MatrixIter {
            inner: self.inner,
            row: 0,
            col: 0,
            dim: self.dim
        }
    }
}

impl<T: Deref> IntoIterator for Matrix<T>
{
    type Item = T;
    type IntoIter = MatrixIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter
    {
        MatrixIter {
            inner: self.inner,
            row: 0,
            col: 0,
            dim: self.dim
        }
    }
}

struct MatrixIter<T>
{
    inner: MatrixInner<T>, 
    row: usize,
    col: usize,
    dim: usize,
}

impl<T: Deref> Iterator for MatrixIter<T>
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item>
    {
        match self.row==self.dim {
            true => {
                self.row+=1;
                self.col=0;
            },
            false => self.col+=1;
        }
        match self.inner.get( &(self.row, self.col) )
        {
            Some(i) => Some(*i),
            _ => None,
        }        
    }
} 


#[test]
fn test_matrix_iterator()
{
    let inner = vec![ 18.0, 16.0, 10.0, 14.0, 17.0, 16.0, 22.0, 14.0, 18.0 ];
    let exp = Matrix::new_with_inner(inner);
    let inner = vec![ 18.0, 16.0, 10.0, 14.0, 17.0, 16.0, 22.0, 14.0, 18.0 ];
    let test = Matrix::new_with_inner(inner);

    test.into_iter()
}