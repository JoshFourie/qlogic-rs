use criterion::{criterion_group, criterion_main, Criterion};

use algebra::vector::*;
use algebra::ndvector;

const BENCH_ADDITION_TEST_SIZE: usize = 100;
const BENCH_ADDITION_TEST_CONST: isize = 1;

struct VectorSpaceImpl;

ndvector!(100);

impl VectorSpace for VectorSpaceImpl 
{
    type Scalar = isize;

    type Vector = Vector100<isize>;

    fn dimensions(&self) -> usize 
    {
        BENCH_ADDITION_TEST_SIZE
    }
}

#[cfg(not(feature="manual"))] 
fn bench_addition(bench: &mut Criterion) 
{
    bench.bench_function("Vector Addition", |c| {
        c.iter(|| {
            let vector_space = VectorSpaceImpl;
        
            let x: _ = Vector100::new([ BENCH_ADDITION_TEST_CONST; BENCH_ADDITION_TEST_SIZE ]);
            let y: _ = Vector100::new([ BENCH_ADDITION_TEST_CONST; BENCH_ADDITION_TEST_SIZE ]);
            vector_space.vadd(&x, &y)
        })
    });
}

#[cfg(not(feature="manual"))] 
fn bench_multiplication(bench: &mut Criterion) 
{
    bench.bench_function("Vector Multiplication", |c| {
        c.iter(|| {
            let vector_space = VectorSpaceImpl;
        
            let x: _ = Vector100::new([ BENCH_ADDITION_TEST_CONST; BENCH_ADDITION_TEST_SIZE ]);
            let x_out: _ = vector_space.vscale(&BENCH_ADDITION_TEST_CONST, &x);
            x_out
        })
    });
}

#[cfg(not(feature="manual"))] 
criterion_group!(
    vector_benches, 
    bench_multiplication,
    bench_addition
);

#[cfg(not(feature="manual"))] 
criterion_main!(vector_benches);
