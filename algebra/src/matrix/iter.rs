pub struct MatrixIter<T> {
    mat: Vec<T>,
    count: usize,
}

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

macro_rules! impl_into_iter {

    ($id:ty) => {
        impl<'a, T:Copy> IntoIterator for $id {
            type Item = T;
            type IntoIter = crate::matrix::iter::MatrixIter<T>;
            
            fn into_iter(self) -> Self::IntoIter 
            {
                crate::matrix::iter::MatrixIter {
                    mat: self.into(),
                    count: 0,
                }
            }
        }
    }
}

impl_into_iter!(crate::matrix::Matrix<T>);
impl_into_iter!(&'a crate::matrix::Matrix<T>);
impl_into_iter!(&'a mut crate::matrix::Matrix<T>);