/**** Imports *******/
use num::Num;
use super::matrix::*;

/***** Impl *******/
/******************
 * Matrix impl: Multiplicative identity, IndexMut
******************/

pub trait StdOps<T> {
    
    fn transpose(self) -> Self;

    /* fn eucl_norm(self) -> T;

    fn scalar(self, rhs: T) -> Self;

    fn cross(self, rhs: Self) -> Self;

    fn addition(self, rhs: Self) -> Self;

    fn subtraction(self, rhs: Self) -> Self;

    fn trace(self) -> T;

    fn diagonal(self) -> Vec<T>; */

}

pub trait Kronecker {

    type Output;

    fn kronecker<A>(self, rhs: A) -> Self::Output;

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