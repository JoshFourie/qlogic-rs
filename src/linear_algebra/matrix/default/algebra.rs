/***** Imports ******/
use super::Matrix;
use crate::linear_algebra::interface::*;

/***** Implements *****/
impl<T> StdOps<T> for Matrix<T> 
{
    fn transpose(self) -> Self {
        
    }
}

#[cfg(test)] mod StdOpsTest {

    #[test] fn test_transpose_for_matrix() {
        let T: super::Matrix<_> = super::Matrix {
            inner: vec![0,1,2,3,4,5,6,7,8],
            row: 3,
            col: 3,
        };
        let E: super::Matrix<_> = super::Matrix {
            inner: vec![0,3,6,1,4,7]
        }
    }

}