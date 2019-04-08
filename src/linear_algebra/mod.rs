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
 * Structs are used for specialisation purposes.
 * The empty traits are for specialisation purpose.
 * They should all relate to CoreMatrix.
 * Some matrices have unique functions that will be stored in the relevant trait.
 * The Matrix struct is capable of every function call.
 * I've commented out Tridiagonal for the moment until the hermitian is implemented.
 * Currently working on the Hessenberg form.
 * IMPORTANT: requires a method for interfacing between structs: mystruct.cast_into::<TYPE>()
****************************/ 