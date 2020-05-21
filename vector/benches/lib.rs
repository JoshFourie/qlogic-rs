use criterion::criterion_main;

mod vector;

criterion_main!(
    vector::smallvec::smallvec,
    vector::medvec::medvec,
    vector::bigvec::bigvec
);
