/***** Exports ********/

pub mod matrix;
pub mod vector;
pub mod error;

pub use matrix::{ Matrix, ComplexMatrix };
pub use vector::{ Vector };

/***** Interfaces ********/

pub trait QuantumUnit: std::fmt::Debug 
    + std::ops::Mul<Self,Output=Self>
    + std::ops::Div<Self,Output=Self>
    + std::ops::Add<Self,Output=Self>
    + std::ops::Sub<Self,Output=Self>
    + std::ops::Neg<Output=Self>
    + std::ops::AddAssign
    + std::ops::MulAssign
    + num::Zero
    + num::One 
    + num::Num
    + Copy
    + Clone
{
    // convenience supertrait 
}

pub trait MatrixAlgebra<T>
where
    Self: From<Vec<T>>
{
    type Inner;
    type Error: std::fmt::Debug;

    fn dim(&self) -> usize;

    fn into_inner(self) -> Self::Inner;

    fn push(&mut self, val: T);

    fn permute_rows(self) -> std::vec::IntoIter<T>;

    fn permute_cols(self) -> std::vec::IntoIter<T>;

    fn get(&self, row:usize, col:usize) -> Result<T,Self::Error>;

    fn set(&mut self, row:usize, col:usize, val:T) -> Result<(),Self::Error>;

    fn transpose(self) -> Self;

    fn kronecker(self, rhs:Self) -> Self;

    fn scalar(self, rhs: T) -> Self;

    fn cross(self, rhs: Self) -> Self;

    fn vector_product<V: VectorAlgebra<T>>(self, rhs: V) -> V;

    fn eigen_value(self) -> T;

    fn identity(self) -> Self;

    fn trace(self) -> T;

    fn diagonal(&self) -> Self::Inner;
}

pub trait ComplexMatrixAlgebra
{
    
    fn complex_conjugate(self) -> Self;

    fn hermitian_conjugate(self) -> Self;
}

pub trait VectorAlgebra<T>
where
    Self: From<Vec<T>>
{
    type Inner;
    type Error: std::fmt::Debug;

    fn into_inner(self) -> Self::Inner;

    fn push(&mut self, val:T);

    fn len(&self) -> usize;

    fn get(&self, index:usize) -> Result<T,Self::Error>;

    fn dot(self, rhs:Self) -> T;

    fn tensor(self,rhs:Self) -> Self;

    fn matrix_product<M: MatrixAlgebra<T>>(self, rhs: M) -> Self;
}

pub trait ComplexVectorAlgebra<T>
{
    fn dot(self,rhs:Self) -> num::Complex<T>;
}

/***** Impls ********/

impl QuantumUnit for isize { }
impl QuantumUnit for f32 { }
impl QuantumUnit for f64 { }
impl QuantumUnit for num::Complex<f32> { }
impl QuantumUnit for num::Complex<f64> { }