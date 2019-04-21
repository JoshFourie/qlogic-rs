/**** Imports *******/
use num::Num;
use super::matrix::*;

/***** Impl *******/
pub trait StdOps<A,T>
where
    A: StdOps<A,T>,
    T: Num,
{
    fn transpose(self) -> Self;

    fn eucl_norm(self) -> T;

    fn kronecker(self, rhs: A) -> T;

    fn scalar(self, rhs: T) -> Self;

    fn cross(self, rhs: Self) -> Self;

    fn identity(self) -> square::Square<T>;    

    fn trace(self) -> T;

    fn diagonal(self) -> Vec<T>;

    fn addition(self, rhs: Self) -> Self;

    fn subtraction(self, rhs: Self) -> Self;

}

pub trait EVD<T> {
    
    type Output;
    
    fn decompose(self) -> (Self::Output, Self::Output);

    fn det(self) -> T;

    fn eigen_val(self) -> [T];

}   

pub trait ElementaryRowOps<T> {
    
    fn row_swap(self) -> Self;

    fn row_add(self) -> Self;

    fn row_mul(self) -> Self;

}