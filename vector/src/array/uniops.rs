#[macro_export]
macro_rules! uniops {
    (@additive_inverse $length:expr, $name:ident, $space:ident, $inner:ty, $T:ident) => {
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
}
