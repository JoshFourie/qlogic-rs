//! Docs: InProgress, view src.

use crate::matrix;
use matrix::{interface, ops};
use interface::Minor;

use ops::householder::HouseholderReflection;

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
    fn francis_double_step_qr_decomposition(self) -> (matrix::Matrix<T>, matrix::Matrix<T>) 
    {
        let mut mat = self.mat;

        let active_matrix_end = mat.col;
        let matrix_col = mat.col;

        while active_matrix_end > 2 {
            let active_matrix_start = active_matrix_end - 1;

            let (mut x, mut y, mut z): (T,T,T) = Self::precompute_xyz(
                &mat,
                active_matrix_end,
                active_matrix_start
            );

            for current_col in 0..active_matrix_end-3 
            {
                // this may require a submatrix to be passed as the argument.
                let householder_reflector: matrix::Matrix<T> = HouseholderReflection::new(&mat, current_col)
                    .reflector_with_ae1(vec![x,y,z].into());

                Self::reflect_minors_with_householder(
                    active_matrix_end,
                    current_col,
                    householder_reflector,
                    &mut mat
                );

                x = mat[current_col+2][current_col+1];
                y = mat[current_col+3][current_col+1];

                if current_col < active_matrix_end-3 {
                    z = mat[current_col+4][current_col+1]
                }
            }

            let sub_matrix: _ = (&mat).minor(
                active_matrix_start..active_matrix_end,
                active_matrix_end-2..matrix_col
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

    fn precompute_xyz(mat: &matrix::Matrix<T>, p: usize, q: usize) -> (T,T,T) {

        let s = mat[q][q] + mat[p][p];
        let t = mat[q][q]* mat[p][p] - mat[q][p]*mat[p][q];

        let x = mat[0][0]*mat[0][0] + mat[0][1]*mat[1][0] - s*mat[0][0] + t;
        let y = mat[1][0]*(mat[0][0]+mat[1][1] - s);
        let z = mat[1][0]*mat[2][1];

        (x,y,z)
    }

    fn reflect_minors_with_householder(p: usize, k: usize, P: matrix::Matrix<T>, mut mat: &mut matrix::Matrix<T>) 
    {
        let n: usize = mat.col;     
        let r1: usize = std::cmp::max(1,k); 
        let r2: usize = std::cmp::min(k+4, p);
    
        let (alpha_mat, beta_mat): _ = Self::get_minors(mat,k,r1,r2);   

        let new_sub_matrix: matrix::Matrix<T> = P.clone()*alpha_mat;
        Self::set_sub_matrix_alpha(
            mat,
            new_sub_matrix,
            k,
            r1,
            n
        );

        let new_sub_matrix: matrix::Matrix<T> = beta_mat*P;
        Self::set_sub_matrix_beta(
            mat, 
            new_sub_matrix,
            k, 
            r2
        );       
    }

    fn get_minors(mat: &matrix::Matrix<T>, k: usize, r1: usize, r2: usize) -> (matrix::Matrix<T>, matrix::Matrix<T>)
    {
        let n: usize = mat.col;
        let alpha: _ = mat.minor(k+1..k+3, r1..n);
        let beta: _ = mat.minor(0..r2, k+1..k+3);
        (alpha, beta)
    }

    fn set_sub_matrix_alpha(mut old: &mut matrix::Matrix<T>, new: matrix::Matrix<T>, k: usize, r: usize, n: usize) 
    {
        for (i,x) in (k+1..k+3)
            .zip(0..3) 
        {
            for (j,y) in (r..n)
                .zip(0..n-r) 
            {
                old[i][j] = new[x][y];
            }
        }
    }

    fn set_sub_matrix_beta(mut old: &mut matrix::Matrix<T>, new: matrix::Matrix<T>, k: usize, r: usize)
    {
        for i in 0..r {
            for (j,y) in (k+1..k+3)
                .zip(0..3) 
            {
                old[i][j] = new[i][y]
            }
        }   
    }
}