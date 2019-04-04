// pub mod bra_ket;
pub mod linear_ops;

pub trait QuantumUnit: num::Num
    + std::ops::Neg<Output=Self>
    + std::ops::AddAssign
    + std::fmt::Debug
    + Copy
{
    // these are not implemented as a trait in Rust.
    fn pow64(self, rhs: f64) -> Self;
    fn sqroot(self) -> Self;
}

// trait for scalar units that have traits like PartialOrd<Self> & Signum
// the QuantumUnit is only useful for allowing specialisation.
pub trait QuantumReal: QuantumUnit 
    + num_traits::real::Real
{

}