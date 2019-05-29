/****** Imports ******/
use crate::interface::{
    Dimension, 
    Identity, 
    Column, 
    Row, 
    Norm
};
use crate::matrix::Matrix;
use crate::error;

use num::Float;
use std::ops::{MulAssign, DivAssign};

/**************
 * We need an inverse to complete the EVD.
 * We need GuassianElimination for inverse
***************/

// Gaussian Elimination w/ Partial Pivoting.
macro_rules! impl_lu_decomp
{
    ($id:ty) => 
    {
        impl<'a,T: Copy> crate::interface::PLUDecomposition for $id
        where
            T: num::Float
            + std::fmt::Debug
        {
            type Output = (Matrix<T>, Matrix<T>, Matrix<T>);

            fn plu_decomposition(self) -> Self::Output
            {
                // instantiating working space.
                let mut A: Matrix<T> = Matrix {
                    inner: self.inner.clone(),
                    row: self.row,
                    col: self.col
                };

                // initialising variables.
                let (m,n): (usize,usize) = (&A).dim();
                let mut L: Matrix<T> = (&A).identity();
                let mut U: Matrix<T> = (&A) * T::zero();

                let mut p: Matrix<T> = 
                {
                    let mut idx: T = T::zero();
                    let i: Vec<T> = vec![T::zero(); m]
                        .into_iter()
                        .map(|_| {
                            idx = idx + T::one();
                            idx
                        }).collect();
                    Matrix {
                        inner: i,
                        row: 1,
                        col: m
                    }
                };

                let mut r: usize = 0; // potential error.

                // println!(" \n m: {}, n: {}, r: {} \n\n L: {:?} \n U: {:?} \n P: {:?}", m,n,r,L,U,p);
                
                /******* Elimination Routine.
                 * u is the pivot.
                    * the pivot is the largest value in the left-most column along the diagonal.
                    * we set the row_idx to 0 and initialise 'next'.
                    * we enumerate to take the position for a comparison.
                    * we need the values along the kth column between r and m filtered using if condition.
                    * we then compare the value at the kth col and ith row.
                    * if val is greater than 'next' then idx is set to the position.
                    * we now have the 'pivot' column that should be in the rth position.
                 * if A[u][k] is a non-zero value, we can proceed.
                    * we swap a portion of the rth row with the uth row between r and m for L and A.
                    * next, we need to set the values of the L and U matrices.
                    * for the L matrix: we set L[r+1..m][r] to A[r+1..m][k] divided by A[r][k]. (DooLittle Algo.)
                    * for the U matrix: we set U[r][k..n] to A[r][k..n].
                    * we then iterate back through for each column.
                ****************************/

                // println!(" A: {:?}", A);

                for k in 0..n
                {
                    // max value in left-most column for idx < r.
                    let u: usize = 
                    {
                        let mut ridx: usize = 0; 
                        let mut max: T = T::zero();
                        for (idx,val) in (&A).get_col(k)
                            .into_iter()
                            .enumerate()
                        {
                            if val.abs() > max {
                                ridx = idx;
                                max = val.abs()
                            }
                        }
                        ridx
                    };

                    // println!(" \n k: {}, u:{}", k,u);

                    if A[u][k] != T::zero() // && u != r
                    {
                        // swapping the uth and rth rows/permutation index.
                        L = {
                            let mut mat: Matrix<T> = L.clone();
                            for c in 0..r {
                                mat[u][c] = L[r][c];
                                mat[r][c] = L[u][c];
                            }
                            mat
                        };
                        
                        A = {
                            let mut mat: Matrix<T> = A.clone();
                            for c in k..n {
                                mat[u][c] = A[r][c];
                                mat[r][c] = A[u][c];
                            }
                            mat
                        };

                        // println!(" \n PA: {:?}", A);

                        // we have to adjust r at this point for zero-indexing
                        p.inner.swap(u,r);

                        // setting row in L/U to corresponding row of A.
                        for idx in r+1..m { L[idx][r] = A[idx][k]/A[r][k] }
                        
                        for idx in k..n { U[r][idx] = A[r][idx] }

                        for i in r..m {
                            for j in k..n 
                            {
                                // println!(" \n A: {:?}, L: {:?}, U: {:?}", A[i][j],L[i][r],U[r][j]);
                                A[i][j] = A[i][j] - (L[i][r] * U[r][j]);
                                // println!(" \n A: {:?}", A[i][j]);  
                            }
                        }

                        //  println!(" \n p: {:?} \n\n L: {:?} \n\n U: {:?}, \n\n A: {:?} \n\n r: {}", p,L,U,A,r);

                        r += 1;
                    }
                }
                (A, L, U)
            }
        }
    }
}

impl_lu_decomp!(Matrix<T>);
impl_lu_decomp!(&'a Matrix<T>);
impl_lu_decomp!(&'a mut Matrix<T>);

impl<T> crate::interface::Norm<T> for Vec<T>
where
    T: Float
{
    fn eucl_norm(self) -> T
    {
        self.into_iter()
            .fold(
                T::zero(), |acc,x| acc + num::pow(x,2) 
            ).sqrt()
    }
}

impl<'a, T: Copy> crate::interface::Balance for Matrix<T>
where
    T: Float
    + DivAssign<T>
    + MulAssign<T>
{
    type Output = crate::interface::Result<Matrix<T>>;

    fn balance(self) -> Self::Output 
    {
        let mut C: Matrix<T> = Matrix {
            inner: self.inner.clone(),
            row: self.row,
            col: self.col
        };

        let ref mut worker: _ = C;

        let dim: usize = { 
            if worker.row != worker.col { 
                return Err(error::AlgebraError::from(error::ErrorKind::MatrixStructure)) 
            } else { worker.row }
        };

        let mut D: _ = worker.identity();
        let mut converged: bool = false;
        
        let beta: T = T::from(2.0)?;

        while !converged {
            for i in 0..dim
            {
                let mut c: T = worker.get_col(i).eucl_norm();
                let mut r: T = worker.get_row(i).eucl_norm();
                let mut f: T = T::one();

                let s: T = num::pow(c,2) + num::pow(r,2);
                
                while c < r/beta {
                    c = c * beta;
                    r = r/beta;
                    f = f * beta;
                }

                while c >= r * beta {
                    c = c/beta;
                    r = r * beta;
                    f = f/beta;
                }

                let x: T = num::pow(c,2) + num::pow(r,2);

                if x < T::from(0.95)? * s 
                {
                    D[i][i] = f;
                    for j in 0..dim 
                    {
                        (*worker)[j][i] *= f;
                        (*worker)[i][j] /= f;                
                    } 
                } else { converged=true }
            }
        }

        // let out: _ = &D * &C * self * D.inverse() * D.transpose();
        
        Ok(C)
    }
}

#[cfg(test)] mod test 
{
    use super::Matrix;

    use crate::interface::*;

    use std::ops::{Mul, Sub};

    #[ignore] #[test] fn test_eucl_norm()
    {
        let v: Vec<f32> = vec![0.0,1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0];
        let exp: f32 = 14.2;
        let t: f32 = v.eucl_norm();
        assert_eq!(exp, t);
    }

    #[test]
    fn test_balance() {
        let T1: Matrix<f64> = Matrix {
            inner: vec![
                -5.5849.mul(10_f64.powf(-1.0)),
                -2.4075.mul(10_f64.powf(7.0)),
                -6.1644.mul(10_f64.powf(14.0)),
                -6.6275.mul(10_f64.powf(0.0)),
                -7.1724.mul(10_f64.powf(-9.0)),
                -2.1248.mul(10_f64.powf(0.0)),
                -3.6083.mul(10_f64.powf(6.0)),
                -2.6435.mul(10_f64.powf(-6.0)),
                -4.1508.mul(10_f64.powf(-16.0)),
                -2.1647.mul(10_f64.powf(-7.0)),
                1.6229.mul(10_f64.powf(-1.0)),
                -7.6315.mul(10_f64.powf(-14.0)),
                4.3648.mul(10_f64.powf(-3.0)),
                1.2614.mul(10_f64.powf(6.0)),
                -1.1986.mul(10_f64.powf(13.0)),
                -6.2002.mul(10_f64.powf(-1.0))
            ],
            row: 4,
            col: 4
        };
        let E: Matrix<_> = Matrix {
            inner: vec![
                -0.5585, -0.3587, -1.0950, 0.1036,
                -0.4813, -2.1248, -0.4313, 2.7719,
                -0.2337, -1.8158, 0.1623, -0.6713,
                0.2793, 1.2029, -1.3627, -0.6200 
            ],
            row: 4,
            col: 4
        };
        let C: _ = T1.balance().unwrap();

        for (exp,test) in E.into_iter()
            .zip(C.into_iter())
        {
            match exp.sub(test) < 0.0001 {
                true => { },
                false => { assert_eq!(exp,test) }
            }
        }
    }

    #[test] fn test_plu_decomposition_for_matrix()
    {
        let A: Matrix<f32> = vec![
            1.0, 1.0, 2.0, 2.0, 
            2.0, 2.0, 4.0, 6.0, 
            -1.0, -1.0, -1.0, 1.0, 
            1.0, 1.0, 3.0, 1.0
        ].into();

        let LE: Matrix<f32> = vec![
            1.0, 0.0, 0.0, 0.0,
            -0.5, 1.0, 0.0, 0.0,
            0.5, 1.0, 1.0, 0.0,
            0.5, 0.0, 0.16666666667, 1.0 
        ].into();

        let UE: Matrix<f32> = vec![
            2.0, 2.0, 4.0, 6.0,
            0.0, 0.0, 1.0, 4.0,
            0.0, 0.0, 0.0, -6.0,
            0.0, 0.0, 0.0, 0.0
        ].into();

        let (_,L,U) = A.plu_decomposition(); 

        assert_eq!(LE,L);

        assert_eq!(UE,U);
    }

    #[ignore]#[test] fn test_plu_decomposition_for_matrix_two()
    {
        let A: Matrix<f32> = vec![
            1.0, -2.0, 3.0,
            2.0, -5.0, 12.0,
            0.0, 2.0, 10.0
        ].into();

        let LE: Matrix<f32> = vec![
            1.0, 0.0, 0.0,
            2.0, 1.0, 0.0,
            0.0, -2.0, 1.0 
        ].into();

        let UE: Matrix<f32> = vec![
            1.0, -2.0, 3.0,
            0.0, -1.0, 6.0,
            0.0, 0.0, 2.0
        ].into();

        let (_,L,U) = A.plu_decomposition(); 

        assert_eq!(LE,L);

        assert_eq!(UE,U);
    }
}