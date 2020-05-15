#[cfg(not(feature="manual"))] mod generic;
#[cfg(not(feature="manual"))] pub use generic::*;

mod std_array;
pub use std_array::*;
