/***** Imports *******/
use super::Matrix;
use std::ops::{Mul, Add, Index, IndexMut, Range};

/**** Std. Impl. *****/
impl<T> Index<(usize,usize)> for Matrix<T>
{
    type Output = T;
    fn index<'a>(&'a self, idx: (usize,usize)) -> &'a T {
        let (r,c): (usize,usize) = idx;
        let i = r.mul(self.col).add(c);
        &self.inner[i]
    }
}

impl<T> IndexMut<(usize,usize)> for Matrix<T>
{
    fn index_mut<'a>(&'a mut self, idx:(usize,usize)) -> &'a mut T {
        let (r,c): (usize,usize) = idx;
        let i = r.mul(self.col).add(c);
        &mut self.inner[i]   
    }    
}

// row extraction by indexing.
impl<T> Index<usize> for Matrix<T>
{
    type Output = [T];

    fn index<'a>(&'a self,idx:usize) -> &'a Self::Output {
        let i0: usize = idx.mul(self.col);
        let ter: usize = i0.add(self.col);
        let i: Range<usize> = i0..ter;
        &self.inner[i]
    }
}

impl<T> IndexMut<usize> for Matrix<T>
{
    fn index_mut<'a>(&'a mut self, idx:usize) -> &'a mut Self::Output {
        let i0: usize = idx.mul(self.col);
        let ter: usize = i0.add(self.col);
        let i: Range<usize> = i0..ter;
        &mut self.inner[i]
    }
}

#[cfg(test)] mod index_test_for_matrix 
{
    #[test] fn index_tuple_test() {
        let M: _ = super::Matrix {
            inner: vec![0,1,2,3,4,5,6,7,8],
            col: 3,
            row: 3
        };
        assert_eq!(M[(1,1)], 4);
        assert_eq!(M[(2,2)],8);
    }

    #[test] fn index_mut_tuple_test() {
        let mut M: _ = super::Matrix {
            inner: vec![0,0,0,0,0,0,0,0,0],
            col: 3,
            row: 3
        };
        let E: _ = super::Matrix {
            inner: vec![1,0,0,0,1,0,0,0,1],
            col: 3,
            row: 3
        };
        for i in 0..3 {
            M[(i,i)] = 1;
        }
        assert_eq!(M,E);            
    }

    #[test] fn index_usize_test() {
        let M: _ = super::Matrix {
            inner: vec![0,1,2,3,4,5,6,7,8],
            col: 3,
            row: 3
        };
        assert_eq!(M[2][2],8);
    }

    #[test] fn index_usize_mut_test() {
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

}