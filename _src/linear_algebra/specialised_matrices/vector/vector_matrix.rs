use super::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Vector<T>
{
    pub(crate) inner: Vec<T>,
    pub(crate) len: Option<usize>,
}

impl<T> From<Vec<T>> for Vector<T>
{
    fn from(x: Vec<T>) -> Self {
        Self {
            len: Some(x.len()),
            inner: x
        }
    }
}

impl<T> From<Vector<T>> for Matrix<T>
{
    fn from(v: Vector<T>) -> Self 
    {
        Self {
            col: Some(1),
            row: Some(v.inner.len()),
            dim: Some(v.inner.len()),
            inner: v.inner,   
        }
    }
}

impl<'a, T: Clone> From<&'a Vector<T>> for Matrix<T>
{
    fn from(v: &'a Vector<T>) -> Self
    {
        return Matrix {
            col: Some(1),
            row: Some(v.inner.len()),
            dim: Some(v.inner.len()),
            inner: v.inner.clone(),   
        }
    }
}

impl<T: Copy> IntoIterator for Vector<T>
{
    type Item = T;
    type IntoIter = MatrixIter<Matrix<T>>;
    fn into_iter(self) -> Self::IntoIter
    {
        Matrix::from(self).into_iter()
    }
}

impl <'a, T: Copy> IntoIterator for &'a Vector<T>
{
    type Item = T;
    type IntoIter = MatrixIter<Matrix<T>>;
    fn into_iter(self) -> Self::IntoIter
    {
        Matrix::from(self).into_iter()
    }
}


impl<T: Copy + Debug> CoreMatrix<T> for Vector<T>
where
    T: Num
{
    type Error = MathError;

    fn dim(&self) -> Option<usize> { self.len }

    fn col_dim(&self) -> Option<usize> { Some(1) }

    fn row_dim(&self) -> Option<usize> { self.len }

    // Should probably be an Option on the col.
    fn update(self, row: Option<usize>, col: Option<usize>) -> Result<Self,Self::Error>
    { 
        match (row, col) {
            (None, None) => Ok( self.inner.into() ),
            _ => MathError::bad_op("Invalid update: cannot over-ride the auto-generated dimensions of a Vector").as_result()
        }
    }

    fn into_inner(&self) -> Vec<T> { self.inner.clone() }

    
    fn push(&mut self, val: T) -> Result<&Self, Self::Error> { 
        self.inner.push(val); 
        Ok(self)
    }

    fn get(&self, row: Option<usize>, col: Option<usize>) -> Result<T,Self::Error>
    {
        match (row, col) 
        {
            (Some(index), None) => Ok(self.inner[index]),
            (None, Some(index)) => Ok(self.inner[index]),
            _ => MathError::invalid_index(row?, col?, self.len?, 1).as_result(), 
        }
        
    }

    fn set(&mut self, row: Option<usize>, col: Option<usize>, val:T) -> Result<(),Self::Error>
    {
        match (row, col) 
        {
            (Some(index), None) => {
                self.inner[index] = val;
                Ok(())
            },
            (None, Some(index)) => {
                self.inner[index] = val;
                Ok(())
            },
            _ => Err(MathError::invalid_index(row?, col?, self.len?, 1)), 
        }
    }
}

impl<T: Copy + Debug> BasicTransform<T> for Vector<T>
where
    T: Num
{
    fn get_sub_matrix(
        &self, 
        alpha: Option<Range<usize>>, 
        beta: Option<Range<usize>>
    ) -> Result<Self, Self::Error>
    {
        match beta {
            None => {
                let mut A: Self = Vec::new().into();
                for i in alpha? {
                    A.push(self.get(Some(i), None)?)?;
                }
                return Ok(A)
            },
            Some(_) => MathError::bad_input("Col. index of a Vector should be a None, you have called Some({usize}).").as_result(),
        }
        
    }

    fn set_sub_matrix(        
        &mut self,
        alpha: Option<Range<usize>>, 
        beta: Option<Range<usize>>,
        delta: Vec<T>
    ) -> Result<(), Self::Error>
    {
        match beta {
            None => {
                let mut l: usize = 0;
                for i in alpha? {
                    for j in beta.clone()? {
                        self.set(Some(i), Some(j), delta[l])?;
                        l += 1;
                    }
                }
                Ok(())
            },
            Some(_) => MathError::bad_input("Col. index of a Vector should be a None, you have called Some({usize}).").as_result(),
        }
    }
}