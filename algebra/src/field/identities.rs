pub trait AdditiveIdentity
{
    type Output;

    fn additive_identity() -> Self::Output;
}

#[macro_use]
macro_rules! primitives {
    (
        $($primitive:ty),*
    ) => {
        $(
            impl AdditiveIdentity for $primitive
            {
                type Output = $primitive;

                fn additive_identity() -> Self::Output
                {
                    0
                }
            }   
            
            impl<'a> AdditiveIdentity for &'a $primitive
            {
                type Output = $primitive;

                fn additive_identity() -> Self::Output
                {
                    0
                }
            } 

            impl<'a> AdditiveIdentity for &'a mut $primitive
            {
                type Output = $primitive;

                fn additive_identity() -> Self::Output
                {
                    0
                }
            } 
        )*
    };

    (
        @float $($primitive:ty),*
    ) => {
        $(
            impl AdditiveIdentity for $primitive
            {
                type Output = $primitive;

                fn additive_identity() -> Self::Output
                {
                    0.0
                }
            }   
            
            impl<'a> AdditiveIdentity for &'a $primitive
            {
                type Output = $primitive;

                fn additive_identity() -> Self::Output
                {
                    0.0
                }
            } 

            impl<'a> AdditiveIdentity for &'a mut $primitive
            {
                type Output = $primitive;

                fn additive_identity() -> Self::Output
                {
                    0.0
                }
            } 
        )*
    };
}

primitives!{
    u8, u16, u32, u64, u128, usize,
    i8, i16, i32, i64, i128, isize
}

primitives!{@float f32, f64}
