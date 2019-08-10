use crate::matrix;
use matrix::interface;
use interface::{Identity, BackwardSubstitution};

use std::ops;

impl<T:Copy> interface::Inverse for matrix::Matrix<T>
where
    T: ops::Div<Output=T>
    + ops::Mul<Output=T>
    + ops::Sub<Output=T>
    + ops::AddAssign
    + num::Zero   
    + num::One
{
    type Output = Self;

    fn inverse(self) -> Self::Output 
    {
        // let mat: _ = AugmentedMatrix::new(self).into_matrix();
        // let mut inv: Vec<T> = Vec::new();
        // let I: _ = (&self).identity();

        // for idx in 0..self.col {
        //     let buf: _ = (&self).backward_substitution(I[idx].to_vec().into());
        //     inv.append(&mut buf.into())
        // }

        // inv.into()

        
    }
}

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
