#[test] fn index_usize_test() {
    let M: _ = super::Matrix {
        inner: vec![0,1,2,3,4,5,6,7,8],
        col: 3,
        row: 3
    };
    assert_eq!(M[2][2],8);
}

#[test] 
fn index_usize_mut_test() {
    let mut M: super::Matrix<usize> = super::Matrix {
        inner: vec![0,1,2,3,4,5,6,7,8],
        col: 3,
        row: 3
    };
    let x: Vec<usize> = M[2].into();
    M[2][2] = 3; 

    assert_eq!(x, vec![6,7,8]);
    assert_eq!(3, M[2][2]);
    assert_eq!(vec![0,1,2,3,4,5,6,7,3], M.inner);
}