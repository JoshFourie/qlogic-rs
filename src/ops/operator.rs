use crate::math::{ Matrix, QuantumUnit, MatrixAlgebra, VectorAlgebra };
use crate::qubit::{ Qubit, QuantumBit };
use std::ops::Mul;

#[derive(Debug,PartialEq)]
pub struct Operator<T: Copy> 
where
    Matrix<T>: MatrixAlgebra<T>
{
    inner: Matrix<T>
}

impl<T: Copy> Operator<T>
where
    T: QuantumUnit
{
    pub fn map_into<Q:QuantumUnit>(self) -> Operator<Q>
    where
        Q: From<T>
    {
        Operator::<Q>::from(
            self.into_iter()
                .map(|x| Q::from(x))
                .collect::<Vec<_>>()
        )
    }

    pub fn into_inner(self) -> Matrix<T> { self.inner }

    pub fn tensor(self, rhs: Self) -> Self
    {
        Self::from( self.inner.kronecker(rhs.inner) )
    }
}

impl<T:QuantumUnit> Mul<Qubit<T>> for Operator<T>
where
    Matrix<T>: MatrixAlgebra<T>,
{
    type Output = Qubit<T>;
    fn mul(self, rhs: Qubit<T>) -> Qubit<T>
    {
        Qubit::from(self.inner.vector_product(rhs.into_inner()))
    }
}

impl<T:QuantumUnit> Mul<T> for Operator<T>
where
    Matrix<T>: MatrixAlgebra<T>,
{
    type Output = Operator<T>;
    fn mul(self, rhs: T) -> Self::Output
    {
        Operator::from(self.inner.scalar(rhs))
    }
}

impl<T:QuantumUnit> Mul<Self> for Operator<T>
where
    Matrix<T>: MatrixAlgebra<T>
{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output
    {
        Operator::from( self.inner.cross(rhs.inner) )
    }
}

impl<T:QuantumUnit> From<Vec<T>> for Operator<T>
{
    fn from(inner: Vec<T>) -> Self
    {
        Operator {
            inner: Matrix::from(inner)
        }
    }
}

impl<T:QuantumUnit> From<Matrix<T>> for Operator<T>
{
    fn from(inner: Matrix<T>) -> Self
    {
        Operator { inner }
    }
}

// row major permutation of inner matrix.
impl<T:QuantumUnit> IntoIterator for Operator<T>
{
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter
    {
        self.inner.into_iter()
    }
}
