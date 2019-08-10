use crate::matrix;
use matrix::interface;
use interface::{QR, Diagonal};

impl<T> interface::EigenValue for matrix::Matrix<T> 
where
    T: num::traits::real::Real + From<f32>
{
    type Output = Vec<T>;

    fn eigenvalues(mut self) -> Self::Output {
        let estimated_convergence: usize = self.row;
        for _ in 0..estimated_convergence {
            let (Q,R): _ = self.qr();
            self = R*Q
        }
        self.diagonal()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use interface::EigenValue;
    use float_cmp::ApproxEq;

    #[test]
    fn test_qr_algorithm() {
        let matrix: _ = matrix::Matrix::from(vec![
            52_f64, 30_f64, 49_f64, 28_f64,
            30_f64, 50_f64, 7_f64, 44_f64,
            49_f64, 8_f64, 46_f64, 16_f64,
            28_f64, 44_f64, 16_f64, 22_f64
        ]);

        let exp: Vec<f64> = vec![132.6279, 52.4423, -11.54113, -3.52904];
        let test: _ = matrix.eigenvalues();

        for (t,e) in test.into_iter()
            .zip(exp)
        {
            if !t.abs().approx_eq(e.abs(), (1.0, 4)) {
                panic!("{} != {}", t, e)
            }
        }
    }
}
