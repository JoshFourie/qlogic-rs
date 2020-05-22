pub trait AdditiveIdent
{
    type Output;

    fn additive_ident() -> Self::Output;
}

#[macro_use]
macro_rules! primitives {
    (
        $($primitive:ty),*
    ) => {
        $(
            impl AdditiveIdent for $primitive
            {
                type Output = $primitive;

                fn additive_ident() -> Self::Output
                {
                    0
                }
            }   
            
            impl<'a> AdditiveIdent for &'a $primitive
            {
                type Output = $primitive;

                fn additive_ident() -> Self::Output
                {
                    0
                }
            } 

            impl<'a> AdditiveIdent for &'a mut $primitive
            {
                type Output = $primitive;

                fn additive_ident() -> Self::Output
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
            impl AdditiveIdent for $primitive
            {
                type Output = $primitive;

                fn additive_ident() -> Self::Output
                {
                    0.0
                }
            }   
            
            impl<'a> AdditiveIdent for &'a $primitive
            {
                type Output = $primitive;

                fn additive_ident() -> Self::Output
                {
                    0.0
                }
            } 

            impl<'a> AdditiveIdent for &'a mut $primitive
            {
                type Output = $primitive;

                fn additive_ident() -> Self::Output
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
