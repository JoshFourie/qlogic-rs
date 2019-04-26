#[test] fn test_unchecked_mul_for_matrix() {

    let M: super::Matrix<_> = super::Matrix {
        inner: vec![1,2,1,0,1,0,2,3,4],
        row: 3,
        col: 3
    };
    let N: super::Matrix<_> = super::Matrix {
        inner: vec![2,5,6,7,1,8],
        row: 3,
        col: 2
    };
    let T: _ = M * N;

    let E: _ = super::Matrix {
        inner: vec![15,27,6,7,26,63],
        row: 3,
        col: 2
    };

    assert_eq!(T,E);
}

#[test] fn test_unchecked_add_for_matrix() {
    
    let M: super::Matrix<_> = super::Matrix {
        inner: vec![1,2,3,4,5,6,7,8,9],
        row: 3,
        col: 3
    };
    
    let N: super::Matrix<_> = super::Matrix {
        inner: vec![1,2,3,4,5,6,7,8,9],
        row: 3,
        col: 3
    };   

    let T: _ = M + N;

    let E: super::Matrix<_> = super::Matrix {
        inner: vec![2,4,6,8,10,12,14,16,18],
        row: 3,
        col: 3
    };

    assert_eq!(T, E);
}

#[test] fn test_unchecked_sub_for_matrix() {

    let M: super::Matrix<_> = super::Matrix {
        inner: vec![1,2,3,4,5,6,7,8,9],
        row: 3,
        col: 3
    };
    
    let N: super::Matrix<_> = super::Matrix {
        inner: vec![1,2,3,4,5,6,7,8,9],
        row: 3,
        col: 3
    };   

    let T: _ = M - N;

    let E: super::Matrix<_> = super::Matrix {
        inner: vec![0; 9],
        row: 3,
        col: 3
    };

    assert_eq!(T,E);
} 

#[test] fn test_checked_mul_for_matrix() {
    
    use super::{CheckedMul, ErrorKind};

    let M: super::Matrix<_> = super::Matrix {
        inner: vec![1,2,1,0,1,0,2,3,4],
        row: 3,
        col: 3
    };
    let N: super::Matrix<_> = super::Matrix {
        inner: vec![2,5,6,7,1,8],
        row: 2,
        col: 3
    };
    match M.checked_mul(N) {
        Err(e) => match e.kind() {
            ErrorKind::MatrixStructure => { },
            _ => panic!()
        },
        Ok(_) => {}
    }

}


#[test] fn test_checked_add_for_matrix() {

    use super::{CheckedAdd, ErrorKind};
    
    let M: super::Matrix<_> = super::Matrix {
        inner: vec![1,2,3,4,5],
        row: 2,
        col: 3
    };
    
    let N: super::Matrix<_> = super::Matrix {
        inner: vec![1,2,3,4,5,6,7,8,9],
        row: 3,
        col: 3
    };   
    match M.checked_add(N) {
        Err(e) => match e.kind() {
            ErrorKind::MatrixStructure => { },
            _ => panic!()
        },
        Ok(_) => {}
    }
}

#[test] fn test_checked_sub_for_matrix() {

    use super::{CheckedSub, ErrorKind};

    let M: super::Matrix<_> = super::Matrix {
        inner: vec![1,2,3,4,5,6],
        row: 2,
        col: 3
    };
    
    let N: super::Matrix<_> = super::Matrix {
        inner: vec![1,2,3,4,5,6,7,8,9],
        row: 3,
        col: 3
    };   
    match M.checked_sub(N) {
        Err(e) => match e.kind() {
            ErrorKind::MatrixStructure => { },
            _ => panic!()
        },
        Ok(_) => {}
    }
} 
