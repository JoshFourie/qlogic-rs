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

                $(
                    ndarray!(@vector $length, $name, $array, $generic);
                    ndarray!(@vectorspace $length, $name, $space, $array, $generic);

                    ndarray!(@with_array $length, $name, $space, $array, $generic);
                )?

                $(
                    ndarray!(@vector $length, $name, $vector, $generic);
                    ndarray!(@vectorspace $length, $name, $space, $vector, $generic);

                    ndarray!(@with_vec $length, $name, $space, $vector, $generic);
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

    /********************* VAdd *******************************/
    (@with_array $length:expr, $name:ident, $space:ident, $inner:ty, $T:ident) => {
        impl<$T> VAdd for $space<$T>
        where
            for <'a> $T: Copy + AddAssign<&'a $T>,
            for <'a> &'a $T: Add<&'a $T, Output=$T>
        {
            type Vector = $name<$T>;
            
            fn vadd_mut(&self, lhs: &mut Self::Vector, rhs: &Self::Vector)
            {
                lhs
                    .0
                    .iter_mut()
                    .zip(rhs)
                    .for_each(|(l,r)| l.add_assign(r));
            }

            fn vadd(&self, lhs: &Self::Vector, rhs: &Self::Vector) -> Self::Vector
            {
                let mut buf: Self::Vector = lhs.clone();
                self.vadd_mut(&mut buf, rhs);
                buf
            }
        }

        impl<$T> VScale for $space<$T>
        where
            for <'a> $T: Copy + MulAssign<&'a $T>,
            for <'a> &'a $T: Mul<&'a $T, Output=$T>
        {
            type Vector = $name<$T>;

            type Scalar = $T;

            fn vscale_mut(&self, vector: &mut Self::Vector, scalar: &Self::Scalar)
            {
                vector
                    .0
                    .iter_mut()
                    .for_each(|val| val.mul_assign(scalar));

            }

            fn vscale(&self, vector: &Self::Vector, scalar: &Self::Scalar) -> Self::Vector
            {
                let mut buf: Self::Vector = vector.clone();
                self.vscale_mut(&mut buf, scalar);
                buf
            }
        }

        impl<$T> VAdditiveInverse for $space<$T>
        where
            $T: Copy,
            for <'a> &'a $T: Neg<Output=$T>
        {
            type Vector = $name<$T>;

            fn additive_inv_mut(&self, vector: &mut Self::Vector)
            {
                vector
                    .0
                    .iter_mut()
                    .for_each(|val| *val = (*val).neg() );
            }

            fn additive_inv(&self, vector: &Self::Vector) -> Self::Vector
            {
                let mut buf: Self::Vector = vector.clone();
                self.additive_inv_mut(&mut buf);
                buf
            }
        }
    };

    (@with_vec $length:expr, $name:ident, $space:ident, $inner:ty, $T:ident) => {
        impl<$T> VAdd for $space<$T>
        where
            for <'a> $T: Copy + AddAssign<&'a $T>,
            for <'a> &'a $T: Add<&'a $T,Output=T>
        {
            type Vector = $name<$T>;

            fn vadd_mut(&self, lhs: &mut Self::Vector, rhs: &Self::Vector)
            {
                lhs
                    .0
                    .iter_mut()
                    .zip(rhs)
                    .for_each(|(l, r)| l.add_assign(r));
            }

            fn vadd(&self, lhs: &Self::Vector, rhs: &Self::Vector) -> Self::Vector
            {
                lhs
                    .into_iter()
                    .zip(rhs)
                    .map(|(l,r)| l + r)
                    .collect()
            }
        }


        impl<$T> VScale for $space<$T>
        where
            for <'a> $T: Copy + MulAssign<&'a $T>,
            for <'a> &'a $T: Mul<&'a $T, Output=$T>
        {
            type Vector = $name<$T>;

            type Scalar = $T;

            fn vscale_mut(&self, vector: &mut Self::Vector, scalar: &Self::Scalar)
            {
                vector
                    .0
                    .iter_mut()
                    .for_each(|val| val.mul_assign(scalar));
            }

            fn vscale(&self, vector: &Self::Vector, scalar: &Self::Scalar) -> Self::Vector
            {
                vector
                    .into_iter()
                    .map(|val| val * scalar)
                    .collect()
            }
        }

        impl<$T> VAdditiveInverse for $space<$T>
        where
            $T: Copy,
            for <'a> &'a $T: Neg<Output=$T>
        {
            type Vector = $name<$T>;

            fn additive_inv_mut(&self, vector: &mut Self::Vector)
            {
                vector
                    .0
                    .iter_mut()
                    .for_each(|val| *val = (*val).neg() );
            }

            fn additive_inv(&self, vector: &Self::Vector) -> Self::Vector
            {
                vector
                    .into_iter()
                    .map(|val| -val)
                    .collect()
            }
        }
    };
} 
