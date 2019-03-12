use std::ops::Mul;
use std::result::Result;
use num::Complex;
use crate::math_primitives::interface::{QuantumUnit, MatrixAlgebra, ComplexVectorAlgebra, VectorAlgebra};
use crate::math_primitives::error::VectorError;
/* 
Provided: 
[x] Dot product
[x] Inner product
[x] Tensor product
*/

#[derive(Debug, PartialEq)]
pub struct Vector<T> { inner: Vec<T> }

impl<T:QuantumUnit> VectorAlgebra<T> for Vector<T>
{
    type Inner = Vec<T>;
    type Error = VectorError;

    fn into_inner(self) -> Vec<T> { self.inner } 

    fn push(&mut self, val: T) { self.inner.push(val); }

    fn len(&self) -> usize { self.inner.len() }

    fn get(&self, index: usize) -> Result<T,VectorError> 
    {
        match index < self.inner.len()
        {
            true => Ok(self.inner[index]), 
            false => VectorError::invalid_index(index, self.inner.len()).as_result(),
        }  
    }

    default fn dot(self,rhs:Self) -> T
    {
        self.into_iter()
            .zip(rhs.into_iter())
            .fold(T::zero(),|acc,(a,b)| acc+(a*b))
    }

    fn tensor(self,rhs:Self) -> Self
    {
        let mut scratch=Vec::new();
        for i in 0..self.len() {
            for j in 0..rhs.len() {
                scratch.push(self.inner[i]*rhs.inner[j])
            }
        }
        Self::from(scratch)
    }

    fn matrix_product<M: MatrixAlgebra<T>>(self, rhs: M) -> Self
    {
        let mut new = Self::from(Vec::new());
        for i in 0..rhs.dim() {
            let mut sigma = T::zero();
            for k in 0..rhs.dim() {
                let aik = rhs.get(i,k).unwrap();
                let b = self.get(k).unwrap();
                sigma += aik*b;
            }
            new.push(sigma);
        }
        new
    }
}

impl<T:QuantumUnit> ComplexVectorAlgebra<T> for Vector<Complex<T>>
{
    fn dot(self,rhs:Self) -> Complex<T>
    {
        self.into_iter()
            .zip(rhs.into_iter())
            .map(|(a,b)| a.conj()*b)
            .sum()
    }
}

impl<T> From<Vec<T>> for Vector<T> 
{
    fn from(inner: Vec<T>) -> Self { Self{inner} }
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