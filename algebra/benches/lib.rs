use criterion::{criterion_group, criterion_main, Criterion};

mod vector;

criterion_main!(
    vector::small::small_vector_benchmarks,
    vector::big::big_vector_benchmarks
);
