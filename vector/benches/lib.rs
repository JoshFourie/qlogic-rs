use criterion::criterion_main;

mod vector;
mod comparative;

criterion_main!(
    vector::smallvec::smallvec,
    vector::medvec::medvec,
    vector::bigvec::bigvec,

    comparative::cmp_nalgebra::nalgebra_smallvec,
    comparative::cmp_nalgebra::nalgebra_medvec,
    comparative::cmp_nalgebra::nalgebra_bigvec
);
