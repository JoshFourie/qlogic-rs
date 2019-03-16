/***** Exports ********/

pub mod matrix;
pub mod vector;
pub mod error;
pub mod eigen;

pub use matrix::{ Matrix, ComplexMatrix };
pub use vector::{ Vector };

/***** Interfaces ********/
pub trait QuantumUnit: num::Num
    + std::ops::Neg<Output=Self>
    + std::ops::AddAssign
    + Copy
{
    fn pow64(self, rhs: f64) -> Self;
    fn sqroot(self) -> Self;
}

// trait for scalar units that have traits like PartialOrd<Self> & Signum
pub trait QuantumScalar: QuantumUnit + num_traits::real::Real { }

pub trait MatrixAlgebra<T>
where
    Self: From<Vec<T>>
    + IntoIterator<Item=T>
    + Clone
{
    type Inner;
    type Error: std::fmt::Debug;

    fn dim(&self) -> usize;

    fn into_inner(self) -> Self::Inner;

    fn push(&mut self, val: T);

    fn permute_rows(self) -> std::vec::IntoIter<T>;

    fn permute_cols(self) -> std::vec::IntoIter<T>;

    fn apply_to_each<F: Fn(T)->T>(self, action: F) -> Self;

    fn extract_row(&self, r: usize) -> Self::Inner;

    fn extract_col(&self, c: usize) -> Self::Inner;

    fn get(&self, row:usize, col:usize) -> Result<T,Self::Error>;

    fn set(&mut self, row:usize, col:usize, val:T) -> Result<(),Self::Error>;

    fn transpose(self) -> Self;

    fn kronecker(self, rhs:Self) -> Self;

    fn scalar(self, rhs: T) -> Self;

    fn cross(self, rhs: Self) -> Self;

    fn vector_product<V: VectorAlgebra<T>>(self, rhs: V) -> V;

    // fn eigen_value(self) -> T;

    fn identity(&self) -> Self;

    fn minor(&mut self, m: &Self, d: usize);

    fn trace(self) -> T;

    fn diagonal(&self) -> Self::Inner;

    fn addition(self, rhs: Self) -> Self;

    fn subtraction(self, rhs: Self) -> Self;

    fn qr_decomp<W: VectorAlgebra<T>>(self) -> Self;
}

pub trait ComplexMatrixAlgebra
{
    
    fn complex_conjugate(self) -> Self;

    fn hermitian_conjugate(self) -> Self;
}

pub trait VectorAlgebra<T>
where
    Self: From<Vec<T>>
    + IntoIterator<Item=T>
    + Clone
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

    fn outer_product<M: MatrixAlgebra<T>>(self, rhs: Self) -> M;

    fn addition(self, rhs: Self) -> Self;
    
    fn scalar(self, rhs: T) -> Self;

    fn matrix_product<M: MatrixAlgebra<T>>(self, rhs: M) -> Self;

    fn eucl_dist(&self) -> T;
}

pub trait ComplexVectorAlgebra
{
    fn conjugate_transpose(self) -> Self;
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
impl QuantumUnit for num::Complex<f32> {
    fn pow64(self, rhs: f64) -> Self { self.powf(rhs as f32) }     
    fn sqroot(self) -> Self { self.sqrt() }
}
impl QuantumUnit for num::Complex<f64> {
    fn pow64(self, rhs: f64) -> Self { self.powf(rhs) }     
    fn sqroot(self) -> Self { self.sqrt() }
}