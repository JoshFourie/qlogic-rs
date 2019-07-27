//! Docs: InProgress, view src.

use crate::{vector,matrix};
use matrix::{interface, ops};
use interface::{Column, Minor};

use ops::householder;

use num::traits::real;

struct FrancisDoubleStep<T> {
    mat: matrix::Matrix<T>
}

impl<T> FrancisDoubleStep<T> 
where
    T: real::Real
    + From<f32>
{
    // https://people.inf.ethz.ch/arbenz/ewp/Lnotes/chapter4.pdf
    fn francis_double_step_qr_decomposition(self) -> (matrix::Matrix<T>, matrix::Matrix<T>) {
        let mut mat = self.mat;

        let p = mat.col;
        let matrix_col = mat.col;

        while p > 2 {
            let q = p - 1;

            let mut helper: _ = DoubleStepHelper::new(&mut mat);
            let mut precompute: _ = helper.precompute(p,q);

            for k in 0..p-3 
            {
                let householder: matrix::Matrix<T> = helper.householder(&precompute, k);

                helper.reflect(&householder,k,p);

                precompute[0] = helper.mat[k+2][k+1];
                precompute[1] = helper.mat[k+3][k+1];
                if k < p-3 {
                    precompute[2] = helper.mat[k+4][k+1]
                }
            }

            let sub_matrix: _ = (&mat).minor(
                q..p,
                p-2..matrix_col
            );

                    

            /* let givens_rotator: _ = GivensRotation::new(
                0, 1, 0, sub_matrix
            ).multiplication();

            for */

            // maybe insert row i=0,j=1 with vec![x,y]?
            /* let givens_rotator: _ = GivensRotation::rotator(
                0,
                1, 
                0, 
                &vec![x,y].into()
            );

            let new_sub_matrix: _ = givens_rotator.clone()*mat.minor(
                active_matrix_start..active_matrix_end,
                active_matrix_end-2..matrix_col
            );

            for i in active_matrix_start..active_matrix_end {
                for j in active_matrix_end-2..matrix_col {
                    
                }
            } */


            // for i in q..p {
                // for j in p-2..n {
                    // mat[i][j] = P*mat[i][j]
                //  }
            // } 
            // for i in 0..p {
                // for j in p-1..p {
                    // mat[i][j] = mat[i][j]*P
                // }
            // }

            // if mat[p][q].abs() < std::f64::EPSILON * (mat[qq].abs() + mat[p][p]) {
                // mat[p][q] = T::zero();
                // p = p - 1;
                // q = p - 1;
            // } else if mat[p-1][q-1].abs() < std::f64::EPSILON*(mat[q-1][q-1], mat[q][q]) {
                // mat[p-1][q-1] = T::zero();
                // p = p-2;
                // q = p-1;
            // } 
        }
        unimplemented!()     
    }
}

struct DoubleStepHelper<'a,T> {
    mat: &'a mut matrix::Matrix<T>,
}

impl<'a,T> DoubleStepHelper<'a,T> 
where
    T: real::Real
    + From<f32>
{
    pub fn new(mat: &'a mut matrix::Matrix<T>) -> Self {
        Self { mat }
    }

    pub fn precompute(&self, p: usize, q: usize) -> vector::Vector<T> {
        let mat: _ = &self.mat;
        let s: _ = mat[q][q] + mat[p][p];
        let t: _ = mat[q][q]* mat[p][p] - mat[q][p]*mat[p][q];
        let x: _ = mat[0][0]*mat[0][0] + mat[0][1]*mat[1][0] - s*mat[0][0] + t;
        let y: _ = mat[1][0]*(mat[0][0] + mat[1][1] - s);
        let z: _ = mat[1][0]*mat[2][1];

        vec![x,y,z].into()
    }

    pub fn householder(&self, precompute: &vector::Vector<T>, k: usize) -> matrix::Matrix<T> {
        let ae1: _ = householder::AlphaEpsilonOne::manual(
            vec![
                precompute[0],
                precompute[1],
                precompute[2]
            ].into());
        let helper: _ = householder::Helper::new(self.mat, k);
        householder::Householder::from_ae1(ae1, &helper).into()
    }

    pub fn reflect(&mut self, householder: &matrix::Matrix<T>, k: usize, p: usize) {
        let mat: &matrix::Matrix<T> = self.mat;
        let n: usize = mat.col;     

        let r1: usize = std::cmp::max(1,k); 
        let r2: usize = std::cmp::min(k+4, p);
        let (alpha_mat, beta_mat): _ = self.get_minors(k,r1,r2);  

        let new_sub_matrix: matrix::Matrix<T> = householder * alpha_mat;
        self.set_sub_matrix_alpha(new_sub_matrix, k, r1);

        let new_sub_matrix: matrix::Matrix<T> = beta_mat * householder;
        self.set_sub_matrix_beta(new_sub_matrix, k, r2);      
    }

    fn get_minors(&mut self, k: usize, r1: usize, r2: usize) -> (matrix::Matrix<T>, matrix::Matrix<T>)
    {
        let mat: _ = &mut self.mat;
        let n: usize = mat.col;
        let alpha: _ = mat.minor(k+1..k+3, r1..n);
        let beta: _ = mat.minor(0..r2, k+1..k+3);
        (alpha, beta)
    }

    fn set_sub_matrix_alpha(&mut self, new: matrix::Matrix<T>, k: usize, r: usize) {
        let n: usize = self.mat.col;     
        let mat: _ = &mut self.mat;   
        for (i,x) in (k+1..k+3)
            .zip(0..3) 
        {
            for (j,y) in (r..n)
                .zip(0..n-r) 
            {
                mat[i][j] = new[x][y];
            }
        }
    }

    fn set_sub_matrix_beta(&mut self, new: matrix::Matrix<T>, k: usize, r: usize) {
        let mat: _ = &mut self.mat;
        for i in 0..r {
            for (j,y) in (k+1..k+3)
                .zip(0..3) 
            {
                mat[i][j] = new[i][y]
            }
        }   
    }
} 
