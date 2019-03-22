/***** Exports ********/

pub mod matrix;
pub mod vector;
pub mod error;

pub use matrix::{ Matrix, ComplexMatrix };
pub use vector::{ Vector };

/***** Interfaces ********/
pub trait QuantumUnit: num::Num
    + std::ops::Neg<Output=Self>
    + std::ops::AddAssign
    + Copy
{
    // these are not implemented as a trait in Rust.
    fn pow64(self, rhs: f64) -> Self;
    fn sqroot(self) -> Self;
}

// trait for scalar units that have traits like PartialOrd<Self> & Signum
// the QuantumUnit is only useful for allowing specialisation.
pub trait QuantumReal: QuantumUnit + num_traits::real::Real { }

pub trait MatrixAlgebra<T>
where
    Self: From<Vec<T>>
    + IntoIterator<Item=T>
    + Clone,
{
    type Inner;
    type Error: std::fmt::Debug;

    fn dim(&self) -> usize;

    fn update_dim(&mut self);

    fn into_inner(self) -> Self::Inner;

    fn push(&mut self, val: T);

    fn permute_rows(self) -> std::vec::IntoIter<T>;

    fn permute_cols(self) -> std::vec::IntoIter<T>;

    fn apply_to_each<F: Fn(T)->T>(self, action: F) -> Self;

    fn extract_row(&self, r: usize) -> Result<Self::Inner,Self::Error>;

    fn extract_col(&self, c: usize) -> Result<Self::Inner,Self::Error>;

    fn get(&self, row:usize, col:usize) -> Result<T,Self::Error>;

    fn set(&mut self, row:usize, col:usize, val:T) -> Result<(),Self::Error>;

    fn transpose(self) -> Self;

    fn kronecker(&self, rhs: &Self) -> Result<Self,Self::Error>;

    fn scalar(self, rhs: T) -> Self;

    fn cross(&self, rhs: &Self) -> Result<Self,Self::Error>;

    fn vector_product<V: VectorAlgebra<T>>(self, rhs: V) -> Result<V,Self::Error>
    where
        Self::Error: From<V::Error>;

    fn eigen_values<Y>(&self) -> Result<Vec<T>, Self::Error>
    where 
        Y: VectorAlgebra<T>,
        Self::Error: From<Y::Error>;

    fn identity(&self) -> Result<Self,Self::Error>;

    fn trace(self) -> Result<T,Self::Error>;

    fn diagonal(&self) -> Result<Self::Inner,Self::Error>;

    fn addition(self, rhs: Self) -> Self;

    fn subtraction(self, rhs: Self) -> Self;

    fn hessenberg<W>(&self) -> Result<(Self,Self),Self::Error>
    where 
        W: VectorAlgebra<T>,
        Self::Error: From<W::Error>;
    
    fn determinant<X>(&self) -> Result<T,Self::Error>
    where 
        X: VectorAlgebra<T>,
        Self::Error: From<X::Error>;

    // fn destructor(self);
}

pub trait ComplexMatrixAlgebra<T>: MatrixAlgebra<num::Complex<T>>
{   
    fn complex_conjugate(self) -> Self;

    fn hermitian_conjugate(self) -> Self;
}

pub trait VectorAlgebra<T>
where
    Self: From<Vec<T>>
    + IntoIterator<Item=T>
    + Clone,
{
    type Inner;
    type Error: std::fmt::Debug;

    fn into_inner(self) -> Self::Inner;

    fn apply_to_each<F: Fn(T)->T>(self, action: F) -> Self;

    fn push(&mut self, val:T);

    fn len(&self) -> usize;

    fn get(&self, index:usize) -> Result<T,Self::Error>;

    fn dot(self, rhs:Self) -> T;

    fn tensor(self,rhs:Self) -> Self;

    fn kronecker<M: MatrixAlgebra<T>>(self, rhs: Self) -> M;

    fn addition(self, rhs: Self) -> Self;

    fn subtraction(self, rhs: Self) -> Self;
    
    fn scalar(self, rhs: T) -> Self;

    fn matrix_product<M: MatrixAlgebra<T>>(self, rhs: M) -> Result<Self,M::Error>
    where
        M::Error: From<Self::Error>;

    fn eucl_norm(&self) -> T;
}

pub trait ComplexVectorAlgebra<T>: VectorAlgebra<T>
{
    fn hermitian_conjugate(self) -> Self;
}

/***** Impls ********/
impl QuantumUnit for isize { 
    fn pow64(self, rhs: f64) -> Self { self.pow(rhs as u32) }
    fn sqroot(self) -> Self { (self as f64).sqrt() as isize }
}
impl QuantumUnit for f32 {
    fn pow64(self, rhs: f64) -> Self { self.powf(rhs as  f32) }     
    fn sqroot(self) -> Self { self.sqrt() }
}
impl QuantumUnit for f64 {     
    fn pow64(self, rhs: f64) -> Self { self.powf(rhs) }     
    fn sqroot(self) -> Self { self.sqrt() }
}

impl QuantumReal for f32 { }
impl QuantumReal for f64 { }

impl QuantumUnit for num::Complex<f32> {
    fn pow64(self, rhs: f64) -> Self { self.powf(rhs as f32) }     
    fn sqroot(self) -> Self { self.sqrt() }
}
impl QuantumUnit for num::Complex<f64> {
    fn pow64(self, rhs: f64) -> Self { self.powf(rhs) }     
    fn sqroot(self) -> Self { self.sqrt() }
}