# Ventifact's Linear Algebra
A repository for the series on writing a Linear Algebra library available here: https://medium.com/ventifacts-linear-algebra.

This repository has been archived. It shows that we can get faster performance on some things in Rust by using macros. I think that this result is fairly predictable: by using macros to define a vector-space, we get to use fixed-size arrays. This is useful because vector that change dimensions (via flattening of a matrix, for example) often do that change predictably. 
