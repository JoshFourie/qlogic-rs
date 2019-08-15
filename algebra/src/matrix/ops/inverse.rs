use crate::matrix;
use matrix::interface;
use interface::{Identity, LU, BackwardSubstitution, ForwardSubstitution};

use crate::vector;

use std::ops;

impl<T:Copy> interface::Inverse for matrix::Matrix<T>
where
    T: num::Zero 
    + num::One 
    + ops::Div<Output=T>
    + ops::Mul<Output=T>
    + ops::Sub<Output=T>
    + ops::AddAssign<T>
    + num::Signed
    + PartialOrd<T>
{
    type Output = Self;

    fn inverse(self) -> Self::Output 
    {
        unimplemented!() 
    }
}

/*
struct PenroseAlgorithm<T> {
    mat: matrix::Matrix<T>
}

impl<T: Copy> PenroseAlgorithm<T> 
where
    T: num::Zero 
    + num::One
    + ops::Mul<T,Output=T>
    + ops::Div<T,Output=T>
    + ops::Sub<T,Output=T>
    + ops::Div<usize,Output=T>
{
    fn new(mat: matrix::Matrix<T>) -> Self {
        let I = (&mat).identity();
        let B =  (&mat).adjoint() * (&mat);
        let rank: usize = (&mat).rank();
        let mut sigma: Vec<matrix::Matrix<T>> = vec![I.clone()];

        for k in 1..rank {
            let prev: matrix::Matrix<T> = sigma.last()
                .expect("expected previous matrix in series")
                .clone();
            let trace: T = ((&prev) * (&B)).trace();
            sigma.push(
                I.clone()*(trace/k) - (prev * B.clone())
            )
        }
        
        let last: matrix::Matrix<T> = sigma.last()
            .expect("expected previous matrix in series")
            .clone();
        let trace: T = (last.clone() * B.clone()).trace();
        let dagger: matrix::Matrix<T> = last * mat.adjoint() * (rank/trace);
        
        Self {
            mat: dagger
        }
    }
} */

struct AugmentedMatrix<T>(matrix::Matrix<T>);

impl<T:Clone> AugmentedMatrix<T> 
where
    T: num::Zero + num::One
{
    fn new(mat: matrix::Matrix<T>) -> Self 
    {
        let I: _ = (&mat).identity();
        let mut buf: Vec<T> = Vec::new();

        for i in 0..mat.row {
            buf.extend_from_slice(&mat[i]);
            buf.extend_from_slice(&I[i]);
        }

        let out: _ = matrix::Matrix::new(buf, mat.row, 2*mat.col);
        AugmentedMatrix(out)
    }

    fn into_matrix(self) -> matrix::Matrix<T> {
        self.0
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use interface::Inverse;

    #[test]
    fn test_augment() {
        let matrix: matrix::Matrix<f64> = vec![
            12.0, -51.0, 4.0, 
            6.0, 167.0, -68.0, 
            -4.0, 24.0, -41.0
        ].into();

        let test: _ = AugmentedMatrix::new(matrix).0;

        let exp: matrix::Matrix<f64> = matrix::Matrix::new(vec![
            12.0, -51.0, 4.0, 1.0, 0.0, 0.0,
            6.0, 167.0, -68.0, 0.0, 1.0, 0.0,
            -4.0, 24.0, -41.0, 0.0, 0.0, 1.0
        ], 3, 6);
        

        assert_eq!(test, exp)
    }

    #[test] fn test_inversion() 
    {
        let matrix: matrix::Matrix<f64> = vec![
            2.0, -1.0, 0.0, 
            -1.0, 2.0, -1.0, 
            0.0, -1.0, 2.0
        ].into();   

        let test: _ = matrix.inverse();

        let exp: matrix::Matrix<f64> = vec![
            3.0/4.0, 1.0/2.0, 1.0/4.0, 
            1.0/2.0, 1.0, 1.0/2.0, 
            1.0/4.0, 1.0/2.0, 3.0/4.0
        ].into();

        assert_eq!(test, exp)       
    }

}
