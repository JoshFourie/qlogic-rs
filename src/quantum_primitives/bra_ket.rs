use crate::math::{ 
    QuantumUnit, Vector, ComplexVectorAlgebra, 
    VectorAlgebra, error::MatrixError
};
use std::ops::Mul;

/***** Interface ****/ 

pub trait Qubit<T>
{
    fn into_inner(self) -> Vector<T>;   
}

/**** Enum *****/

pub enum BraKetError
{
    LinearAlgebra(MatrixError)
}

/****** Structs *****/

pub struct Bra<T: QuantumUnit> 
{ 
    inner: Vector<T> 
}

pub struct Ket<T: QuantumUnit> { inner: Vector<T> }

/****** Impls ******/

impl<T: QuantumUnit> Qubit<T> for Bra<T>
{
    fn into_inner(self) -> Vector<T> { self.inner }
}

impl<T: QuantumUnit> Qubit<T> for Ket<T>
{
    fn into_inner(self) -> Vector<T> { self.inner }
}

/***** Traits *******/

// std::convert::From;
impl<T: QuantumUnit> From<Vec<T>> for Bra<T> { fn from(v: Vec<T> )-> Self { Self{ inner: v.into() }}}

impl<T: QuantumUnit> From<Vec<T>> for Ket<T> { fn from(v: Vec<T> )-> Self { Self{ inner: v.into() }}}

impl<T: QuantumUnit> From<Vector<T>> for Bra<T> { fn from(v: Vector<T>) -> Self { Self{ inner: v }}}

impl<T: QuantumUnit> From<Vector<T>> for Ket<T> { fn from(v: Vector<T>) -> Self { Self{ inner: v }}}

impl<T: QuantumUnit> From<Bra<T>> for Ket<T>
where
    Vector<T>: ComplexVectorAlgebra<T>
{ 
    fn from(v: Bra<T>) -> Self { v.into_inner().hermitian_conjugate().into() }
}

impl<T: QuantumUnit> From<Ket<T>> for Bra<T> 
where
    Vector<T>: ComplexVectorAlgebra<T>
{ 
    fn from(v: Ket<T>) -> Self { v.into_inner().hermitian_conjugate().into() }
}

// std::ops::Mul;
impl<T: QuantumUnit> Mul<T> for Bra<T>
{
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output
    {
        self.into_inner()
            .scalar(rhs)
            .into()
    }
}

impl<T: QuantumUnit> Mul<Self> for Ket<T>
{
    type Output = T;
    fn mul(self, rhs: Self) -> Self::Output
    {
        self.into_inner().dot(rhs.into_inner())
    }
}

impl<T: QuantumUnit> Mul<Self> for Bra<T>
{
    type Output = T;
    fn mul(self, rhs: Self) -> Self::Output
    {
        self.into_inner().dot(rhs.into_inner())
    }
}

impl<T: QuantumUnit> Mul<Ket<T>> for Bra<T>
where
    Vector<T>: ComplexVectorAlgebra<T>
{
    type Output = T;
    fn mul(self, ket: Ket<T>) -> Self::Output
    {
        self.into_inner()
            .hermitian_conjugate()
            .dot( ket.into_inner() )
    }
}