use crate::math::{ Vector, VectorAlgebra, QuantumUnit };
use super::QuantumBit;
use std::ops::Mul;

#[derive(Debug,PartialEq)]
pub struct Qubit<T>
{
    state: Vector<T>
}

impl<T:QuantumUnit> QuantumBit<T> for Qubit<T>
{
    type Inner = Vector<T>;
    type Error = std::io::Error;

    fn into_inner(self) -> Vector<T> { self.state }
}

impl<T> From<Vector<T>> for Qubit<T>
{
    fn from(state: Vector<T>) -> Self { Self{state} }
}

impl<T> From<Vec<T>> for Qubit<T>
{
    fn from(inner: Vec<T>) -> Self {
        Self {
            state: Vector::from(inner)
        }
    }
}

impl<T: QuantumUnit> Mul<T> for Qubit<T>
{
    type Output = Self;
    fn mul(self, rhs: T) -> Self
    {
        Self::from(self.state*rhs)
    }
}

impl<T:QuantumUnit> Mul<Self> for Qubit<T>
{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self
    {
        Self::from(self.state.tensor(rhs.state))
    }
}

pub struct Ket<T>
where
    Vector<T>: VectorAlgebra<T>
{
    inner: Vector<T>
}