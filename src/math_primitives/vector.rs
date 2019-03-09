use std::ops::Mul;
use std::fmt::Debug;
use std::result::Result;

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

#[derive(Debug, PartialEq)]
pub struct Vector<T> { inner: Vec<T> }

impl<T: Copy> Vector<T>
{
    pub fn new(inner: Vec<T>) -> Self { Self{inner} }

    pub fn push(&mut self, val: T) { self.inner.push(val); }

    pub fn get(&self, index: usize) -> Result<T,VectorError> 
    {
        match index < self.inner.len()
        {
            true => Ok(self.inner[index]), 
            false => VectorError::invalid_index(index, self.inner.len()).as_result(),
        }  
    }
}

impl<T> IntoIterator for Vector<T>
{
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter
    {
        self.inner.into_iter()
    }
}

impl<T: Copy> Mul<T> for Vector<T>
where
    T: Mul<T,Output=T>
{
    type Output=Self;
    fn mul(self, rhs: T) -> Self
    {
        Self{ 
            inner: self.into_iter()
                .map(|x| rhs*x)
                .collect::<Vec<_>>() 
        }
    } 
}