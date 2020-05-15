#[allow(unused_macros)]

#[macro_export]
macro_rules! ndvector {

    ($length:tt) => {
        paste::item! {
            pub use [< _ vector $length >]::{[< Vector $length >], [< VectorSpace $length >]};
            
            mod [< _ vector $length >] 
            {
                use std::{marker, ops};
                use marker::PhantomData;
                use ops::Add;

                use super::*;

                #[derive(Copy, Clone)]
                pub struct [< Vector $length >]<T>([T; $length]);  

                impl<T> [< Vector $length >]<T>
                {
                    pub fn new(inner: [T; $length]) -> Self 
                    {
                        [< Vector $length >](inner)
                    }
                }

                impl<'a,T> IntoIterator for &'a [< Vector $length >]<T>
                {
                    type Item = &'a T;
                    type IntoIter = std::slice::Iter<'a,T>;

                    fn into_iter(self) -> Self::IntoIter
                    {
                        self.0.iter()
                    }
                }


                pub struct [< VectorSpace $length >]<T> {
                    _phantom: PhantomData<T>
                }

                impl<T> [< VectorSpace $length >]<T>
                {
                    pub fn new() -> Self 
                    {
                        [< VectorSpace $length >] {
                            _phantom: PhantomData
                        }
                    }
                }

                impl<T> VectorSpace for [< VectorSpace $length >]<T>
                {
                    type Scalar = T;

                    type Vector = [< VectorSpace $length >]<T>;

                    fn dimensions(&self) -> usize 
                    {
                        $length
                    }
                }

                impl<T> VAdd for [< VectorSpace $length >]<T>
                where
                    T: Copy + Default,
                    for <'a> &'a T: Add<&'a T, Output=T>
                {
                    type Vector = [< Vector $length >]<T>;

                    fn vadd(&self, lhs: &Self::Vector, rhs: &Self::Vector) -> Self::Vector
                    {
                        let mut buf: [T; $length] = [T::default() ; $length];
                        buf
                            .iter_mut()
                            .zip( lhs.into_iter().zip( rhs.into_iter() ) )
                            .for_each(|(out, (l, r))| *out = l + r );
                        Self::Vector::new(buf)
                    }
                }

                impl<T> VPartialEq for [< VectorSpace $length >]<T>
                where
                    T: PartialEq
                {
                    type Vector = [< Vector $length >]<T>;

                    fn eq(lhs: Self::Vector, rhs: Self::Vector) -> bool
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
            }
        }
    };

} 

use crate::ndvector;
use crate::vector::*;

ndvector!(1024);
