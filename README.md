# qlogic-rs
Quantum Logic for Personal Interest.

# Introduction

The purpose is to implement a simple simulation of a Quantum Key Distribution protocol, as well as some extras.

The project uses a nested-crates structure:
- `./algebra` contains the linear algebra for the simulation.

`Docs` are a work in progress on the project and module levels. Currently the auto-generated documentation for the linear algebra crate can either be built by the Rust Cargo compiler or navigating to `./target/doc/algebra/index.html` after cloning the repo. There are no online copies available.

The majority of the linear algebra code is based on psuedo-code from the Princeton Handbook of Linear Algebra and scattered university lecture slides freely available online. 

# Linear Algebra
The library needed to support basic linear algebra operations, which have been implemented in the './algebra' sub-module. Currently, the following operations relevant to Quantum mechanics are implemented:
- Vectors:
    - Addition
    - Multiplication: Scalar, Dot (Inner), Direct (Outer).
    - Euclidean Norm.
- Matrix:
    - Eigenvalues. 
    - Tensor (Kronecker).
    - Trace.
    - Tranposition

Some bonus features that have been implemented out of interest or as ancillary to the above are:
- Matrix:
    - Gaussian Elimination
    - Minor
    - LU Decomposition
    - Givens Transformation
    - Householder Reflector
    - QR Decomposition
    - Elementary Row Operations

There are some remaining components that need to be implemented:
- Vectors:
    - Gram-Scmidt procedure for finding the orthonormal basis set.
- Matrix:
    - Eigenvectors.
    - Inverse.
    - Singular Value Decomposition

Then there are some others that should be improved because they are currently lacking:
- Matrix:
    - Eigenvalues: numerically unstable and should be using the Francis Double Step instead of the Householder Trasformation.
    - Balance: currently a broken implementation.
