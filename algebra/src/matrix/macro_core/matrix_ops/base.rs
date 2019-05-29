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
    
        impl<'a, T: Copy> crate::interface::CheckedMul<$rhs> for $lhs
        where
            T: std::ops::Mul<Output=T> + num::Zero
        {
            type Output = crate::interface::Result<crate::matrix::Matrix<T>>;

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

        impl<'a, T: Copy> crate::interface::$checked<$rhs> for $lhs 
        where
            T: std::ops::$unchecked<T,Output=T>
        {
            type Output = crate::interface::Result<crate::matrix::Matrix<T>>;

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
        impl<'a, T: Clone> crate::interface::Identity for $s 
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

macro_rules! impl_elem_row_operations
{
    ($id:ty) => 
    {
        impl<'a, T: Copy> crate::interface::ElementaryRowOperations<T,usize> for $id
        where
            T: std::ops::Add<Output=T> + std::ops::Mul<Output=T> + num::One
        {
            type Output = crate::matrix::Matrix<T>;
    
            fn row_swap(self, r1: usize, r2: usize) -> Self::Output
            {
                let mut mat: crate::matrix::Matrix<T> = self.clone();
                for c in 0..self.col {
                    mat[r1][c] = self[r2][c];
                    mat[r2][c] = self[r1][c];
                }
                mat
            }

            fn row_add(self, scalar: Option<T>, lhs: usize, rhs: usize) -> Self::Output
            {
                let mut mat: crate::matrix::Matrix<T> = self.clone();
                let scal: T = match scalar {
                    Some(s) => s,
                    None => T::one()
                };
                for c in 0..self.col {
                    mat[lhs][c] = scal * self[lhs][c] + self[rhs][c];
                }
                mat
            }

            fn row_mul(self, scal: T, r: usize) -> Self::Output
            {
                let mut mat: crate::matrix::Matrix<T> = self.clone();
                for c in 0..self.col {
                    mat[r][c] = scal * mat[r][c];   
                }
                mat
            }
        }
    }
}