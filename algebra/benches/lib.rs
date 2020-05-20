use criterion::{criterion_group, criterion_main, Criterion};

mod vector;

criterion_main!(
    vector::smallvec::smallvec,
    vector::medvec::medvec,
    vector::bigvec::bigvec
);
