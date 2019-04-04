#![feature(specialization, try_trait, associated_type_defaults)]

pub mod linear_algebra;
pub mod quantum_primitives;

pub use linear_algebra as math;
pub use quantum_primitives as qu_prim;

pub use math::*;

#[cfg(test)] mod test;