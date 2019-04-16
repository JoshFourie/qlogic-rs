#![feature(specialization, try_trait, associated_type_defaults)]

pub mod linear_algebra;
pub mod quantum_primitives;

pub use linear_algebra as math;
pub use quantum_primitives as qu_prim;

pub use math::*;

#[cfg(test)] mod test;

/***** Commit Log: 16-04-19 
 * .update() call is somehow introducing a rounding error...
 * 
 * diverting from quantum components to expand linear algebra crate.
 * eigenvalues are incorrect with an unacceptable round-off error in the std QR algo.
 * currently implementing implicit shift QR algo. for eigenvalues.
 * encounterd signum error on balancing, might be an issue with test param. but unlikely.
 * the subsequent subroutines should write over the problematic column, so we may be able to continue. 
 
 * i would like to rewrite the lib. with macro impl. for v2.0. 
 * the trait system should be better organised and collated under super-traits.
***************************/