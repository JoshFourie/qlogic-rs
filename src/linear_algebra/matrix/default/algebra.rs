/***** Imports ******/
use super::Matrix;
use crate::linear_algebra::interface::*;

/***** Implements *****/
macro_rules! impl_transpose_for_matrix {
    
    ($id:ty) => {
        impl<'a, T: Copy> Transpose for $id {

            type Output = Matrix<T>;
            
            fn transpose(self) -> Self::Output {
                let mut C: Vec<T> = Vec::new();
                let (r,c): (usize,usize) = (self.row, self.col);
                for i in 0..r {
                    for j in 0..c {
                        C.push(self[j][i])
                    }
                }
                Matrix {
                    inner: C,
                    row: r,
                    col: c
                }
            }    
        }
    }
}

impl_transpose_for_matrix!(Matrix<T>);
impl_transpose_for_matrix!(&'a Matrix<T>);
