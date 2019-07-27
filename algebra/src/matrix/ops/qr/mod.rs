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
       householder::HouseholderDecomposition::new(self).into_tuple()
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

    #[ignore]
    #[test] fn test_qr_eigenvalue()
    {
        use crate::matrix::interface::Transpose;

        let matrix: matrix::Matrix<f64> = vec![
            4.0, -3.0, 0.0, 0.0,
            -3.0, 2.0, 3.16228, 0.0,
            0.0, 3.16228, -1.4, 0.2,
            0.0, 0.0, -0.2, 1.4
        ].into();

        let (Q,R): _ = matrix.clone().qr();
        println!("{:?}", Q.clone().transpose()*matrix*Q);
        panic!("")
    }
}