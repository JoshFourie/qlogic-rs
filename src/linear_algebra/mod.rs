#![allow(non_snake_case)]

/***** Declarations ********/
pub mod basic_matrices;
pub mod specialised_matrices;
pub mod ancillary_algorithms;

/***** Pub. Exports ********/
pub use basic_matrices::*;
pub use specialised_matrices::*;
pub use ancillary_algorithms::*;

/******** Dev Notes ********
 * I need an interface for vector/matrix multiplications, but type parameters 
 * blow-out quickly. Might be better to leverage the None val to specify a vector...
 * This would require the traits to be rewritten, but it could be offloaded to a special
 * vector struct that is invoked whenever there is a None in the row line...
 * 
 * Structs are used for specialisation purposes.
 * The empty traits are for specialisation purpose.
 * They should all relate to CoreMatrix.
 * Some matrices have unique functions that will be stored in the relevant trait.
 * The Matrix struct is capable of every function call.
 * I've commented out Tridiagonal for the moment until the hermitian is implemented.
 * Currently working on the Hessenberg form.
 * Not sure that the memory management is efficient with the &self implementation, but a struct could easily
 * specialise and do a swap in place instead.
 * IMPORTANT: requires a method for interfacing between structs: mystruct.cast_into::<TYPE>()
****************************/ 