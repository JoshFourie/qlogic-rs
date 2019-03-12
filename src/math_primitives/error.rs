use std::fmt::Debug;

#[derive(Debug)]
pub enum MatrixError
{
    InvalidIndex(String),
    InvalidDimension(String),
}

impl MatrixError
{
    pub fn as_result<T>(self) -> Result<T, Self> { Err(self) } 

    pub fn invalid_index<T: Debug>(row:T, col:T, len:T) -> Self { MatrixError::InvalidIndex(format!("index ({:?},{:?}) exceeds max dimension of ({:?},{:?})", row, col, len, len)) }
    
    pub fn invalid_dimension<T: Debug>(dim1:T, dim2:T) -> Self { MatrixError::InvalidDimension(format!("cannot multiply inequal dimensions: {:?} != {:?}", dim1, dim2)) }

}


#[derive(Debug)]
pub enum VectorError
{
    Multiplication(String),
    InvalidIndex(String),
}

impl VectorError
{
    pub fn as_result<T>(self) -> Result<T,Self> { Err(self) }
    
    pub fn invalid_index<T: Debug>(index:T, len:T) -> Self { VectorError::InvalidIndex(format!("index ({:?}) exceeds length of vector ({:?})", index, len)) }
}
