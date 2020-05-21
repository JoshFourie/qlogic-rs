mod array;
mod vec;

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
                
            #[allow(unused_imports)]
            mod [< _ vector $name With $space $length >] 
            {
                use std::{marker, fmt, ops, iter};
                use iter::FromIterator;
                use marker::PhantomData;
                use fmt::Debug;
                use ops::{AddAssign, Add, MulAssign, Mul, Index, IndexMut, Neg};

                use super::{VAdd, VScale, VectorSpace, VPartialEq, VAdditiveInverse, ndarray};

                use vector::{binops, uniops};

                $(
                    ndarray!(@vector $length, $name, $array, $generic);
                    ndarray!(@vectorspace $length, $name, $space, $array, $generic);
                    ndarray!(@array $length, $name, $space, $array, $generic);
                )?

                $(
                    ndarray!(@vector $length, $name, $vector, $generic);
                    ndarray!(@vectorspace $length, $name, $space, $vector, $generic);
                    ndarray!(@vec $length, $name, $space, $vector, $generic);
                )?
            }
        }
    };

    (@vector $length:expr, $name:ident, $inner:ty, $T:ident) => {
        #[derive(Clone)]
        pub struct $name<$T>($inner);  

        impl<$T> $name<$T>
        {
            pub fn new(inner: $inner) -> Self 
            {   
                assert!(inner.len() == $length);
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

        impl<$T> FromIterator<$T> for $name<$T>
        where
            $inner: FromIterator<$T>
        {
            fn from_iter<I>(iterator: I) -> Self
            where
                I: IntoIterator<Item=$T>
            {
                let buf: $inner = iterator
                    .into_iter()
                    .collect();
                Self::new(buf)
            }
        }

        impl<'a,$T> FromIterator<&'a $T> for $name<$T>
        where
            $inner: FromIterator<&'a $T>
        {
            fn from_iter<I>(iterator: I) -> Self
            where
                I: IntoIterator<Item=&'a $T>
            {
                let buf: $inner = iterator
                    .into_iter()
                    .collect();
                Self::new(buf)
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

        impl<$T> Debug for $name<$T>
        where
            $inner: Debug
        {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
            {
                write!(f, "{:?}", self.0)
            }
        }

    };

    (@vectorspace $length:expr, $name:ident, $space:ident, $inner:ty, $T:ident) => {
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
        binops!(@addition $length, $name, $space, $inner, $T);
        binops!(@scale $length, $name, $space, $inner, $T);
        uniops!(@additive_inverse $length, $name, $space, $inner, $T);
    };

    (@vec $length:expr, $name:ident, $space:ident, $inner:ty, $T:ident) => {
        binops!(@addition $length, $name, $space, $inner, $T);
        binops!(@scale $length, $name, $space, $inner, $T);
        uniops!(@additive_inverse $length, $name, $space, $inner, $T);
    };
} 
