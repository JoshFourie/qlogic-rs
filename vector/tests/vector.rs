macro_rules! test {
    ($name:ident, $object:ty, $space:ty) => {
        mod $name {
            use super::*;

            #[test]
            fn test_addition_mut() 
            {
                let vector_space = <$space>::new();
                let mut x = <$object>::from([ 3, 0, -1 ]);
                let y = <$object>::from([ 10, 1, 2 ]);

                let exp: $object = <$object>::from([ 13, 1, 1 ]);
                let test: $object = vadd!(vector_space, x, &y);

                assert!( vector_space.eq(&exp, &test) );
            }

            #[test]
            fn test_multiplication_mut()
            {
                let vector_space = <$space>::new();
                let mut x = <$object>::from([ 3, 0, -1 ]);
                let c = 2;

                let exp = <$object>::from([ 6, 0, -2 ]);
                let test = vscale!(vector_space, x, &c);
                assert!( vector_space.eq(&exp, &test), "Expected: {:?}, Got: {:?}", &exp, &test );
            }

            #[test]
            fn test_commutative_mut()
            {
                let vector_space = <$space>::new();
                let mut x1 = <$object>::from([ 3, 1, 5 ]);
                let x2: $object = x1.clone();
                let mut y = <$object>::from([ 6, 2, 7 ]);

                let lhs = vadd!(vector_space, x1, &y);
                let rhs = vadd!(vector_space, y, &x2);
                assert!( vector_space.eq(&lhs, &rhs) );
            }

            #[test]
            fn test_associative_addition_mut()
            {
                let vector_space = <$space>::new();
                let mut x1: $object = <$object>::from([ 3, 1, 5 ]);
                let x2: $object = x1.clone();
                let mut y: $object = <$object>::from([ 6, 2, 7 ]);
                let z = <$object>::from([ 4, 5, 1 ]);

                let lhs: $object = vadd!(vector_space, x1, &y, &z);
                let rhs: $object = vadd!(vector_space, y, &z, &x2);
                assert!( vector_space.eq(&lhs, &rhs) );
            }

            #[test]
            fn test_additive_ident_mut()
            {
                let vector_space = <$space>::new();
                let exp: $object = <$object>::from([ 0, 0, 0 ]);

                let test: $object = vector_space.additive_ident();
                assert!( vector_space.eq(&exp, &test) );
            }

            #[test]
            fn test_additive_inverse_mut()
            {
                let vector_space = <$space>::new();
                let mut x: $object = <$object>::from([ 3, 1, 5 ]);
                let exp: $object = <$object>::from([ -3, -1, -5 ]);
                    
                vector_space.additive_inv_mut(&mut x);
                assert!( vector_space.eq(&exp, &x) );
            }

            #[test]
            fn test_vadd() 
            {
                let vector_space = <$space>::new();
                
                let mut x: $object = <$object>::from([ 3, 1, 5 ]);
                let y: $object = <$object>::from([ 6, 2, 7 ]);
                let z: $object = <$object>::from([ 4, 5, 1 ]);
                let test: $object = vadd!(vector_space, x, &y, &z);

                let exp: $object = <$object>::from([ 13, 8, 13]);
                assert!( vector_space.eq(&test, &exp) );
            }   
        }
    };
}

use vector::ndarray;

use algebra::vector::*;
use algebra::{vadd, vscale};

ndarray! {
    @vector_space(VectorSpaceArray) {
        @vector_ident(VectorArray)
        @length(3)
        @generic(U)
        @with([U; 3])
    }
}

test!(test_ndarray, VectorArray<isize>, VectorSpaceArray<isize>);

impl VAdditiveIdent for VectorSpaceArray<isize>
{
    type Output = VectorArray<isize>;

    fn additive_ident(&self) -> Self::Output
    {
        VectorArray::new( [0; 3] )
    }
}

impl VMultiplicativeIdent for VectorSpaceArray<isize>
{
    type Output = isize;

    fn mul_ident(&self) -> Self::Output
    {
        1
    }
}


ndarray! {
    @vector_space(VectorSpaceVec) {
        @vector_ident(VectorVec)
        @length(3)
        @generic(T)
        @with(Vec<T>)
    }
}   

test!(test_ndvec, VectorVec<isize>, VectorSpaceVec<isize>);

impl From<[isize; 3]> for VectorVec<isize>
{
    fn from(array: [isize; 3]) -> Self {
        Self::from( array.to_vec() )
    }
}

impl VAdditiveIdent for VectorSpaceVec<isize>
{
    type Output = VectorVec<isize>;

    fn additive_ident(&self) -> Self::Output
    {
        VectorVec::new( vec![0; 3] )
    }
}

impl VMultiplicativeIdent for VectorSpaceVec<isize>
{
    type Output = isize;

    fn mul_ident(&self) -> Self::Output
    {
        1
    }
}