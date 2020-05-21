#[macro_export]
macro_rules! BinOps {
    (@VAdd $name:ident, $space:ident) => {
        impl<T> VAdd for $space<T>
        where
            for <'a> T: Copy + AddAssign<&'a T>,
        {
            type Vector = $name<T>;
            
            fn vadd(&self, lhs: &Self::Vector, rhs: &Self::Vector) -> Self::Vector
            {
                let mut buf: Self::Vector = lhs.clone();
                self.vadd_mut(&mut buf, rhs);
                buf
            }
        }
    };

    (@VAddMut $name:ident, $space:ident) => {
        impl<T> VAddMut for $space<T>
        where
            for <'a> T: Copy + AddAssign<&'a T>,
        {
            type Vector = $name<T>;
            
            fn vadd_mut(&self, lhs: &mut Self::Vector, rhs: &Self::Vector)
            {
                lhs
                    .0
                    .iter_mut()
                    .zip(rhs)
                    .for_each(|(l,r)| l.add_assign(r));
            }
        }
    };

    (@VScale $name:ident, $space:ident) => {
        impl<T> VScale for $space<T>
        where
            for <'a> T: Copy + MulAssign<&'a T>,
        {
            type Vector = $name<T>;

            type Scalar = T;

            fn vscale(&self, vector: &Self::Vector, scalar: &Self::Scalar) -> Self::Vector
            {
                let mut buf: Self::Vector = vector.clone();
                self.vscale_mut(&mut buf, scalar);
                buf
            }
        }
    };

    (@VScaleMut $name:ident, $space:ident) => {
        impl<T> VScaleMut for $space<T>
        where
            for <'a> T: Copy + MulAssign<&'a T>,
        {
            type Vector = $name<T>;

            type Scalar = T;

            fn vscale_mut(&self, vector: &mut Self::Vector, scalar: &Self::Scalar)
            {
                vector
                    .0
                    .iter_mut()
                    .for_each(|val| val.mul_assign(scalar));
            }
        }
    };
}
