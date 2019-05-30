macro_rules! impl_into_iter {
    ($id:ty) => {
        impl<'a, T: Copy> IntoIterator for $id {
            
            type Item = T;
            type IntoIter = std::vec::IntoIter<T>;

            fn into_iter(self) -> Self::IntoIter
            {
                self.inner.into_iter()
            }
        }
    }
}