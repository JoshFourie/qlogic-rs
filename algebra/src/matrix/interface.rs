//! A module containing the Traits that are implemented over 
//! the `Matrix` structure.
//! 
//! # Using the Traits
//! ```
//! use crate::algebra::matrix;
//! 
//! use matrix::interface::Column;
//! 
//! fn main()
//! {
//!     let matrix: matrix::Matrix<usize> = vec![0,1,2,3,4,5,6,7,8].into();
//! 
//!     let col = matrix.get_col(0);
//! 
//!     assert_eq!(col, vec![0,3,6]);
//! }
//! ```

/// A convenient `Result` type that wraps around an `AlgebraError`.
pub type Result<T> = std::result::Result<T, crate::error::AlgebraError>;

/// A trait for multiplying with integrity checks over the elements and dimensions.
pub trait CheckedMul<RHS> {

    type Output;

    fn checked_mul(self, rhs: RHS) -> Self::Output;
}

/// A trait for adding with integrity checks over the elements and dimensions.
pub trait CheckedAdd<RHS> {

    type Output;
    
    fn checked_add(self, rhs: RHS) -> Self::Output;
}

/// A trait for subtracting with integrity checks over the elements and dimensions.
pub trait CheckedSub<RHS> {

    type Output;

    fn checked_sub(self, rhs: RHS) -> Self::Output;
}

/// A trait for returning the row and col values of the `Matrix`.
pub trait Dimension<T> {

    fn dim(self) -> (T,T);
}

/// A trait for returning a column from a `Matrix`. The Output may
/// be defined as something like a `Vec<T>` or a `vector::Vector<T>`.
pub trait Column<T> {

    type Output;

    fn get_col(self, idx: T) -> Self::Output;
}

pub trait MutableColumn<T> {
    
    type Output;

    fn get_mut_col(self, idx: T) -> Self::Output;
}

/// A trait for returning a row from a `Matrix`. The Output may
/// be defined as something like a `Vec<T>` or a `vector::Vector<T>`.
pub trait Row<T> {

    type Output;

    fn get_row(self, idx: T) -> Self::Output;
}

pub trait MutableRow<T> {
    
    type Output;

    fn get_mut_row(self, idx: T) -> Self::Output;
}

/// A trait for returning the [identity] of a `Matrix`. It
/// requires `self` in the argument paramaters to derive the dimensions.
/// 
/// [identity]: http://mathworld.wolfram.com/IdentityMatrix.html
pub trait Identity {
    
    type Output;
    
    fn identity(self) -> Self::Output;
}

/// A [transpose] of a doubly indexed object is the object obtained 
/// by replacing all elements `a[i][j]` with `a[j][i]`.  
/// The matrix transpose, most commonly written `A^(T)`, is the 
/// matrix obtained by exchanging A's rows and columns.
/// 
/// [transpose]: http://mathworld.wolfram.com/Transpose.html.
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

/// A trait storing the [balancing] sub-routine used for improving the accuracy
/// of Eigenvalue Decomposition algorithms. 
/// 
/// [balancing]: https://arxiv.org/abs/1401.5766
pub trait Balance
{
    type Output;

    fn balance(self) -> Self::Output;
}

/// A trait storing the multiplicative [inverse] of the `Matrix` 
/// structure.
/// 
/// [inverse]: http://mathworld.wolfram.com/MatrixInverse.html. 
pub trait Inverse
{
    type Output;

    fn inverse(self) -> Self::Output;
}

/// A trait storing the [Lower Upper (LU) decomposition] routine.
/// 
/// [Lower Upper (LU) decomposition]: http://mathworld.wolfram.com/LUDecomposition.html
pub trait LU 
{
    type Output;

    fn lu(self) -> Self::Output;
}

/// A trait storing the `.diagonal(self)` and `.trace(self)`
/// methods. 
/// 
/// A call on `.diagonal(self)` should return the [diagonal] of 
/// a `Matrix`, whereas the `.trace(self)` takes the sum of the diagonal (the [trace]).
/// 
/// [trace]: http://mathworld.wolfram.com/MatrixTrace.html
/// 
/// [diagonal]: http://mathworld.wolfram.com/Diagonal.html
pub trait Diagonal<T> {

    type Output;

    fn diagonal(self) -> Self::Output;

    fn trace(self) -> T;
}

/// Given an m×n matrix A and a p×q matrix B, their [Kronecker] product `C = A(X)B`,
/// also called their matrix direct product, is an `(mp)*(nq)` matrix with elements 
/// defined by `c[a][b] = a[i][j] * b[k][l]`, where:
///  	
///     1. a = p(i-1)+k.
///     2. b = q(j-1)+l.
/// 
/// The matrix direct product gives the matrix of the linear transformation induced 
/// by the vector space tensor product of the original vector spaces
/// 
/// [Kronecker]: http://mathworld.wolfram.com/KroneckerProduct.html.
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

/// A trait storing the [QR Decomposition] routine.
/// 
/// [QR Decomposition]: http://mathworld.wolfram.com/QRDecomposition.html.
pub trait QR {

    type Output;

    fn qr(self) -> Self::Output;
}

/// A trait storing the 3 [Elementary Row Operations].
/// 
/// [Elementary Row Operations]: http://mathworld.wolfram.com/ElementaryRowandColumnOperations.html.
pub trait ElementaryRow<T,U> {

    type Output;
    
    fn row_swap(self, r1: U, r2: U) -> Self::Output;

    fn row_add(self, scal: Option<T>, lhs: U, rhs: U) -> Self::Output;

    fn row_mul(self, scal: T, r: U) -> Self::Output;
}

/// A trait storing a routine for solving a triangular system by [forward substitution].
/// 
/// [forward substitution]: http://mathfaculty.fullerton.edu/mathews/n2003/BackSubstitutionMod.html
pub trait ForwardSubstitution<T> {

    type Output = crate::vector::Vector<T>;

    type Vector = crate::vector::Vector<T>;

    fn forward_substitution(self, b: Self::Vector) -> Self::Output;
}

/// A trait storing a routine for solving a triangular system by [backward substitution].
/// 
/// [backward substitution]: http://mathfaculty.fullerton.edu/mathews/n2003/BackSubstitutionMod.html
pub trait BackwardSubstitution<T> {

    type Output = crate::vector::Vector<T>;

    type Vector = crate::vector::Vector<T>;

    fn backward_substitution(self, rhs: Self::Vector) -> Self::Output;
}

/// A trait for returning the [minor] of a Matrix.
/// 
/// [minor]: http://mathworld.wolfram.com/Minor.html.
pub trait Minor<T> {

    type Output;

    fn minor(self, row: T, col: T) -> Self::Output;
}

/// A trait for retrieving the Eigenvalues of a Matrix.
pub trait EigenValue {

    type Output;

    fn eigenvalues(self) -> Self::Output;
}

pub trait SchurDecomposition {

    type Output;

    fn schur(self) -> Self::Output;
}

pub trait LinearSystem<T> {
    
    type Output;
    
    type Vector;

    fn solve(self, rhs: Self::Vector) -> Self::Output;
}
