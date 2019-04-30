use crate::linear_algebra::interface::{
    Transpose,
    Norm,
    Diagonal
};

use super::Matrix;

fn default() -> Matrix<usize>
{
    Matrix {
        inner: vec![0,1,2,3,4,5,6,7,8],
        row: 3,
        col: 3
    }
} 

#[test] fn test_transpose_for_matrix() 
{
    let T: Matrix<_> = default();

    let E: Matrix<_> = Matrix {
        inner: vec![0,3,6,1,4,7,2,5,8],
        row: 3,
        col: 3
    };
    assert_eq!(T.transpose(), E);
}

#[test] fn test_eucl_norm_for_matrix() 
{
    let T: Matrix<_> = default();
    let E: usize = 106;
    let N: usize = T.eucl_norm();
}

#[test] fn test_diagonal_trait_for_matrix()
{
    let T1: Vec<usize> = default().diagonal();
    let E1: Vec<usize> = vec![0,4,8];
    assert_eq!(T1,E1);

    let T2: usize = default().trace();
    let E2: usize = 12;
    assert_eq!(T2,E2);
}