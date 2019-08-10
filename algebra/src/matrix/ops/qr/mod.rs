use crate::matrix;
use matrix::interface;
use num::traits::real;

pub mod householder;
mod francis;

// rayleigh quotient shift: https://ocw.mit.edu/courses/mathematics/18-335j-introduction-to-numerical-methods-fall-2010/readings/MIT18_335JF10_lec16.pdf
// implicit psuedocode: https://people.inf.ethz.ch/arbenz/ewp/Lnotes/chapter4.pdf

impl<T:Copy> interface::QR for matrix::Matrix<T>
where
    T: real::Real + From<f32>
{
    type Output = (matrix::Matrix<T>, matrix::Matrix<T>);

    fn qr(self) -> (matrix::Matrix<T>, matrix::Matrix<T>) {
       let (Q,_,R): _ = householder::HouseholderDecomposition::new(self).into_tuple();
       (Q,R)
    }
}

#[cfg(test)] mod tests 
{
    use crate::matrix;

    use matrix::interface::QR;
    use float_cmp::ApproxEq;

    #[test] 
    fn test_qr_decomposition() {
        let matrix: matrix::Matrix<f64> = vec![
            12.0, -51.0, 4.0, 
            6.0, 167.0, -68.0, 
            -4.0, 24.0, -41.0
        ].into();

        let exp_Q: matrix::Matrix<f64> = vec![
            0.8571, -0.3943, 0.3314,
            0.4286, 0.9029, -0.0343,
            -0.2857, 0.1714, 0.9429
        ].into();

        let exp_R: matrix::Matrix<f64> = vec![
            14.0, 21.0, -14.0,
            0.0, 175.0, -70.0,
            0.0, 0.0, -35.0
        ].into();
        
        let (Q,R): _ = matrix.qr();

        for (test,exp) in Q.into_iter()
            .zip(exp_Q)
        {
            if !test.abs().approx_eq(exp.abs(), (0.001, 4)) {
                panic!("{} != {}", test, exp)
            }
        }

        for (test,exp) in R.into_iter()
            .zip(exp_R)
        {
            if !test.abs().approx_eq(exp.abs(), (0.001, 4)) {
                panic!("{} != {}", test, exp)
            }
        }
    }

    #[test]
    fn test_second_qr_decomposition() {
        let matrix: matrix::Matrix<f32> = vec![
            4.0, 1.0, 3.0, -2.0,
            1.0, -2.0, 4.0, 1.0,
            3.0, 4.0, 1.0, 2.0,
            -2.0, 1.0, 2.0, 3.0
        ].into();

        let exp_Q: matrix::Matrix<f32> = vec![
            -0.730_297, -0.144_673, -0.179_957, -0.642_924,
            -0.182_574, -0.578_691, -0.610_841, 0.508_582,
            -0.547_723, 0.67514, -0.0684_345, 0.48939,
            0.365_148, 0.434_019, -0.767_987, -0.297_472
        ].into();

        let exp_R: matrix::Matrix<f32> = vec![
            -5.47723, -2.19089, -2.73861, 1.27802,
            0.0, 4.14729, -1.20561, 2.36299,
            0.0, 0.0, -4.58765, -2.69176,
            0.0, 0.0, 0.0, 1.88079
        ].into();

        let (Q,R): _ = matrix.qr();

        for (test,exp) in Q.into_iter()
            .zip(exp_Q)
        {
            if !test.abs().approx_eq(exp.abs(), (0.001, 4)) {
                panic!("{} != {}", test, exp)
            }
        }

        for (test,exp) in R.into_iter()
            .zip(exp_R)
        {
            if !test.abs().approx_eq(exp.abs(), (0.001, 4)) {
                panic!("{} != {}", test, exp)
            }
        }
    }
}
