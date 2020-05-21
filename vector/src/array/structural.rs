#[macro_export]
macro_rules! vector_base {
    ($length:expr, $name:ident, $inner:ty, $T:ident) => {
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
} 
