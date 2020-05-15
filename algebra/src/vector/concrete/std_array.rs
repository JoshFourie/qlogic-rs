#[allow(unused_macros)]

#[macro_export]
macro_rules! ndvector {
    ($length:tt) => {
        paste::item! {
            pub use [< _ vector $length >]::[< Vector $length >];
            
            mod [< _ vector $length >] 
            {
                use std::{iter, mem};
                use iter::FromIterator;

                pub struct [< Vector $length >]<T> ([T; $length]);  

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
                    type IntoIter = std::slice::Iter<'a, T>;

                    fn into_iter(self) -> Self::IntoIter {
                        self.0.iter()
                    }
                }

                impl<'a, T: Copy> FromIterator<T> for [< Vector $length >]<T>
                where
                    T: num_traits::Zero
                {
                    fn from_iter<I>(iter: I) -> Self 
                    where
                        I: IntoIterator<Item=T>
                    {
                        let mut buf: [T; $length] = [T::zero(); $length];
                        for (idx, item) in iter
                            .into_iter()
                            .enumerate() 
                        {
                            assert!(idx < $length);
                            buf[idx] = item;
                        }
                        [< Vector $length >](buf)
                        
                    }
                }
            }
        }
    }
} 
