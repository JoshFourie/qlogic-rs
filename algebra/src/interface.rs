/*** Types ***/

pub type Result<T> = std::result::Result<T, crate::error::AlgebraError>;

/*** Impl ***/

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

pub trait Transpose {

    type Output;
    
    fn transpose(self) -> Self::Output;
    
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

pub trait PLUDecomposition 
{
    type Output;

    fn plu_decomposition(self) -> Self::Output;
}

pub trait Diagonal<T> {

    type Output;

    fn diagonal(self) -> Self::Output;

    fn trace(self) -> T;

}

pub trait Kronecker<RHS> {

    type Output;

    fn kronecker(self, rhs: RHS) -> Self::Output;

}

pub trait EigenvalueDecomposition<T> {
    
    type Output;
    
    fn decompose(self) -> (Self::Output, Self::Output);

    fn det(self) -> T;

    fn eigen_val(self) -> [T];

}   

pub trait ElementaryRowOperations<T,U> {

    type Output;
    
    fn row_swap(self, r1: U, r2: U) -> Self::Output;

    fn row_add(self, scal: Option<T>, lhs: U, rhs: U) -> Self::Output;

    fn row_mul(self, scal: T, r: U) -> Self::Output;

}

pub trait SubMatrix<T> {

    fn set_sub_matrix(self, input: Self) -> Self; 

}