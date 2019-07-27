use crate::vector;

use vector::interface;

use interface::{Scalar, Dot};

use std::ops;

impl<T:Copy> ops::Mul<T> for vector::Vector<T>
where
    T: ops::Mul<Output=T>
{
    type Output = Self;

    #[inline]
    fn mul(self, rhs: T) -> Self
    {
        self.scalar(rhs)
    }
}

impl<T:Copy> ops::Mul<Self> for vector::Vector<T>
where
    T: ops::Mul<Output=T>
    + ops::Add
    + num::Zero
{
    type Output = T;
    
    #[inline]
    fn mul(self, rhs: Self) -> T
    {
        self.dot(rhs)
    }
} 

/* 
STATUS: to deprecate.

WARNING: 

    This implementation of ops::Mul is an inline call to the interface::Scalar/Dot/Cross/Direct implementations.

    It relies on specialisation in the type parameters to distinguish between calls: Rust CAN NOT specialise based on Outputs at the moment.

    It is a broken implementation that is not necessary to implement at the moment.

default impl<T:Copy> ops::Mul<Self> for vector::Vector<T>
{
    type Output = Self;
    
    #[inline]
    fn mul(self, rhs: Self) -> Self
    {
        self.cross(rhs)
    }
} */

impl<T> interface::Cross for vector::Vector<T>
{
    fn cross(self, _rhs: Self) -> Self
    {
        unimplemented!()
    }
}

impl<T:Copy> interface::Direct for vector::Vector<T>
where
    T: ops::Mul<Output=T>
{
    type Output = crate::matrix::Matrix<T>;

    fn direct_product(self, rhs: Self) -> Self::Output 
    {
        use crate::matrix::interface::Kronecker;

        let new_col: usize = self.inner.len();
        let new_row: usize = rhs.inner.len();

        let mut A: crate::matrix::Matrix<T> = self.inner.into();
        A.col = new_col;
        A.row = 1;

        let mut B: crate::matrix::Matrix<T> = rhs.inner.into();
        B.col = 1;
        B.row = new_row;
        
        A.kronecker(B)
    }
}

impl<T> interface::Dot<T> for vector::Vector<T>
where
    T: ops::Mul<Output=T>
    + ops::Add
    + num::Zero
{
    fn dot(self, rhs: Self) -> T
    {
        self.into_iter()
            .zip(rhs.into_iter())
            .fold(T::zero(), |acc,(a,b)| acc + a*b )
    }
}

impl<T:Copy> interface::Scalar<T> for vector::Vector<T>
where
    T: ops::Mul<Output=T>
{
    fn scalar(self, rhs: T) -> Self 
    {
        self.into_iter()
            .map(|a| a*rhs)
            .collect::<Vec<T>>()
            .into()
    }
}

#[cfg(test)] mod tests 
{
    use crate::matrix;

    use crate::vector;

    use vector::interface::Direct;

    #[ignore] #[test] fn test_vector_cross() {
        unimplemented!();
    } 

    #[ignore] #[test] fn test_vector_direct() 
    {
        let A: vector::Vector<_> = vec![0,1,2,3,4,5,6].into();

        let B: vector::Vector<_> = vec![0,1,2,3,4,5].into();

        let test: matrix::Matrix<_> = A.direct_product(B);

        assert_eq!(test.row, 7);
        assert_eq!(test.col, 6);
    } 

    #[ignore] #[test] fn test_vector_dot() {
        unimplemented!();
    } 

    #[ignore] #[test] fn test_vector_scalar() {
        unimplemented!();
    } 

}