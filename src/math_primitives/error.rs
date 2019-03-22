/***** Imports ********/

use std::fmt::Debug;

/***** Enums ********/

#[derive(Debug)]
pub enum MatrixError
{
    InvalidIndex(String),
    InvalidDimension(String),
    Specialisation(String),
    VectorInteraction(String),
    Other(VectorError)
}

#[derive(Debug)]
pub enum VectorError
{
    Multiplication(String),
    InvalidIndex(String),
    Computational(String),
    MatrixInteraction(String),
    Other(String)
}

/***** Impls ********/
impl MatrixError
{

    pub fn as_result<T>(self) -> Result<T, Self> { Err(self) } 

    pub fn invalid_index<T: Debug>(row:T, col:T, len:T) -> Self { MatrixError::InvalidIndex(format!("index ({:?},{:?}) exceeds max dimension of ({:?},{:?})", row, col, len, len)) }
    
    pub fn invalid_dimension<T: Debug>(dim1:T, dim2:T) -> Self { MatrixError::InvalidDimension(format!("cannot multiply inequal dimensions: {:?} != {:?}", dim1, dim2)) }

    pub fn specialisation<T: Debug>(msg: T) -> Self { MatrixError::Specialisation(format!("There was a specialisation error whilst {:?}",msg)) }

}

impl VectorError
{
    pub fn as_result<T>(self) -> Result<T,Self> { Err(self) }
    
    pub fn invalid_index<T: Debug>(index:T, len:T) -> Self { VectorError::InvalidIndex(format!("index ({:?}) exceeds length of vector ({:?})", index, len)) }

    pub fn computational(err: &'static str) -> Self { VectorError::Computational(format!("Computational error: { }", err)) }
}

impl From<String> for MatrixError 
{
    fn from(e: String) -> Self
    {
        MatrixError::Other(e.into())
    }
}

impl<T> From<T> for VectorError 
where
    String: From<T>
{
    fn from(e: T) -> Self
    {
        VectorError::Other(e.into())
    }
}

impl From<VectorError> for MatrixError
{
    fn from(e: VectorError) -> Self
    {
        MatrixError::Other(e)
    }
}