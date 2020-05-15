// use std::{ops, iter};
// use ops::{Add, Mul, Neg};
// use iter::FromIterator;

// use crate::vector::*;

// // impl<U> VAdd for U
// // where
// //     U: VectorSpace,
// //     U::Vector: FromIterator<U::Scalar>,
// //     for <'a> &'a U::Vector: IntoIterator<Item=&'a U::Scalar>,
// //     for <'a> &'a U::Scalar: Add<&'a U::Scalar, Output=U::Scalar>,
// // {
// //     type Vector = U::Vector;

// //     default fn vadd(&self, lhs: &Self::Vector, rhs: &Self::Vector) -> Self::Vector
// //     {
// //         lhs
// //             .into_iter()
// //             .zip( rhs.into_iter() )
// //             .map(|(l,r)| l + r)
// //             .collect()
// //     }
// // }

// impl<U> VScale for U
// where
//     U: VectorSpace,
//     U::Vector: FromIterator<U::Scalar>,
//     for <'a> &'a U::Vector: IntoIterator<Item=&'a U::Scalar>,
//     for <'a> &'a U::Scalar: Mul<&'a U::Scalar, Output=U::Scalar>,
// {
//     type Scalar = U::Scalar;

//     type Vector = U::Vector;

//     default fn vscale(&self, scalar: &Self::Scalar, vector: &Self::Vector) -> Self::Vector        
//     {
//         vector
//             .into_iter()
//             .map(|val| scalar * val)
//             .collect()
//     }
// }

// impl<U> VIdentity for U
// where
//     U: VMultiplicativeIdent + VAdditiveIdent
// {
//     // Empty.
// }

// impl<U> VAdditiveIdent for U
// where
//     U: VectorSpace,
//     U::Scalar: num_traits::Zero,
//     U::Vector: FromIterator<U::Scalar>
// {
//     type Output = U::Vector;
    
//     default fn additive_ident(&self) -> Self::Output 
//     {
//         use num_traits::Zero;

//         ( 0..self.dimensions() )
//             .into_iter()
//             .map(|_| U::Scalar::zero() )
//             .collect()
//     }
// }

// impl<U> VMultiplicativeIdent for U
// where
//     U: VectorSpace,
//     U::Scalar: num_traits::One
// {
//     type Output = U::Scalar;
    
//     default fn mul_ident(&self) -> Self::Output 
//     {
//         use num_traits::One;
//         U::Scalar::one()
//     }
// }

// impl<U> VAdditiveInverse for U
// where
//     U: VectorSpace,
//     U::Vector: FromIterator<U::Scalar>,
//     for <'a> &'a U::Vector: IntoIterator<Item=&'a U::Scalar>,
//     for <'a> &'a U::Scalar: Neg<Output=U::Scalar>,
// {
//     type Vector = U::Vector;

//     type Output = U::Vector;

//     default fn additive_inv(&self, vector: &Self::Vector) -> Self::Output 
//     {
//         vector
//             .into_iter()
//             .map(|val| -val)
//             .collect()
//     }
// }
