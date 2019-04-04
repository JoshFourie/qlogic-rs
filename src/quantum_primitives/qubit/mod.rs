pub mod quantum_bit;
pub use quantum_bit::Qubit as Qubit;

pub trait QuantumBit<T>
{
    type Inner;
    type Error;

    fn into_inner(self) -> Self::Inner;
}