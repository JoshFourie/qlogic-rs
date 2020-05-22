use criterion::criterion_main;

mod comparative;

criterion_main!(
    comparative::cmp_nalgebra::nalgebra_smallvec,
    comparative::cmp_nalgebra::nalgebra_medvec,
    comparative::cmp_nalgebra::nalgebra_bigvec
);
