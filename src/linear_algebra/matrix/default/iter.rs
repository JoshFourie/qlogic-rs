/***** Imports ******/

use super::Matrix;

/**** Struct ******/
pub struct MatrixIter<T> {
    mat: Vec<T>,
    count: usize,
}

/**** Std. Impl *****/
impl<T:Copy> Iterator for MatrixIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        let idx: usize = self.count;
        if self.mat.len() > self.count {
            self.count += 1;
            Some(self.mat[idx])
        } else { None }
    }
}

macro_rules! imlp_into_iter_for_matrix {

    ($id:ty) => {
        impl<'a, T:Copy> IntoIterator for $id {
            type Item = T;
            type IntoIter = MatrixIter<T>;
            
            fn into_iter(self) -> Self::IntoIter 
            {
                MatrixIter {
                    mat: self.into(),
                    count: 0,
                }
            }
        }
    }

}

imlp_into_iter_for_matrix!(Matrix<T>);
imlp_into_iter_for_matrix!(&'a Matrix<T>);
imlp_into_iter_for_matrix!(&'a mut Matrix<T>);

#[cfg(test)] mod iter_test_for_matrix {

    #[test] fn matrix_into_iter_test() {
        let test: Vec<_> = vec![0,1,2,3,4,5,6,7,8];
        let M: super::Matrix<_> = test.clone().into();
        for (t,e) in test.into_iter()
            .zip(M.into_iter()) 
        {
            assert_eq!(t,e);
        }
    }

    #[test] fn matrix_borrow_into_iter_test() {
        let test: Vec<_> = vec![0,1,2,3,4,5,6,7,8];
        let M: super::Matrix<_> = test.clone().into();
        for (t,e) in test.clone().into_iter()
            .zip((&M).into_iter()) 
        {
            assert_eq!(t,e);
        }
        let _A = M;        
    }
}