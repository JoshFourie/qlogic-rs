#[macro_export]
macro_rules! BlasOps {
    (@VAXPY $name:ident, $space:ident) => {
        impl<T> VAXPY for $space<T>
        where
            T: Copy + AddAssign<T>,
            for <'a> &'a T: Mul<&'a T, Output=T>,
        {
            type Vector = $name<T>;

            type Scalar = T;

            fn vaxpy(&self, alpha: &Self::Scalar, x: &Self::Vector, y: &Self::Vector) -> Self::Vector
            {
                let mut output: Self::Vector = y.clone();
                self.vaxpy_mut(alpha, x, &mut output);
                output
            }
        }
    };

    (@VAXPYMut $name:ident, $space:ident) => {
        impl<T> VAXPYMut for $space<T>
        where
            T: AddAssign<T>,
            for <'a> &'a T: Mul<&'a T, Output=T>,
        {
            type Vector = $name<T>;

            type Scalar = T;

            fn vaxpy_mut(&self, a: &Self::Scalar, x: &Self::Vector, y: &mut Self::Vector)
            {
                for (l,r) in y
                    .0
                    .iter_mut()
                    .zip(x)
                {
                    l.add_assign( r * a )
                }
            }
        }
    };

    (@DotVMut $name:ident, $space:ident) => {
        impl<T> DotVMut for $space<T>
        where
            T: AddAssign<T>,
            for <'a> &'a T: Mul<&'a T,Output=T>
        {
            type Vector = $name<T>;

            type Scalar = T;
        
            fn dotv_mut(&self, x: &Self::Vector, y: &Self::Vector, output: &mut Self::Scalar)
            {
                x
                    .into_iter()
                    .zip(y) 
                    .for_each(|(xi,yi)| output.add_assign(xi * yi));
            }
        }
    };

    (@DotV $name:ident, $space:ident) => {
        impl<T> DotV for $space<T>
        where
            T: Add<T,Output=T> + AdditiveIdentity<Output=T>,
            for <'a> &'a T: Mul<&'a T,Output=T>,
        {
            type Vector = $name<T>;

            type Scalar = T;
        
            fn dotv(&self, x: &Self::Vector, y: &Self::Vector) -> Self::Scalar
            {
                let acc: Self::Scalar = T::additive_identity();
                x
                    .into_iter()
                    .zip(y)
                    .map(|(xi,yi)| xi * yi)
                    .fold(acc, |acc, val| acc + val)          
            }
        }
    };
}
