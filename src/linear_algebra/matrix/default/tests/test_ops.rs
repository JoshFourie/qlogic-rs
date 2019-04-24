use super ::*;

#[cfg(test)] mod test_mul_for_matrix {

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

}