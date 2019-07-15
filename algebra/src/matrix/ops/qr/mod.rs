use crate::matrix;
use matrix::interface;
use num::traits::real;

mod double_step;
mod householder;

// rayleigh quotient shift: https://ocw.mit.edu/courses/mathematics/18-335j-introduction-to-numerical-methods-fall-2010/readings/MIT18_335JF10_lec16.pdf
// implicit psuedocode: https://people.inf.ethz.ch/arbenz/ewp/Lnotes/chapter4.pdf

impl<T:Copy> interface::QR for matrix::Matrix<T>
where
    T: real::Real + From<f32>
{
    type Output = (matrix::Matrix<T>, matrix::Matrix<T>);

    fn qr(self) -> (matrix::Matrix<T>, matrix::Matrix<T>) {
       householder::HouseholderDecomposition::new(self).qr()
    }
}

#[cfg(test)] mod tests 
{
    use crate::matrix;

    use matrix::interface::QR;
    use float_cmp::ApproxEq;

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