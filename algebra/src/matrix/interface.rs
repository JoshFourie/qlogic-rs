pub type Result<T> = std::result::Result<T, crate::error::AlgebraError>;

pub trait CheckedMul<RHS> {

    type Output;

    fn checked_mul(self, rhs: RHS) -> Self::Output;

}

pub trait CheckedAdd<RHS> {

    type Output;
    
    fn checked_add(self, rhs: RHS) -> Self::Output;

}

pub trait CheckedSub<RHS> {

    type Output;

    fn checked_sub(self, rhs: RHS) -> Self::Output;

}

pub trait Dimension<T> {

    fn dim(self) -> (T,T);

}

pub trait Column<T> {

    type Output;

    fn get_col(self, idx: T) -> Self::Output;

}

pub trait Row<T> {

    type Output;

    fn get_row(self, idx: T) -> Self::Output;

}

pub trait Identity {
    
    type Output;
    
    fn identity(self) -> Self::Output;

}

/// A transpose of a doubly indexed object is the object obtained 
/// by replacing all elements `a[i][j]` with `a[j][i]`.  
/// The matrix transpose, most commonly written `A^(T)`, is the 
/// matrix obtained by exchanging A's rows and columns.
/// 
/// WolframAlpha: http://mathworld.wolfram.com/Transpose.html.
pub trait Transpose {

    type Output;
    
    fn transpose(self) -> Self::Output;
    
}

/// A [`Transpose`] operation that identifies errors and returns exceptions
/// instead of panicking.
/// 
/// [`Transpose`]: ../trait.Transpose.html
pub trait SafeTranspose {

    type Output;
    
    fn safe_transpose(self) -> Self::Output;
    
}

pub trait Norm<T> { // eigenvalues required.
    
    fn eucl_norm(self) -> T;

}

pub trait Balance
{
    type Output;

    fn balance(self) -> Self::Output;

}

pub trait Inverse
{
    type Output;

    fn inverse(self) -> Self::Output;
}

pub trait LU 
{
    type Output;

    fn lu(self) -> Self::Output;
}

pub trait Diagonal<T> {

    type Output;

    fn diagonal(self) -> Self::Output;

    fn trace(self) -> T;

}

/// Given an m×n matrix A and a p×q matrix B, their Kronecker product `C = A(X)B`,
/// also called their matrix direct product, is an `(mp)*(nq)` matrix with elements 
/// defined by `c[a][b] = a[i][j] * b[k][l]`, where:
///  	
///     1. a = p(i-1)+k.
///     2. b = q(j-1)+l.
/// 
/// The matrix direct product gives the matrix of the linear transformation induced 
/// by the vector space tensor product of the original vector spaces
/// 
/// WolframAlpha: http://mathworld.wolfram.com/KroneckerProduct.html.
pub trait Kronecker<RHS> {

    type Output;

    fn kronecker(self, rhs: RHS) -> Self::Output;

}

/// A [`Kronecker`] operation that identifies errors and returns exceptions
/// instead of panicking.
/// 
/// [`Kronecker`]: ../trait.Kronecker.html
pub trait SafeKronecker<RHS> {

    type Output;

    fn safe_kronecker(self, rhs: RHS) -> Self::Output;

}

pub trait QR {

    type Output;

    fn qr(self) -> Self::Output;

}

pub trait ERO<T,U> {

    type Output;
    
    fn row_swap(self, r1: U, r2: U) -> Self::Output;

    fn row_add(self, scal: Option<T>, lhs: U, rhs: U) -> Self::Output;

    fn row_mul(self, scal: T, r: U) -> Self::Output;

}

pub trait ForwardSubstitution<T> {

    type Output = crate::vector::Vector<T>;

    type Vector = crate::vector::Vector<T>;

    fn forward_substitution(self, b: Self::Vector) -> Self::Output;

}

pub trait BackwardSubstitution<T> {

    type Output = crate::vector::Vector<T>;

    type Vector = crate::vector::Vector<T>;

    fn backward_substitution(self, rhs: Self::Vector) -> Self::Output;

}