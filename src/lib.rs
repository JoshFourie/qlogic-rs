#![feature(specialization)]

pub mod math_primitives;
pub mod qubit;
pub mod ops;

use math_primitives as math;

#[cfg(test)] mod test;