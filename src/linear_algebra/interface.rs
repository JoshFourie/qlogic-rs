
/***** Impl *******/

pub trait Transpose<T> {
    
    fn transpose(self) -> Self;
    
}

pub trait Norm<T> {

    fn eucl_norm(self) -> T;

}

pub trait Diagonal<T> {

    fn diagonal(self) -> Vec<T>;

    fn trace(self) -> T;

}

pub trait Kronecker {

    type Output;

    fn kronecker<A>(self, rhs: A) -> Self::Output;

}

pub trait EigenvalueDecomposition<T> {
    
    type Output;
    
    fn decompose(self) -> (Self::Output, Self::Output);

    fn det(self) -> T;

    fn eigen_val(self) -> [T];

}   

pub trait ElementaryRowOperations<T> {
    
    fn row_swap(self) -> Self;

    fn row_add(self) -> Self;

    fn row_mul(self) -> Self;

}