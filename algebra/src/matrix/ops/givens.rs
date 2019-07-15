use crate::matrix;
use matrix::interface;
use interface::{Transpose, Column, Minor, Row, Identity};

use crate::vector;
use vector::interface::{Norm, Direct};

use num::traits::real;

struct GivensRotation<T> {
    rotators: GivensRotators<T>,
    matrix: matrix::Matrix<T>
}

impl<T> GivensRotation<T> 
where
    T: real::Real
    + PartialEq
{
    fn new(rotators: GivensRotators<T>, matrix: matrix::Matrix<T>) -> Self {
        Self { rotators, matrix }
    }

    fn rotate(self, i: usize, j: usize) -> matrix::Matrix<T> 
    {
        let mut mat: _ = self.matrix;
        let rotator_alpha: _ = self.rotators.alpha;
        let rotator_beta: _ = self.rotators.beta;

        for idx in 0..mat.col {
            mat[i][idx] = rotator_alpha[idx]; 
            mat[j][idx] = rotator_beta[idx];
        }
        mat
    }
}

struct GivensRotators<T> {
    alpha: vector::Vector<T>,
    beta: vector::Vector<T>
}

impl<T> GivensRotators<T>  
where
    T: real::Real
    + PartialEq
{   
    fn new(vectors: GivensVectors<T>, constants: GivensConstants<T>) -> Self 
    {
        let row_i: _ = vectors.row_i;
        let row_j: _ = vectors.row_j;

        let (c,s): (T,T) = (constants.c, constants.s);

        let alpha: _ = row_i.clone()*c + row_j.clone()*s;
        let beta: _ = row_i*(-s) + row_j*c;

        Self { alpha, beta }
    }
}

struct GivensConstants<T> {
    c: T,
    s: T,
}

impl<T> GivensConstants<T> 
where
    T: real::Real
    + PartialEq
{
    fn new(xi: T, xj: T) -> Self {
        let (c,s): _ = if xj==T::zero() {
            (T::one(), T::zero())   
        } else if xj.abs() > xi.abs() {
            let xi_div_xj: _ = xi/xj;
            let s: _ = {
                let denominator: T = (T::one() + xi_div_xj.powi(2)).sqrt();
                T::one()/denominator
            };
            let c: _ = s*xi_div_xj;
            (c,s)
        } else {
            let xj_div_xi: _ = xj/xi;
            let c: _ = {
                let denominator: T = (T::one() + xj_div_xi.powi(2)).sqrt();
                T::one()/denominator
            };
            let s: _ = c*xj_div_xi;
            (c,s)
        };
        Self { c,s }
    } 
}

struct GivensVectors<T> {
    row_i: vector::Vector<T>,
    row_j: vector::Vector<T>,
}

impl<T:Copy> GivensVectors<T> 
where
    T: real::Real
    + PartialEq
{
    fn new(mat: &matrix::Matrix<T>, i: usize, j: usize) -> Self 
    {
        let row_i: vector::Vector<T> = (&mat).get_row(i).into();
        let row_j: vector::Vector<T> = (&mat).get_row(j).into();

        GivensVectors { row_i, row_j }
    }   
}

#[cfg(test)]
mod tests 
{
    use crate::matrix;
    use matrix::interface::Column;
    
    use float_cmp::ApproxEq;

    #[test] fn test_givens_constants()
    {
        let A: matrix::Matrix<f64> = vec![
            1.0, 3.0, -6.0, -1.0,
            4.0, 8.0, 7.0, 3.0,
            2.0, 3.0, 4.0, 5.0,
            -9.0, 6.0, 3.0, 2.0
        ].into();

        let x: _ = (&A).get_col(0); 
        let (i,j): (usize,usize) = (2,3);
        let constants: _ = super::GivensConstants::new(x[i], x[j]);
        let (c,s): _ = (constants.c, constants.s);

        let (exp_c, exp_s): _ = (0.2425, 0.9701);

        if c.approx_eq(exp_c, (0.001, 4)) {
            panic!("{} != {}", c, exp_c)
        }

        if s.approx_eq(exp_s, (0.001, 4)) {
            panic!("{} != {}", s, exp_s)
        }
    }

    #[test] fn test_givens_rotation()
    {
        let A: matrix::Matrix<f64> = vec![
            1.0, 3.0, -6.0, -1.0,
            4.0, 8.0, 7.0, 3.0,
            2.0, 3.0, 4.0, 5.0,
            -9.0, 6.0, 3.0, 2.0
        ].into();

        let x: _ = (&A).get_col(0); 
        let (i,j): (usize,usize) = (0,1);

        let test: matrix::Matrix<f64> = {

            let rotators: _ = {
                let constants: _ = super::GivensConstants::new(x[i], x[j]);
                let vectors: _ = super::GivensVectors::new(&A,i,j);
                super::GivensRotators::new(vectors, constants)
            };
            super::GivensRotation::new(rotators,A).rotate(i,j)
        };
        
        let exp: matrix::Matrix<f64> = vec![
            4.1231, 8.4887, 5.3358, 2.6679, 
            0.0, -0.9701, 7.5186, 1.6977, 
            2.0, 3.0, 4.0, 5.0, 
            -9.0, 6.0, 3.0, 2.0
        ].into();

        for (test,exp) in test.into_iter()
            .zip(exp)
        {
            if !test.approx_eq(exp, (0.001, 4)) {
               panic!("{} != {}", test, exp)
            }
        }
    }
}