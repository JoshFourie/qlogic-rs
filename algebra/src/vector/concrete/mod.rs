#[allow(unused_macros)]

#[macro_export]
macro_rules! ndarray {
    (
        @vector_space($space:ident) {
            @vector_ident($name:ident)
            @length($length:expr)
            @generic($generic:ident)
            $(@with_array($array:ty))?
            $(@with_vec($vector:ty))?
        }
    ) => {
        paste::item! {
            pub use [< _ vector $name With $space $length >]::{$name, $space};
                
            mod [< _ vector $name With $space $length >] 
            {
                use std::{marker, ops};
                use marker::PhantomData;
                use ops::{AddAssign, MulAssign, Index, IndexMut, Neg};

                use super::{VAdd, VScale, VectorSpace, VPartialEq, VAdditiveInverse, ndarray};

                $(
                    ndarray!(@implementation $length, $name, $space, $array, $generic);
                    ndarray!(@array $length, $name, $space, $array, $generic);
                )?

                $(
                    ndarray!(@implementation $length, $name, $space, $vector, $generic);
                    ndarray!(@vec $length, $name, $space, $vector, $generic);
                )?
            }
        }
    };

    (@implementation $length:expr, $name:ident, $space:ident, $inner:ty, $T:ident) => {
        #[derive(Clone)]
        pub struct $name<$T>($inner);  

        impl<$T> $name<$T>
        {
            pub fn new(inner: $inner) -> Self 
            {
                $name(inner)
            }
        }

        impl<$T> From<$inner> for $name<$T>
        {
            fn from(inner: $inner) -> Self {
                Self::new( inner )
            }
        }

        impl<'a,$T> IntoIterator for &'a $name<$T>
        {
            type Item = &'a $T;
            type IntoIter = std::slice::Iter<'a,$T>;

            fn into_iter(self) -> Self::IntoIter
            {
                self.0.iter()
            }
        }

        impl<$T> Index<usize> for $name<$T>
        {
            type Output = $T;

            fn index(&self, idx: usize) -> &Self::Output 
            {
                &self.0[idx]
            }
        }

        impl<$T> IndexMut<usize> for $name<$T>
        {
            fn index_mut(&mut self, idx: usize) -> &mut Self::Output 
            {
                &mut self.0[idx]
            }
        }

        pub struct $space<$T> {
            _phantom: PhantomData<$T>
        }

        impl<$T> $space<$T>
        {
            #[inline]
            pub fn new() -> Self 
            {
                $space {
                    _phantom: PhantomData
                }
            }
        }

        impl<$T> VectorSpace for $space<$T>
        {
            type Scalar = $T;

            type Vector = $space<$T>;

            fn dimensions(&self) -> usize 
            {
                $length
            }
        }

        impl<$T> VScale for $space<$T>
        where
            $T: Copy + MulAssign<$T>
        {
            type Vector = $name<$T>;

            type Scalar = $T;

            fn vscale(&self, vector: &mut Self::Vector, scalar: &Self::Scalar)
            {
                for idx in 0..$length {
                    unsafe { 
                        vector.0.get_unchecked_mut(idx).mul_assign( scalar.clone() ) 
                    }
                }
            }
        }

        impl<$T> VAdditiveInverse for $space<$T>
        where
            for <'a> &'a $T: Neg<Output=$T>
        {
            type Vector = $name<$T>;

            fn additive_inv(&self, vector: &mut Self::Vector)
            {
                for idx in 0..$length {
                    unsafe { 
                        let val: &$T = vector.0.get_unchecked(idx);
                        *vector.0.get_unchecked_mut(idx) = -val; 
                    }
                }
            }
        }

        impl<$T> VPartialEq for $space<$T>
        where
            $T: PartialEq
        {
            type Vector = $name<$T>;

            fn eq(&self, lhs: &Self::Vector, rhs: &Self::Vector) -> bool
            {
                for (l, r) in lhs
                    .into_iter()
                    .zip( rhs.into_iter() ) 
                {
                    if (l != r) {
                        return false
                    }
                }
                return true
            }
        }
    };

    (@array $length:expr, $name:ident, $space:ident, $inner:ty, $T:ident) => {
        impl<$T> VAdd for $space<$T>
        where
            $T: Copy + AddAssign<$T>
        {
            type Vector = $name<$T>;

            fn vadd_mut(&self, lhs: &mut Self::Vector, rhs: &Self::Vector)
            {
                for idx in 0..$length {
                    unsafe { 
                        lhs.0.get_unchecked_mut(idx).add_assign( rhs.0.get_unchecked(idx).clone() ) 
                    }
                }
            }

            fn vadd(&self, lhs: &Self::Vector, rhs: &Self::Vector) -> Self::Vector
            {
                unimplemented!()
            }
        }
    };

    (@vec $length:expr, $name:ident, $space:ident, $inner:ty, $T:ident) => {
        impl<$T> VAdd for $space<$T>
        where
            $T: Copy + AddAssign<$T>
        {
            type Vector = $name<$T>;

            fn vadd_mut(&self, lhs: &mut Self::Vector, rhs: &Self::Vector)
            {
                for idx in 0..$length {
                    unsafe { 
                        lhs.0.get_unchecked_mut(idx).add_assign( rhs.0.get_unchecked(idx).clone() ) 
                    }
                }
            }

            fn vadd(&self, lhs: &Self::Vector, rhs: &Self::Vector) -> Self::Vector
            {
                unimplemented!()
            }
        }
    };
} 
