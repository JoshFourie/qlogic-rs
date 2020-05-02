use std::{ops, iter};
use ops::{Index, IndexMut};
use iter::FromIterator;

pub trait Vector<T>: VMultiplicativeIdent<Scalar=T> + VAdditiveIdent + IntoIterator<Item=T> + FromIterator<T>
{
    // Supertrait.
}  


pub trait VectorSpace<T>: VAdd<T> + VScale<T>
{
    type Vector: Vector<T>;
}


pub trait VAdd<T>
{   
    type AdditiveVector: Vector<T>;

    fn vadd(&self, lhs: Self::AdditiveVector, rhs: Self::AdditiveVector) -> Self::AdditiveVector;
}

impl<T,U> VAdd<T> for U
where
    U: VectorSpace<T>,
    T: Copy + ops::Add<T,Output=T>
{
    type AdditiveVector = <U as VectorSpace<T>>::Vector;

    fn vadd(&self, lhs: Self::AdditiveVector, rhs: Self::AdditiveVector) -> Self::AdditiveVector 
    {
        lhs.into_iter()
            .zip(rhs)
            .map(|(l,r)| l + r)
            .collect()
    }
}

pub trait VScale<T> 
{
    type ScalarVector: Vector<T>;

    fn vscale(&self, scalar: T, vector: Self::ScalarVector) -> Self::ScalarVector;
}

impl<T,U> VScale<T> for U
where
    U: VectorSpace<T>,
    T: Copy + ops::Mul<T,Output=T>
{
    type ScalarVector = <U as VectorSpace<T>>::Vector;

    fn vscale(&self, scalar: T, vector: Self::ScalarVector) -> Self::ScalarVector 
    {
        vector.into_iter()
            .map(|val| scalar * val)
            .collect()
    }
}


pub trait VAdditiveIdent
{
    type Output;

    fn additive_ident(&self) -> Self::Output;    
}

pub trait VMultiplicativeIdent
{
    type Scalar;

    fn mul_ident(&self) -> Self::Scalar;
}


#[macro_export]
macro_rules! vadd {
    ($vector_space:expr, $lhs:expr, $($rhs:expr),+) => {
        {
            use crate::vector::VAdd;
            $vector_space.vadd($lhs, crate::vadd!($vector_space, $($rhs),+))
        }
    };
    ($vector_space:expr, $lhs:expr) => { { $lhs } };
}

#[macro_export]
macro_rules! vscale {
    ($vector_space:expr, $lhs:expr, $($rhs:expr),+) => {
        {
            use crate::vector::VScale;
            $vector_space.vscale($lhs, crate::vscale!($vector_space, $($rhs),+))
        }
    };
    ($vector_space:expr, $lhs:expr) => { { $lhs } };
}


#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::{VectorSpace, VAdd, VAdditiveIdent, VMultiplicativeIdent,  VScale, Vector};

    struct DummyVectorSpace;

    type Vector3 = [isize; 3];

    impl VectorSpace<isize> for DummyVectorSpace 
    {
        type Vector = Vector3;
    }

    impl Vector<isize> for Vec<isize> { }

    impl VAdditiveIdent for Vec<isize>
    {
        type Output = Self;

        fn additive_ident(&self) -> Self::Output 
        {
            (0..length).map(|_| 0_isize).collect()
        }
    }

    impl VMultiplicativeIdent for Vec<isize>
    {
        type Scalar = isize;

        fn mul_ident(&self) -> Self::Scalar 
        {
            1_isize
        }
    }

    #[test]
    fn test_addition() 
    {
        let vector_space = DummyVectorSpace;
        let x = vec![ 3, 0, -1 ];
        let y = vec![ 10, 1, 2 ];

        let exp = vec![ 13, 1, 1 ];
        let test = vector_space.vadd(x, y);

        assert_eq!(exp, test);
    }

    #[test]
    fn test_multiplication()
    {
        let vector_space = DummyVectorSpace;
        let x = vec![ 3, 0, -1 ];
        let c = 2;

        let exp = vec![ 6, 0, -2 ];
        let test = vector_space.vscale(c, x);
        assert_eq!(exp, test);
    }

    #[test]
    fn test_commutative()
    {
        let vector_space = DummyVectorSpace;
        let x = vec![ 3, 1, 5 ];
        let y = vec![ 6, 2, 7 ];

        let lhs = vector_space.vadd(x.clone(), y.clone());
        let rhs = vector_space.vadd(y, x);
        assert_eq!(lhs, rhs);
    }

    #[test]
    fn test_associative()
    {
        let vector_space = DummyVectorSpace;
        let x = vec![ 3, 1, 5 ];
        let y = vec![ 6, 2, 7 ];
        let z = vec![ 4, 5, 1 ];

        let lhs: Vec<isize> = vadd!(vector_space, x.clone(), y.clone(), z.clone());
        let rhs: Vec<isize> = vadd!(vector_space, y, z, x);
        assert_eq!(lhs, rhs);
    }

    #[test]
    fn test_additive_ident()
    {
        let vector_space = DummyVectorSpace;
        let x = vec![ 3, 1, 5 ];
        let exp = vec![ 0, 0, 0 ];
    }

    #[test]
    fn test_vadd() {
        use crate::vadd;

        let vector_space = DummyVectorSpace;
        
        let x: Vec<isize> = vec![ 3, 1, 5 ];
        let y: Vec<isize> = vec![ 6, 2, 7 ];
        let z: Vec<isize> = vec![ 4, 5, 1 ];
        let test: Vec<isize> = vadd!(vector_space, x, y, z);

        let exp: Vec<isize> = vec![ 13, 8, 13];
        assert_eq!(test, exp);
    }
}
