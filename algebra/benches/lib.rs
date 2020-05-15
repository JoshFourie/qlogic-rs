use criterion::{criterion_group, criterion_main, Criterion};

#[cfg(not(feature="manual"))] 
mod vector;

#[cfg(not(feature="manual"))]
criterion_main!(
    vector::vector_benches
);

fn dummy(_bench: &mut Criterion) {  }

criterion_group!(temporary, dummy);

#[cfg(feature="manual")]
criterion_main!(
    temporary
);
