#[macro_export]
macro_rules! UniOps {
    (@VAdditiveInverse $name:ident, $space:ident) => {
        impl<T> VAdditiveInverse for $space<T>
        where
            T: Copy,
            for <'a> &'a T: Neg<Output=T>
        {
            type Vector = $name<T>;

            fn additive_inv(&self, vector: &Self::Vector) -> Self::Vector
            {
                let mut buf: Self::Vector = vector.clone();
                self.additive_inv_mut(&mut buf);
                buf
            }
        }
    };

    (@VAdditiveInverseMut $name:ident, $space:ident) => {
        impl<T> VAdditiveInverseMut for $space<T>
        where
            T: Copy,
            for <'a> &'a T: Neg<Output=T>
        {
            type Vector = $name<T>;

            fn additive_inv_mut(&self, vector: &mut Self::Vector)
            {
                vector
                    .0
                    .iter_mut()
                    .for_each(|val| *val = (*val).neg() );
            }
        }
    };
}
