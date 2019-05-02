
/***** Impl *******/

pub trait Transpose {

    type Output;
    
    fn transpose(self) -> Self::Output;
    
}

pub trait Norm<T> {

    fn eucl_norm(self) -> T;

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

pub trait ElementaryRowOperations<T> {

    type Output;
    
    fn row_swap(self, r1: usize, r2: usize) -> Self::Output;

    fn row_add(self, lhs: usize, rhs: usize) -> Self::Output;

    fn row_mul(self, scal: T, r: usize) -> Self::Output;

}