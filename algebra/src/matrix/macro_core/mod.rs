#[macro_use] mod structure;

#[macro_use] mod ops;

use num::integer::Roots;

use std::ops::{Add, Sub};

impl_into_vec!(&'a crate::matrix::Matrix<T>);
impl_into_vec!(&'a mut crate::matrix::Matrix<T>);

impl_index!(crate::matrix::Matrix<T>);
impl_index!(&'a mut crate::matrix::Matrix<T>);

impl<'a, T> std::ops::Index<usize> for &'a crate::matrix::Matrix<T>
{
    type Output = [T];

    fn index<'b>(&'b self,idx:usize) -> &'b Self::Output {
        let i0: usize = idx * self.col;
        let ter: usize = i0 + self.col;
        let i: std::ops::Range<usize> = i0..ter;
        &self.inner[i]
    }
}

impl_getter!(crate::matrix::Matrix<T>);
impl_getter!(&'a crate::matrix::Matrix<T>);
impl_getter!(&'a mut crate::matrix::Matrix<T>);

impl_row_col_traits!(crate::matrix::Matrix<T>);
impl_row_col_traits!(&'a crate::matrix::Matrix<T>);
impl_row_col_traits!(&'a mut crate::matrix::Matrix<T>);

impl_mul!(crate::matrix::Matrix<T>);
impl_mul!(&'a crate::matrix::Matrix<T>);
impl_mul!(&'a mut crate::matrix::Matrix<T>);
impl_mul!(crate::matrix::Matrix<T>, crate::matrix::Matrix<T>);
impl_mul!(crate::matrix::Matrix<T>, &'a crate::matrix::Matrix<T>);
impl_mul!(&'a crate::matrix::Matrix<T>, crate::matrix::Matrix<T>);
impl_mul!(&'a crate::matrix::Matrix<T>, &'a crate::matrix::Matrix<T>);
impl_mul!(crate::matrix::Matrix<T>, &'a mut crate::matrix::Matrix<T>);
impl_mul!(&'a mut crate::matrix::Matrix<T>, crate::matrix::Matrix<T>);
impl_mul!(&'a mut crate::matrix::Matrix<T>, &'a mut crate::matrix::Matrix<T>);

impl_add_or_sub!(crate::matrix::Matrix<T>, crate::matrix::Matrix<T>, Add, add, CheckedAdd, checked_add);
impl_add_or_sub!(&'a crate::matrix::Matrix<T>, crate::matrix::Matrix<T>, Add, add, CheckedAdd, checked_add);
impl_add_or_sub!(crate::matrix::Matrix<T>, &'a crate::matrix::Matrix<T>, Add, add, CheckedAdd, checked_add);
impl_add_or_sub!(&'a crate::matrix::Matrix<T>, &'a crate::matrix::Matrix<T>, Add, add, CheckedAdd, checked_add);

impl_add_or_sub!(crate::matrix::Matrix<T>, crate::matrix::Matrix<T>, Sub, sub, CheckedSub, checked_sub);
impl_add_or_sub!(&'a crate::matrix::Matrix<T>, crate::matrix::Matrix<T>, Sub, sub, CheckedSub, checked_sub);
impl_add_or_sub!(crate::matrix::Matrix<T>, &'a crate::matrix::Matrix<T>, Sub, sub, CheckedSub, checked_sub);
impl_add_or_sub!(&'a crate::matrix::Matrix<T>, &'a crate::matrix::Matrix<T>, Sub, sub, CheckedSub, checked_sub);

impl_add_or_sub!(&'a mut crate::matrix::Matrix<T>, crate::matrix::Matrix<T>, Add, add, CheckedAdd, checked_add);
impl_add_or_sub!(crate::matrix::Matrix<T>, &'a mut crate::matrix::Matrix<T>, Add, add, CheckedAdd, checked_add);
impl_add_or_sub!(&'a mut crate::matrix::Matrix<T>, &'a mut crate::matrix::Matrix<T>, Add, add, CheckedAdd, checked_add);

impl_add_or_sub!(&'a mut crate::matrix::Matrix<T>, crate::matrix::Matrix<T>, Sub, sub, CheckedSub, checked_sub);
impl_add_or_sub!(crate::matrix::Matrix<T>, &'a mut crate::matrix::Matrix<T>, Sub, sub, CheckedSub, checked_sub);
impl_add_or_sub!(&'a mut crate::matrix::Matrix<T>, &'a mut crate::matrix::Matrix<T>, Sub, sub, CheckedSub, checked_sub);

impl_identity!(crate::matrix::Matrix<T>);
impl_identity!(&'a crate::matrix::Matrix<T>);
impl_identity!(&'a mut crate::matrix::Matrix<T>);

#[cfg(test)] mod tests {

    use crate::matrix;

    use matrix::interface;

    #[test] fn test_unchecked_mul_for_matrix() {

        let M: matrix::Matrix<_> = matrix::Matrix {
            inner: vec![1,2,1,0,1,0,2,3,4],
            row: 3,
            col: 3
        };
        let N: matrix::Matrix<_> = matrix::Matrix {
            inner: vec![2,5,6,7,1,8],
            row: 3,
            col: 2
        };
        let T: _ = M * N;

        // we don't run structural checks.
        let E: _ = matrix::Matrix {
            inner: vec![15,27,6,7,26,63],
            row: 3,
            col: 2
        };

        assert_eq!(T.inner,E.inner);
    }

    #[test] fn test_unchecked_add_for_matrix() {
        
        let M: matrix::Matrix<_> = matrix::Matrix {
            inner: vec![1,2,3,4,5,6,7,8,9],
            row: 3,
            col: 3
        };
        
        let N: matrix::Matrix<_> = matrix::Matrix {
            inner: vec![1,2,3,4,5,6,7,8,9],
            row: 3,
            col: 3
        };   

        let T: _ = M + N;

        let E: matrix::Matrix<_> = matrix::Matrix {
            inner: vec![2,4,6,8,10,12,14,16,18],
            row: 3,
            col: 3
        };

        assert_eq!(T, E);
    }

    #[test] fn test_unchecked_sub_for_matrix() {

        let M: matrix::Matrix<_> = matrix::Matrix {
            inner: vec![1,2,3,4,5,6,7,8,9],
            row: 3,
            col: 3
        };
        
        let N: matrix::Matrix<_> = matrix::Matrix {
            inner: vec![1,2,3,4,5,6,7,8,9],
            row: 3,
            col: 3
        };   

        let T: _ = M - N;

        let E: matrix::Matrix<_> = matrix::Matrix {
            inner: vec![0; 9],
            row: 3,
            col: 3
        };

        assert_eq!(T,E);
    } 

    #[ignore] #[test] fn test_checked_mul_for_matrix() {
        
        use interface::CheckedMul;

        let M: matrix::Matrix<_> = matrix::Matrix {
            inner: vec![1,2,1,0,1,0,2,3,4],
            row: 3,
            col: 3
        };
        let N: matrix::Matrix<_> = matrix::Matrix {
            inner: vec![2,5,6,7,1,8],
            row: 2,
            col: 3
        };
        match M.checked_mul(N) {
            _ => unimplemented!()
        }

    }


    #[ignore] #[test] fn test_checked_add_for_matrix() {

        use interface::CheckedAdd;
        
        let M: matrix::Matrix<_> = matrix::Matrix {
            inner: vec![1,2,3,4,5],
            row: 2,
            col: 3
        };
        
        let N: matrix::Matrix<_> = matrix::Matrix {
            inner: vec![1,2,3,4,5,6,7,8,9],
            row: 3,
            col: 3
        };   
        match M.checked_add(N) {
            _ => unimplemented!()
        }
    }

    #[ignore] #[test] fn test_checked_sub_for_matrix() {

        use interface::CheckedSub;

        let M: matrix::Matrix<_> = matrix::Matrix {
            inner: vec![1,2,3,4,5,6],
            row: 2,
            col: 3
        };
        
        let N: matrix::Matrix<_> = matrix::Matrix {
            inner: vec![1,2,3,4,5,6,7,8,9],
            row: 3,
            col: 3
        };   
        match M.checked_sub(N) {
            _ => unimplemented!()
        }
    } 


}