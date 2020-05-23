mod identities;
pub use identities::*;

pub trait Field<T>
{
    
}

impl<T,U> Field<T> for U
where
    U: AdditiveIdent<Output=T>
{
    
}
