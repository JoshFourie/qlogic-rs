/**** Exports ******/

pub mod operator;

pub use operator::Operator;

/**** Interface ******/

pub trait Operative<T,Q>: std::ops::Mul<T,Output=Self>
    + std::ops::Mul<Self,Output=Self>
    + std::ops::Mul<Q,Output=Q>
    + std::marker::Sized
{
    fn tensor(self, rhs: Self) -> Self;
}