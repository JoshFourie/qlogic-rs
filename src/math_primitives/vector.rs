/***** Imports ********/
use std::ops::Mul;
use std::result::Result;
use num::{ Complex, Zero };
use super::{QuantumUnit, ComplexVectorAlgebra, MatrixAlgebra, VectorAlgebra};
use super::error::VectorError;

/***** Structs ********/

#[derive(Debug, PartialEq, Clone)]
pub struct Vector<T> { inner: Vec<T> }

/***** Impls ********/

impl<T:QuantumUnit> VectorAlgebra<T> for Vector<T>
{
    type Inner = Vec<T>;
    type Error = VectorError;

    fn apply_to_each<F: Fn(T)->T>(self, action: F) -> Self
    {
        Self::from(
            self.into_iter()
                .map(|x| action(x) )
                .collect::<Vec<_>>()
        )
    }

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
        let mut m=Vec::new();
        for i in 0..self.len() {
            for j in 0..rhs.len() {
                m.push(self.inner[i]*rhs.inner[j])
            }
        }
        m.into()
    }

    // to construct a register we need a tensor that builds a vector,
    // but for general numops we require a tensor that also builds M.
    fn kronecker<M: MatrixAlgebra<T>>(self, rhs: Self) -> M 
    {
        let mut m=Vec::new();
        for i in 0..self.len() {
            for j in 0..rhs.len() {
                m.push(self.inner[i]*rhs.inner[j])
            }
        }
        m.into()
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

    fn eucl_norm(&self) -> T  
    { 
        self.inner.iter()
            .fold(T::zero(), |acc,x| acc + x.pow64(2.0))
            .sqroot()
    }

    fn addition(self, rhs: Self) -> Self
    {
        self.into_iter().zip(rhs.into_iter())
            .map(|(l,r)| l+r )
            .collect::<Vec<_>>()
            .into()
    }

    fn scalar(self, rhs: T) -> Self { self.apply_to_each(|x| x*rhs) }

}

impl VectorAlgebra<Complex<f32>> for Vector<Complex<f32>>
{
    fn dot(self, rhs: Self) -> Complex<f32>
    {
        self.into_iter().zip(rhs.into_iter())
            .fold(Complex::zero(), |acc,(a,b): (Complex<f32>,Complex<f32>)| acc + a.conj()*b)
    }
}

impl VectorAlgebra<Complex<f64>> for Vector<Complex<f64>>
{
    fn dot(self, rhs: Self) -> Complex<f64>
    {
        self.into_iter().zip(rhs.into_iter())
            .fold(Complex::zero(), |acc,(a,b): (Complex<f64>,Complex<f64>)| acc + a.conj()*b)
    }
}

impl ComplexVectorAlgebra for Vector<Complex<f32>>
{    
    fn conjugate_transpose(self) -> Self { self.apply_to_each(|x| x.conj()) }
}

impl ComplexVectorAlgebra for Vector<Complex<f64>>
{    
    fn conjugate_transpose(self) -> Self { self.apply_to_each(|x| x.conj()) }
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