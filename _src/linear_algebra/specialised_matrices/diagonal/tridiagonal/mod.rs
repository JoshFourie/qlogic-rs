pub mod tridiagonal;
use tridiagonal::*;
use super::*;

// Tridiagonal can be optimised for ignoring the zeros.
impl<T: Copy + Debug + Num> CoreMatrix<T> for TridiagonalMatrix<T> 
where
    Self: Square<T>
{
    type Error = MathError;

    fn dim(&self) -> Option<usize> { self.dim }

    fn col_dim(&self) -> Option<usize> { self.col }

    fn row_dim(&self) -> Option<usize> { self.row }

    fn into_inner(&self) -> Vec<T> 
    {
        self.inner.clone()
        /* self.sup
            .into_iter()
            .chain(self.main.into_iter())
            .chain(self.sub.into_iter())
            .collect() */
    }

    fn update(self, row: Option<usize>, col: Option<usize>) -> Result<Self,Self::Error>
    {
        let mut N: Self = Vec::new().into();
        match row == col {
            true => {
                N.row = row;
                N.col = col;
                N.dim= Some(row?.mul(col?));
                N.inner = self.inner;
                Ok(N)
            },
            false => panic!("temp err on CoreMatrix<T> impl for TridiagonalMatrix<T>: row non-equivalent to column on force-update.")
        }
    }   
    
    fn push(&mut self, val: T) -> Result<&Self, Self::Error> { 
        self.inner.push(val); 
        Ok(self)
    }

    fn get(&self, row: Option<usize>, col: Option<usize>) -> Result<T,Self::Error>
    {
        if self.row? > row? && self.col? > col? 
        {
            let i = row?
                .mul(self.col_dim()?)
                .add(col?);
            return Ok(self.inner[i])
        } else {
            // MathError::invalid_index(row?, col?, self.row?, self.col?).as_result() 
            panic!("temp err on TridiagonalMatrix<T>::get(): BadIndex. (MathError needs to be assoc. type.)")
        }
    }

    fn set(&mut self, row: Option<usize> , col: Option<usize>, val:T) -> Result<(),Self::Error>
    {
        let i = row?
            .mul(self.col_dim()?)
            .add(col?);
                self.inner[i] = val;
        Ok(())
    }
}

impl<T: Copy + Debug + Num> BasicTransform<T> for TridiagonalMatrix<T> { }

// assume the zeroes are NOT part of the input.
impl<T: Copy + Debug> From<Vec<T>> for TridiagonalMatrix<T>
where
    for <'a> &'a Self: IntoIterator<Item=T>,
    Self: Square<T>,
    T: Num
{
    fn from(vec: Vec<T>) -> Self
    {
        // vec![1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16]
        /*****************
            6 1 0 0 0 0
            12 7 2 0 0 0
            0 13 8 3 0 0
            0 0 14 9 4 0
            0 0 0 15 10 5
            0 0 0 0 16 11
        *******************/
        let alpha = vec.len().pow(1/2);
        /* let mut gamma: (Vec<T>, Vec<T>, Vec<T>) = {
            let mut main: Vec<T> = Vec::new();
            let mut sub: Vec<T> = Vec::new();
            let mut sup: Vec<T> = Vec::new();

        }; */
        Self {
            row: Some(alpha),
            col: Some(alpha),
            dim: Some(vec.len()),
            inner: vec,
            /* sup: Vec::new(),
            sub: Vec::new(),
            main: Vec::new(), */
        }
    }
}

impl<T> From<Matrix<T>> for TridiagonalMatrix<T>
{
    fn from(mat: Matrix<T>) -> Self
    {
        let r = mat.row
            .expect("temp err on From<Matrix<T>> for Tridiagonal (square) matrix: called mat.row on a None.");
        let c = mat.col
            .expect("temp err on From<Matrix<T>> for Tridiagonal (square) matrix: called mat.col on a None.");
        let dim = match r==c {
            true => Some(r.mul(c)),
            false => panic!("temp err on From<Matrix<T>> for Tridiagonal (square) matrix: called row.mul(col) on non-equivalent values.")
        };
        Self {
            row: Some(r), 
            col: Some(c),
            dim: dim,
            inner: mat.inner,
        }
    }
}

// assume zeroes are stored in inner.
impl<T> From<TridiagonalMatrix<T>> for Matrix<T> 
{
    fn from(tri: TridiagonalMatrix<T>) -> Self
    {
        Self {
            col: tri.col,
            row: tri.row,
            dim: tri.dim,
            inner: tri.inner,
        }
    }
}

impl<'a, T: Copy> From<&'a TridiagonalMatrix<T>> for Matrix<T>
{
    fn from(tri: &'a TridiagonalMatrix<T>) -> Self 
    {
        Self {
            col: tri.col,
            row: tri.row,
            dim: tri.dim,
            inner: tri.inner.clone(),
        }
    }
}

impl<T: Copy> IntoIterator for TridiagonalMatrix<T>
{
    type Item = T;
    type IntoIter = MatrixIter<Matrix<T>>;
    fn into_iter(self) -> Self::IntoIter
    {
        Matrix::from(self).into_iter()
    }
}

impl <'a, T: Copy> IntoIterator for &'a TridiagonalMatrix<T>
{
    type Item = T;
    type IntoIter = MatrixIter<Matrix<T>>;
    fn into_iter(self) -> Self::IntoIter
    {
        Matrix::from(self).into_iter()
    }
}