use std::ops;

use crate::matrix;
use matrix::interface;

impl<T:Copy> interface::SubMatrix for matrix::Matrix<T> {
    
    type Range = SubMatrixRange;

    fn insert_minor(mut self, rhs: Self, range: Self::Range) -> Self {
        let mut iter: _ = rhs.into_iter();
        for i in range.rows {
            for j in range.cols.clone() {    
                self[i][j] = iter.next().expect("size of minor does not correlate to matrix subsize")
            }
        }
        self
    }
}

#[derive(Clone, Debug)]
pub struct SubMatrixRange {
    cols: ops::Range<usize>,
    rows: ops::Range<usize>
}

impl SubMatrixRange {
    pub fn new(rows: ops::Range<usize>, cols: ops::Range<usize>) -> Self {
        Self{ cols, rows }
    }

    pub fn into_tuple(self) -> (ops::Range<usize>,ops::Range<usize>) {
        (self.rows, self.cols)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use interface::SubMatrix;
    use matrix::ops::submatrix;

    #[test]
    fn test_clone_to_minor() {
        let mat: matrix::Matrix<usize> = vec![0; 16].into();
        let exp: matrix::Matrix<usize> = vec![
            0,0,0,0,
            0,0,1,1,
            0,0,1,1,
            0,0,0,0,
        ].into();
        let min: matrix::Matrix<usize> = vec![1;4].into(); 
        let range: _ = submatrix::SubMatrixRange::new(1..3, 2..4);
        let test: _ = mat.insert_minor(min, range);
        assert_eq!(test,exp)
    }
}
