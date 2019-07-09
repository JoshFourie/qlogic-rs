macro_rules! impl_mul {
    ($id:ty) => {
        impl<'a, T: Copy> std::ops::Mul<T> for $id
        where
            T: std::ops::Mul<Output=T>,
        {
            type Output = crate::matrix::Matrix<T>;

            fn mul(self, rhs: T) -> crate::matrix::Matrix<T> { 
                let inner: Vec<T> = self.into_iter()
                    .map(|l| l * rhs)
                    .collect();
                let l: usize = inner.len().sqrt();
                crate::matrix::Matrix {
                    inner: inner,
                    row: l,
                    col: l
                }
            }
        }
    };
    ($lhs:ty, $rhs:ty) => {
        impl<'a, T: Copy> std::ops::Mul<$rhs> for $lhs
        where
            T: std::ops::Mul<Output=T> + num::Zero
        {
            type Output = crate::matrix::Matrix<T>;

            fn mul(self, rhs: $rhs) -> Self::Output {
                let mut C: Vec<T> = Vec::new();
                for i in 0..self.row {
                    for j in 0..rhs.col {
                        let mut sigma: T = T::zero();
                        for k in 0..rhs.row
                        {
                            sigma = sigma + self[i][k] * rhs[k][j];
                        }
                        C.push(sigma);
                    }
                }
                crate::matrix::Matrix {
                    inner: C,
                    row: self.row,
                    col: self.col
                }
            }
        }
    
        impl<'a, T: Copy> crate::matrix::interface::CheckedMul<$rhs> for $lhs
        where
            T: std::ops::Mul<Output=T> + num::Zero
        {
            type Output = crate::matrix::interface::Result<crate::matrix::Matrix<T>>;

            fn checked_mul(self, rhs: $rhs) -> Self::Output 
            {
                if self.col != rhs.row {
                    Err(crate::error::AlgebraError::from(crate::error::ErrorKind::MatrixStructure))
                } else { 
                    Ok(self * rhs)
                }
            }
        }

        /*********** Mul<Vec> *************/ 
    }
}

macro_rules! impl_add_or_sub 
{
    ($lhs:ty, $rhs:ty, $unchecked:ident, $func:ident, $checked:ident, $checked_func:ident) => 
    {
        impl<'a, T: Copy> std::ops::$unchecked<$rhs> for $lhs 
        where
            T: std::ops::$unchecked<T,Output=T>
        {
            type Output = crate::matrix::Matrix<T>;

            fn $func(self, rhs: $rhs) -> Self::Output {
                let (r,c): (usize,usize) = (self.row,self.col);
                let C: Vec<T> = self.into_iter()
                    .zip(rhs.into_iter())
                    .map(|(l,r)| l.$func(r))
                    .collect();
                crate::matrix::Matrix {
                    inner: C,
                    row: r,
                    col: c
                }
            }
        }  

        impl<'a, T: Copy> crate::matrix::interface::$checked<$rhs> for $lhs 
        where
            T: std::ops::$unchecked<T,Output=T>
        {
            type Output = crate::matrix::interface::Result<crate::matrix::Matrix<T>>;

            fn $checked_func(self, rhs: $rhs) -> Self::Output {
                if self.col == rhs.col && self.row == self.col {
                    Ok(self.$func(rhs))
                } else { 
                    Err(crate::error::AlgebraError::from(crate::error::ErrorKind::MatrixStructure))
                }
            }
        }
    } 
}

macro_rules! impl_identity {
    ($s:ty) => {
        impl<'a, T: Clone> crate::matrix::interface::Identity for $s 
        where
            T: num::Zero + num::One
        {
            type Output = crate::matrix::Matrix<T>;

            fn identity(self) -> Self::Output {
                let mut I: crate::matrix::Matrix<T> = crate::matrix::Matrix {
                    inner: vec![T::zero(); self.row * self.col],
                    row: self.row,
                    col: self.col
                };
                for i in 0..self.row {
                    I[i][i] = T::one();
                }
                I
            }
        }
    }
}