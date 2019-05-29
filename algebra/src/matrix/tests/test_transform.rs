use super::*;

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

/* #[test] fn test_eucl_norm_for_matrix() 
{
    let T: Matrix<_> = Matrix {
        inner: vec![0.0,1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0],
        row: 3,
        col: 3
    };
    let e: f32 = 14.2267;
    let n: f32 = T.eucl_norm();
    assert_eq!(e,n);
} */

#[test] fn test_diagonal_trait_for_matrix()
{
    let T1: Vec<usize> = default().diagonal();
    let E1: Vec<usize> = vec![0,4,8];
    assert_eq!(T1,E1);

    let T2: usize = default().trace();
    let E2: usize = 12;
    assert_eq!(T2,E2);
}

#[test] fn test_kronecker_trait_for_marix()
{
    let T1A: Matrix<_> = Matrix {
        inner: vec![2.0, 4.0, 6.0, 8.0],
        row: 2,
        col: 2
    };
    let T1B: Matrix<_> = Matrix {
        inner: vec![1.0,3.0,5.0,7.0,9.0,11.0],
        row: 3,
        col: 2
    };
    let T1: Matrix<_> = T1A.kronecker(T1B);

    let E1: Matrix<_> = Matrix {
        inner: vec![
            2.0,    6.0,    4.0,    12.0,
            10.0,   14.0,   20.0,   28.0,
            18.0,   22.0,   36.0,   44.0,
            6.0,    18.0,   8.0,    24.0,
            30.0,   42.0,   40.0,   56.0,
            54.0,   66.0,   72.0,   88.0
        ],
        row: 6,
        col: 4
    };

    assert_eq!(E1, T1);
}

#[test] fn test_elementary_row_swap_for_matrix()
{
    let T: Matrix<_> = Matrix {
        inner: vec![
            1.0,3.0,
            5.0,7.0,
            9.0,11.0
        ],
        row: 3,
        col: 2
    };
    let T1: _ = T.row_swap(0,1);
    let E1: Matrix<_> = Matrix {
        inner: vec![
            5.0,7.0,
            1.0,3.0,
            9.0,11.0
        ],
        row: 3,
        col: 2
    };
    assert_eq!(T1,E1);
}

#[test] fn test_elementary_row_add_for_matrix()
{
    let T: Matrix<_> = Matrix {
        inner: vec![
            1.0,3.0,
            5.0,7.0,
            9.0,11.0
        ],
        row: 3,
        col: 2
    };
    let T1: _ = (&T).row_add(None, 0,1);
    let E1: Matrix<_> = Matrix {
        inner: vec![
            6.0,10.0,
            5.0,7.0,
            9.0,11.0
        ],
        row: 3,
        col: 2
    };
    assert_eq!(E1, T1);
    let T2: _ = T.row_add(Some(2.0), 0, 1);
    let E2: Matrix<_> = Matrix {
        inner: vec![
            7.0,13.0,
            5.0,7.0,
            9.0,11.0
        ],
        row: 3,
        col: 2
    };
    assert_eq!(E2, T2);
}

#[test] fn test_elemntary_row_mul_for_matrix()
{
    let T: Matrix<_> = Matrix {
        inner: vec![
            1.0,3.0,
            5.0,7.0,
            9.0,11.0
        ],
        row: 3,
        col: 2
    };
    let T1: _ = (&T).row_mul(2.0, 2);
    let E1: Matrix<_> = Matrix {
        inner: vec![
            1.0,3.0,
            5.0,7.0,
            18.0,22.0
        ],
        row: 3,
        col: 2
    };   
    assert_eq!(E1, T1);
}