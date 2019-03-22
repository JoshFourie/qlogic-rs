#![feature(specialization)]

pub mod math_primitives;
pub mod quantum_primitives;

pub use math_primitives as math;
pub use quantum_primitives as qu_prim;

#[cfg(test)] mod test;