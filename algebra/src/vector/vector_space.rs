use std::{ops, iter};
use ops::{Add, Mul, Neg};
use iter::FromIterator;


pub trait VectorSpace
{
    type Scalar;

    type Vector;

    fn dimensions(&self) -> usize;
}

pub trait VAdd
{
    type Input;

    type Output;

    fn vadd(&self, lhs: &Self::Input, rhs: &Self::Input) -> Self::Output;
}

impl<U> VAdd for U
where
    U: VectorSpace,
    U::Vector: FromIterator<U::Scalar>,
    for <'a> &'a U::Vector: IntoIterator<Item=&'a U::Scalar>,
    for <'a> &'a U::Scalar: Add<&'a U::Scalar, Output=U::Scalar>,
{
    type Input = U::Vector;

    type Output = U::Vector;

    fn vadd(&self, lhs: &Self::Input, rhs: &Self::Input) -> Self::Output
    {
        lhs
            .into_iter()
            .zip( rhs.into_iter() )
            .map(|(l,r)| l + r)
            .collect()
    }
}


pub trait VScale 
{
    type Scalar;

    type Vector;

    type Output;

    fn vscale(&self, scalar: Self::Scalar, vector: Self::Vector) -> Self::Output;
}

impl<U> VScale for U
where
    U: VectorSpace,
    U::Vector: IntoIterator<Item=U::Scalar> + FromIterator<U::Scalar>,
    U::Scalar: Copy + Mul<U::Scalar, Output=U::Scalar>,
{
    type Scalar = U::Scalar;

    type Vector = U::Vector;

    type Output = U::Vector;

    fn vscale(&self, scalar: Self::Scalar, vector: Self::Vector) -> Self::Output        
    {
        vector
            .into_iter()
            .map(|val| scalar * val)
            .collect()
    }
}


pub trait VIdentity: VMultiplicativeIdent + VAdditiveIdent
{
    // Supertrait.
}  

impl<U> VIdentity for U
where
    U: VMultiplicativeIdent + VAdditiveIdent
{
    // Empty.
}

pub trait VAdditiveIdent
{
    type Output;

    fn additive_ident(&self) -> Self::Output;    
}

pub trait VMultiplicativeIdent
{
    type Output;

    fn mul_ident(&self) -> Self::Output;
}


pub trait VAdditiveInverse
{
    type Vector;

    type Output;

    fn additive_inv(&self, vector: Self::Vector) -> Self::Output;
}

impl<U> VAdditiveInverse for U
where
    U: VectorSpace,
    U::Vector: IntoIterator<Item=U::Scalar> + FromIterator<U::Scalar>,
    U::Scalar: Neg<Output=U::Scalar>
{
    type Vector = U::Vector;

    type Output = U::Vector;

    fn additive_inv(&self, vector: Self::Vector) -> Self::Vector 
    {
        vector
            .into_iter()
            .map(|val| -val)
            .collect()
    }
}


#[macro_export]
macro_rules! vadd 
{
    ($vector_space:expr, $lhs:expr, $($rhs:expr),+) => {
        {
            use crate::vector::VAdd;
            $vector_space.vadd($lhs, &crate::vadd!($vector_space, $($rhs),+))
        }
    };
    ($vector_space:expr, $lhs:expr) => { $lhs };
}

#[macro_export]
macro_rules! vscale 
{
    ($vector_space:expr, $lhs:expr, $($rhs:expr),+) => {
        {
            use crate::vector::VScale;
            $vector_space.vscale($lhs, &crate::vscale!($vector_space, $($rhs),+))
        }
    };
    ($vector_space:expr, $lhs:expr, ) => { { $lhs } };
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests 
{
    use crate::{vadd, vscale};
    use super::{VectorSpace, VAdd, VScale, VAdditiveIdent, VMultiplicativeIdent, VAdditiveInverse};

    struct DummyVectorSpace;

    type Vector3 = Vec<isize>;

    impl VectorSpace for DummyVectorSpace 
    {
        type Scalar = isize;

        type Vector = Vector3;

        fn dimensions(&self) -> usize 
        {
            3
        }
    }

    impl<'a> VectorSpace for &'a DummyVectorSpace 
    {
        type Scalar = isize;

        type Vector = Vector3;

        fn dimensions(&self) -> usize 
        {
            3
        }
    }

    impl VAdditiveIdent for DummyVectorSpace
    {
        type Output = Vector3;

        fn additive_ident(&self) -> Self::Output 
        {
            vec![0; 3]
        }
    }

    impl VMultiplicativeIdent for DummyVectorSpace
    {
        type Output = isize;

        fn mul_ident(&self) -> Self::Output 
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
        let test = vector_space.vadd(&x, &y);

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

        let lhs = vector_space.vadd(&x, &y);
        let rhs = vector_space.vadd(&y, &x);
        assert_eq!(lhs, rhs);
    }

    #[test]
    fn test_associative_addition()
    {
        let vector_space = DummyVectorSpace;
        let x = vec![ 3, 1, 5 ];
        let y = vec![ 6, 2, 7 ];
        let z = vec![ 4, 5, 1 ];

        let lhs: Vec<isize> = vadd!(vector_space, &x, &y, &z);
        let rhs: Vec<isize> = vadd!(vector_space, &y, &z, &x);
        assert_eq!(lhs, rhs);
    }

    #[test]
    fn test_additive_ident()
    {
        let vector_space = DummyVectorSpace;
        let exp = vec![ 0, 0, 0 ];

        let test = vector_space.additive_ident();
        assert_eq!(test, exp);
    }

    #[test]
    fn test_additive_inverse()
    {
        let vector_space = DummyVectorSpace;
        let x: Vec<isize> = vec![ 3, 1, 5 ];
        let exp: Vec<isize> = vec![ -3, -1, -5 ];
        
        let test: Vec<isize> = vector_space.additive_inv(x);
        assert_eq!(test, exp);
    }

    #[test]
    fn test_vadd() {
        use crate::vadd;

        let vector_space = DummyVectorSpace;
        
        let x: Vec<isize> = vec![ 3, 1, 5 ];
        let y: Vec<isize> = vec![ 6, 2, 7 ];
        let z: Vec<isize> = vec![ 4, 5, 1 ];
        let test: Vec<isize> = vadd!(vector_space, &x, &y, &z);

        let exp: Vec<isize> = vec![ 13, 8, 13];
        assert_eq!(test, exp);
    }
}
