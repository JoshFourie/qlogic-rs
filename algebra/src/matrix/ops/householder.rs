use crate::matrix;
use matrix::interface;
use interface::{Column, Identity};

use crate::vector;
use vector::interface::{Norm, Length, Direct};

use num::traits::real;

pub struct Householder<T>(matrix::Matrix<T>);

impl<T> Householder<T> 
where
    T: real::Real + From<f32>
{
    pub fn new(mat: &matrix::Matrix<T>, k: usize) -> Self {
        let helper: _ = Helper::new(mat, k);
        let ae1: _ = AlphaEpsilonOne::new(&helper);
        
        Self::from_ae1(ae1, &helper)
    }

    pub fn from_ae1(ae1: AlphaEpsilonOne<T>, helper: &Helper<'_,T>) -> Self {
        let x: _ = &helper.x;
        let I: _ = helper.mat.identity();

        let V: vector::Vector<T> = {
            let u: vector::Vector<T> = x - &ae1.0;
            u.clone() * (T::one()/u.eucl_norm())
        };

        let householder: matrix::Matrix<T> = {
            let VV: matrix::Matrix<T> = V.clone().direct_product(V);
            I - (VV * <T as std::convert::From<f32>>::from(2.0))                
        };

        Householder(householder)
    }
} 

pub struct AlphaEpsilonOne<T>(vector::Vector<T>);

impl<T> AlphaEpsilonOne<T> 
where
    T: real::Real + From<f32>
{
    pub fn new(helper: &Helper<'_,T>) -> Self {
        let col: usize = helper.mat.col;
        let k: usize = helper.k;
        let x: &vector::Vector<T> = &helper.x;

        let mut e1: vector::Vector<T> = vec![T::zero(); col].into();
        e1[k] = T::one();
        let alpha: T = -x[k+1].signum() * x.clone().eucl_norm();

        AlphaEpsilonOne(e1*alpha)
    }

    pub fn manual(inner: vector::Vector<T>) -> Self {
        AlphaEpsilonOne(inner)
    }
}

pub struct Helper<'a,T> {
    mat: &'a matrix::Matrix<T>,
    x: vector::Vector<T>,
    k: usize,
}

impl<'a,T> Helper<'a,T> 
where
    T: real::Real + From<f32>
{
    pub fn new(mat: &'a matrix::Matrix<T>, k: usize) -> Self {
        Helper {
            mat, 
            x: mat.get_col(k).into(), 
            k
        }
    }
}

impl<T> From<Householder<T>> for matrix::Matrix<T> {
    fn from(householder: Householder<T>) -> Self {
        householder.0
    }
} 
