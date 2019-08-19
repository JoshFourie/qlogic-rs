use std::ops;

use crate::matrix;
use matrix::interface;

mod matrix_minor;
pub use matrix_minor::*;

impl<T:Copy> interface::Minor<MinorRange> for matrix::Matrix<T> 
where
    T: ops::Mul<T,Output=T> + num::Zero
{
    type Output = MatrixMinor<T>;

    fn minor(&self, range: MinorRange) -> Self::Output {
        let (row,col): _ = range.clone().into_tuple();
        let mut buf: Vec<T> = Vec::new();
        let new_row: usize = row.end - row.start;
        let new_col: usize = col.end - col.start;

        for i in row {
            for j in col.clone() {
                buf.push(self[i][j]);
            }
        }
        
        let mat: _ = matrix::Matrix::new(buf, new_row, new_col);
        MatrixMinor::new(mat, range)
    }       

    fn clone_from_minor(&mut self, rhs: MatrixMinor<T>) {
        let range: MinorRange = rhs.range;
        let mut iter: _ = rhs.mat.into_iter();
            
        for i in range.rows {
            for j in range.cols.clone() { 
                (*self)[i][j] = iter.next().expect("size of minor does not correlate to matrix subsize")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::matrix::interface::Minor;

    #[test]
    fn test_clone_to_minor() {
        let mut test: matrix::Matrix<usize> = vec![0; 16].into();
        let exp: matrix::Matrix<usize> = vec![
            0,0,0,0,
            0,0,1,1,
            0,0,1,1,
            0,0,0,0,
        ].into();

        let buf: matrix::Matrix<usize> = vec![1;4].into(); 
        let range: _ = MinorRange::new(1..3, 2..4);
        let minor: _ = MatrixMinor::new(buf, range);

        test.clone_from_minor(minor);
        assert_eq!(test,exp)
    }

    #[test]
    fn test_multiply_to_minor() {
        let mut mat: matrix::Matrix<usize> = vec![1; 16].into();
        let operator: matrix::Matrix<usize> = vec![2;4].into();

        let test: matrix::Matrix<usize> = {
            let range: _ = MinorRange::new(1..3, 2..4);
            let buf: _ = mat.minor(range);
            let minor: MatrixMinor<usize> = operator * buf;
            mat.clone_from_minor(minor);
            mat
        };

        let exp: matrix::Matrix<usize> = vec![
            1,1,1,1,
            1,1,4,4,
            1,1,4,4,
            1,1,1,1,
        ].into();

        assert_eq!(test,exp)
    }
    
    #[test] fn test_minor() {
        let A: matrix::Matrix<usize> = vec![
            0,0,0,0,0,
            0,1,1,1,0,
            0,1,1,1,0,
            0,1,1,1,0,
            0,0,0,0,0,
        ].into();

        let test: matrix::Matrix<usize> = A.minor(MinorRange::new(1..3, 1..4)).into();
        let exp: matrix::Matrix<usize> = matrix::Matrix::new(vec![
            1,1,1,
            1,1,1,
        ],2,3);

        assert_eq!(test,exp);
    }
}
