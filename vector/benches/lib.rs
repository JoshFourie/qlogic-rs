use criterion::criterion_main;

mod comparative;

criterion_main!(
    comparative::cmp_nalgebra::nalgebra_smallvec,
    comparative::cmp_nalgebra::nalgebra_medvec,
    comparative::cmp_nalgebra::nalgebra_bigvec,

    comparative::cmp_ndarray::ndarray_smallvec,
    comparative::cmp_ndarray::ndarray_medvec,
    comparative::cmp_ndarray::ndarray_bigvec,
);
