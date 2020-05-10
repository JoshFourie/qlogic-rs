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

    fn vscale(&self, scalar: &Self::Scalar, vector: &Self::Vector) -> Self::Output;
}

impl<U> VScale for U
where
    U: VectorSpace,
    U::Vector: FromIterator<U::Scalar>,
    for <'a> &'a U::Vector: IntoIterator<Item=&'a U::Scalar>,
    for <'a> &'a U::Scalar: Mul<&'a U::Scalar, Output=U::Scalar>,
{
    type Scalar = U::Scalar;

    type Vector = U::Vector;

    type Output = U::Vector;

    fn vscale(&self, scalar: &Self::Scalar, vector: &Self::Vector) -> Self::Output        
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
    U::Vector: FromIterator<U::Scalar>,
    for <'a> &'a U::Vector: IntoIterator<Item=&'a U::Scalar>,
    for <'a> &'a U::Scalar: Neg<Output=U::Scalar>,
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


#[cfg(test)]
#[allow(non_snake_case)]
mod tests 
{
    use crate::{vadd};
    use super::{VectorSpace, VAdd, VScale, VAdditiveIdent, VMultiplicativeIdent, VAdditiveInverse};

    use std::iter::FromIterator;

    struct DummyVectorSpace;

    #[derive(Debug, PartialEq)]
    struct Vector3([isize; 3]);

    impl<'a> IntoIterator for &'a Vector3
    {
        type Item = &'a isize;
        type IntoIter = std::slice::Iter<'a, isize>;

        fn into_iter(self) -> Self::IntoIter {
            self.0.iter()
        }
    }

    impl<'a> FromIterator<isize> for Vector3
    {
        fn from_iter<T>(iter: T) -> Self 
        where
            T: IntoIterator<Item = isize>
        {
            let mut buf: [isize; 3] = [0; 3];
            for (idx, item) in iter
                .into_iter()
                .enumerate() 
            {
                assert!(idx < 3);
                buf[idx] = item;
            }
            Vector3(buf)
        }
    }

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
            Vector3([0; 3])
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
        let x = Vector3([ 3, 0, -1 ]);
        let y = Vector3([ 10, 1, 2 ]);

        let exp = Vector3([ 13, 1, 1 ]);
        let test = vector_space.vadd(&x, &y);

        assert_eq!(exp, test);
    }

    #[test]
    fn test_multiplication()
    {
        let vector_space = DummyVectorSpace;
        let x = Vector3([ 3, 0, -1 ]);
        let c = 2;

        let exp = Vector3([ 6, 0, -2 ]);
        let test = vector_space.vscale(&c, &x);
        assert_eq!(exp, test);
    }

    #[test]
    fn test_commutative()
    {
        let vector_space = DummyVectorSpace;
        let x = Vector3([ 3, 1, 5 ]);
        let y = Vector3([ 6, 2, 7 ]);

        let lhs = vector_space.vadd(&x, &y);
        let rhs = vector_space.vadd(&y, &x);
        assert_eq!(lhs, rhs);
    }

    #[test]
    fn test_associative_addition()
    {
        let vector_space = DummyVectorSpace;
        let x = Vector3([ 3, 1, 5 ]);
        let y = Vector3([ 6, 2, 7 ]);
        let z = Vector3([ 4, 5, 1 ]);

        let lhs: Vector3 = vadd!(vector_space, &x, &y, &z);
        let rhs: Vector3 = vadd!(vector_space, &y, &z, &x);
        assert_eq!(lhs, rhs);
    }

    #[test]
    fn test_additive_ident()
    {
        let vector_space = DummyVectorSpace;
        let exp = Vector3([ 0, 0, 0 ]);

        let test = vector_space.additive_ident();
        assert_eq!(test, exp);
    }

    #[test]
    fn test_additive_inverse()
    {
        let vector_space = DummyVectorSpace;
        let x: Vector3 = Vector3([ 3, 1, 5 ]);
        let exp: Vector3 = Vector3([ -3, -1, -5 ]);
        
        let test: Vector3 = vector_space.additive_inv(x);
        assert_eq!(test, exp);
    }

    #[test]
    fn test_vadd() {
        use crate::vadd;

        let vector_space = DummyVectorSpace;
        
        let x: Vector3 = Vector3([ 3, 1, 5 ]);
        let y: Vector3 = Vector3([ 6, 2, 7 ]);
        let z: Vector3 = Vector3([ 4, 5, 1 ]);
        let test: Vector3 = vadd!(vector_space, &x, &y, &z);

        let exp: Vector3 = Vector3([ 13, 8, 13]);
        assert_eq!(test, exp);
    }


    /// Benchmarked: 35,946 ns/iter for addition.
    /// Bencharked: 52,145 ns/iter for multiplication.
    mod benchmarks
    {
        use test::Bencher;

        use super::*;

        const BENCH_ADDITION_TEST_SIZE: usize = 10000;
        const BENCH_ADDITION_TEST_CONST: isize = 123456789;

        struct BenchmarkedDummyVectorSpace;

        struct BenchmarkedVector3([isize; BENCH_ADDITION_TEST_SIZE]);

        impl<'a> IntoIterator for &'a BenchmarkedVector3
        {
            type Item = &'a isize;
            type IntoIter = std::slice::Iter<'a, isize>;

            fn into_iter(self) -> Self::IntoIter {
                self.0.iter()
            }
        }

        impl<'a> FromIterator<isize> for BenchmarkedVector3
        {
            fn from_iter<T>(iter: T) -> Self 
            where
                T: IntoIterator<Item = isize>
            {
                let mut buf: _ = [0; BENCH_ADDITION_TEST_SIZE];
                for (idx, item) in iter
                    .into_iter()
                    .enumerate() 
                {
                    assert!(idx < BENCH_ADDITION_TEST_SIZE);
                    buf[idx] = item;
                }
                BenchmarkedVector3(buf)
            }
        }

        impl VectorSpace for BenchmarkedDummyVectorSpace 
        {
            type Scalar = isize;

            type Vector = BenchmarkedVector3;

            fn dimensions(&self) -> usize 
            {
                BENCH_ADDITION_TEST_SIZE
            }
        }

        #[bench]
        fn bench_addition(bench: &mut Bencher) 
        {
            bench.iter(|| {
                let vector_space = BenchmarkedDummyVectorSpace;
            
                let x: _ = BenchmarkedVector3([ BENCH_ADDITION_TEST_CONST; BENCH_ADDITION_TEST_SIZE ]);
                let y: _ = BenchmarkedVector3([ BENCH_ADDITION_TEST_CONST; BENCH_ADDITION_TEST_SIZE ]);
                let z: _ = BenchmarkedVector3([ BENCH_ADDITION_TEST_CONST; BENCH_ADDITION_TEST_SIZE ]);
                vadd!(vector_space, &x, &y, &z)
            });
        }

        #[bench]
        fn bench_multiplication(bench: &mut Bencher) 
        {
            bench.iter(|| {
                let vector_space = BenchmarkedDummyVectorSpace;
            
                let x: _ = BenchmarkedVector3([ BENCH_ADDITION_TEST_CONST; BENCH_ADDITION_TEST_SIZE ]);
                let y: _ = BenchmarkedVector3([ BENCH_ADDITION_TEST_CONST; BENCH_ADDITION_TEST_SIZE ]);
                let z: _ = BenchmarkedVector3([ BENCH_ADDITION_TEST_CONST; BENCH_ADDITION_TEST_SIZE ]);
                let x_out: _ = vector_space.vscale(&BENCH_ADDITION_TEST_CONST, &x);
                let y_out: _ = vector_space.vscale(&BENCH_ADDITION_TEST_CONST, &y);
                let z_out: _ = vector_space.vscale(&BENCH_ADDITION_TEST_CONST, &z);
                (x_out, y_out, z_out)
            });
        }

    }

}
