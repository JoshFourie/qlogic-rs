use criterion::{criterion_group, criterion_main, Criterion};

mod vector;

criterion_main!(
    vector::internal_vector_benches
);
