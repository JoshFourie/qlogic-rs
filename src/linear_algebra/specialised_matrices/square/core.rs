
#[derive(Debug, PartialEq, Clone)]
pub struct SquareMatrix<T>
{
    pub(crate) inner: Vec<T>,
    pub(crate) dim: Option<usize>,
}

impl<T: QuantumUnit> CoreMatrix<T> for SquareMatrix<T>
{
    type Error = MathError;

    fn dim(&self) -> Option<usize> { self.dim }

    fn col_dim(&self) -> Option<usize> { self.dim }

    fn row_dim(&self) -> Option<usize> { self.dim }

    fn update(self, row: Option<usize>, col: Option<usize>) -> Result<Self,Self::Error>
    { 
        let mut N: Self = self.into_inner().into();
        match (row,col) {
            (Some(_), None) => { N.dim = row; },
            (None, Some(_)) => { N.dim = col; },
            (None, None) => { N.dim = Some(self.into_inner().len().sqrt()) }
            (Some(r), Some(c)) => {
                if r==c { 
                    N.dim = row 
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

impl<T: QuantumUnit> BasicTransform<T> for SquareMatrix<T> { }

impl<T: QuantumReal> EigenValueDecomposition<T> for SquareMatrix<T> 
{
    fn decomposition(&self) -> Result<(Self,Self),Self::Error> 
    {
        super::eigen::real_hessenberg(self)
    }
}