//! Docs: InProgress, view src.

use crate::matrix;
use matrix::interface;
use interface::{Transpose, Column, Minor, Row, Identity};

use crate::vector;
use vector::interface::{Norm, Direct};

use num::traits::real;

// rayleigh quotient shift: https://ocw.mit.edu/courses/mathematics/18-335j-introduction-to-numerical-methods-fall-2010/readings/MIT18_335JF10_lec16.pdf
// implicit psuedocode: https://people.inf.ethz.ch/arbenz/ewp/Lnotes/chapter4.pdf

impl<T:Copy> interface::QR for matrix::Matrix<T>
where
    T: real::Real + From<f32>
{
    type Output = (matrix::Matrix<T>, matrix::Matrix<T>);

    fn qr(self) -> (matrix::Matrix<T>, matrix::Matrix<T>) {
        HouseholderTransformation::qr(self)
    }
}

struct FrancisDoubleStep<T> {
    _marker: std::marker::PhantomData<T>
}

impl<T> FrancisDoubleStep<T> 
where
    T: real::Real
    + From<f32>
{
    // https://people.inf.ethz.ch/arbenz/ewp/Lnotes/chapter4.pdf
    fn francis_double_step_qr_decomposition(mut mat: matrix::Matrix<T>) -> (matrix::Matrix<T>, matrix::Matrix<T>) 
    {
        let active_matrix_end = mat.col;
        let matrix_col = mat.col;

        while active_matrix_end > 2 
        {
            let active_matrix_start = active_matrix_end - 1;

            let (mut x, mut y, mut z): (T,T,T) = Self::precompute_xyz(
                &mat,
                active_matrix_end,
                active_matrix_start
            );

            for current_col in 0..active_matrix_end-3 
            {
                // this may require a submatrix to be passed as the argument.
                let householder_reflector: matrix::Matrix<T> = HouseholderTransformation::new(&mat, current_col)
                    .reflector_with_ae1(vec![x,y,z].into());

                Self::reflect_minor_with_householders(
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

    fn reflect_minor_with_householders(p: usize, k: usize, P: matrix::Matrix<T>, mut mat: &mut matrix::Matrix<T>) 
    {
        let n: usize = mat.col;        

        let r: usize = std::cmp::max(1,k); 
        let new_sub_matrix: matrix::Matrix<T> = P.clone()*mat.minor(k+1..k+3, r..n);
        for (i,x) in (k+1..k+3)
            .zip(0..3) 
        {
            for (j,y) in (r..n)
                .zip(0..n-r) 
            {
                mat[i][j] = new_sub_matrix[x][y];
            }
        }

        let r: usize = std::cmp::min(k+4, p);
        let new_sub_matrix: matrix::Matrix<T> = mat.minor(0..r, k+1..k+3)*P;
        for i in 0..r {
            for (j,y) in (k+1..k+3)
                .zip(0..3) 
            {
                mat[i][j] = new_sub_matrix[i][y]
            }
        }
    }
}

struct HouseholderTransformation<T> {
    I: matrix::Matrix<T>,
    x: vector::Vector<T>,
    k: usize,
    col: usize
}

impl<T> HouseholderTransformation<T> 
where
    T: real::Real
    + From<f32>
{
    fn new(source_matrix: &matrix::Matrix<T>, k: usize) -> Self {
        HouseholderTransformation {
            I: source_matrix.identity(),
            x: source_matrix.get_col(k).into(),
            k,
            col: source_matrix.col
        }
    }

    fn qr(mut mat: matrix::Matrix<T>) -> (matrix::Matrix<T>,matrix::Matrix<T>) {
        let A: matrix::Matrix<T> = mat.clone();
        let col: usize = mat.col; 
        let I: matrix::Matrix<T> = (&mat).identity();

        let mut series_of_Q: Vec<matrix::Matrix<T>> = Vec::with_capacity(col);
        let mut series_of_R: Vec<matrix::Matrix<T>> = Vec::with_capacity(col);

        for k in 0..col-1 {
            let P: matrix::Matrix<T> = Self::new(&mat,k).reflector();
            let mut Q: matrix::Matrix<T> = (&P)*(&mat);

            for i in 0..mat.row {
                Q[k][i] = T::zero();
                Q[i][k] = T::zero();
            }
            
            series_of_Q.push((&P).transpose());
            series_of_R.push(P);

            Q[k][k] = T::one();
            mat = Q;
        }

        let R: matrix::Matrix<T> = series_of_R.into_iter()
            .rev()
            .fold(I.clone(),|acc,Rk| acc * Rk) * A;
        
        let Q: matrix::Matrix<T> = series_of_Q.into_iter()
            .fold(I, |acc,Qk| acc * Qk);

        (Q,R) 
    }

    fn reflector(self) -> matrix::Matrix<T> {   
        let alpha_e1: vector::Vector<T> = {
            let x = &self.x;
            let k = self.k;

            let mut e1: vector::Vector<T> = vec![T::zero(); self.col].into();
            e1[k] = T::one();
            let alpha: T = -x[k+1].signum() * x.clone().eucl_norm();

            e1*alpha
        };
        
        self.reflector_with_ae1(alpha_e1)        
    }

    fn reflector_with_ae1(self, alpha_e1: vector::Vector<T>) -> matrix::Matrix<T> {
        let V: vector::Vector<T> = {
            let u: vector::Vector<T> = self.x - alpha_e1;
            u.clone() * (T::one()/u.eucl_norm())
        };

        let householder: matrix::Matrix<T> = {
            let VV: matrix::Matrix<T> = V.clone().direct_product(V);
            self.I - (VV * <T as std::convert::From<f32>>::from(2.0))                
        };
        
        householder
    }
} 

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

#[cfg(test)] mod tests 
{
    use crate::matrix;

    use matrix::interface::{QR,Column};
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

    #[test] fn test_qr_decomposition()
    {
        let matrix: matrix::Matrix<f64> = vec![12.0, -51.0, 4.0, 6.0, 167.0, -68.0, -4.0, 24.0, -41.0].into();
        let (Q,R): _ = matrix.clone().qr();
        let test_matrix: _ = Q*R;
        
        for (test,exp) in test_matrix.into_iter()
            .zip(matrix)
        {
            if !test.approx_eq(exp, (0.001, 4)) {
               panic!("{} != {}", test, exp)
            }
        }
    }

    #[test] fn second_test_qr_decomposition()
    {
        let matrix: matrix::Matrix<f64> = vec![
            2.0, 4.0, 2.0,
            -1.0, 0.0, -4.0,
            2.0, 2.0, -1.0
        ].into();

        let (Q,R): _ = matrix.clone().qr();
        let test_matrix: _ = Q*R;
        
        for (test,exp) in test_matrix.into_iter()
            .zip(matrix)
        {
            if !test.approx_eq(exp, (0.001, 4)) {
                panic!("{} != {}", test, exp)
            }
        }
    }

    #[test] fn test_qr_eigenvalue()
    {
        use crate::matrix::interface::Diagonal;

        let matrix: matrix::Matrix<f64> = vec![
            9.0, 5.0, 1.0, 2.0, 1.0,
            9.0, 7.0, 10.0, 5.0, 8.0,
            1.0, 7.0, 2.0, 4.0, 3.0,
            4.0, 3.0, 2.0, 10.0, 5.0,
            6.0, 5.0, 4.0, 10.0, 6.0
        ].into();

        let (Q,R): _ = matrix.qr();
        
        let A: _ = R*Q;

        let expected_eiganvalues: Vec<f64> = vec![
            25.8275, -4.9555, -0.1586, 6.4304, 6.8562
        ];     
        let test_eigenvalues: _ = A.diagonal();

        for (test,exp) in test_eigenvalues.into_iter()
            .zip(expected_eiganvalues)
        {
            if !test.approx_eq(exp, (0.001, 4)) {
                panic!("{} != {}", test, exp)
            }
        }
    }
}