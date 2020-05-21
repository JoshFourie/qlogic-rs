#[macro_use]
macro_rules! binops {
    (@addition $length:expr, $name:ident, $space:ident, $inner:ty, $T:ident) => {
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
    };
}