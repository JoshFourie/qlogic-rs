use criterion::{criterion_group, criterion_main, Criterion};

use algebra::vector::*;
use algebra::ndvector;

pub const BENCH_ADDITION_TEST_SIZE: usize = 1024;
pub const BENCH_ADDITION_TEST_CONST: isize = 123456789;

ndvector!(1024);

impl Vector1024<isize>
{
    pub fn random() -> Self {
        use rand::{thread_rng, Rng};

        let mut inner: _ = [BENCH_ADDITION_TEST_CONST; BENCH_ADDITION_TEST_SIZE];
        thread_rng().fill(&mut inner);        
        Vector1024::new(inner)
    }
}


fn bench_addition(bench: &mut Criterion) 
{
    let vector_space = VectorSpace1024::new();
    let x: _ = Vector1024::new([ BENCH_ADDITION_TEST_CONST; BENCH_ADDITION_TEST_SIZE ]);
    let y: _ = Vector1024::new([ BENCH_ADDITION_TEST_CONST; BENCH_ADDITION_TEST_SIZE ]);

    bench.bench_function("Vector Addition", |c| {
        c.iter(|| {
            vector_space.vadd( &x, &y )
        })
    });
}

// fn bench_multiplication(bench: &mut Criterion) 
// {
//     let vector_space = VectorSpaceImpl;
//     let x: _ = Vector1024::new([ BENCH_ADDITION_TEST_CONST; BENCH_ADDITION_TEST_SIZE ]);

//     bench.bench_function("Vector Multiplication", |c| {
//         c.iter(|| {
//             vector_space.vscale(&BENCH_ADDITION_TEST_CONST, &x)
//         })
//     });
// }

// fn bench_addition_against_nalgebra(bench: &mut Criterion)
// {
//     let mut group: _ = bench.benchmark_group("Nalgebra Vector Addition Group");

//     // Qlogic
//     {
//         let vector_space = VectorSpaceImpl;

//         let x: Vector1024<isize> = Vector1024::random();
//         let y: Vector1024<isize> = Vector1024::random();

//         group.bench_function("Q-Logic Vector Addition", |c| {
//             c.iter(|| {
//                 vector_space.vadd(&x, &y)
//             })
//         });
//     }

//     // Nalgebra
//     {
//         use rand::SeedableRng;
//         let x: nalgebra::DVector<isize> = nalgebra::DVector::new_random(BENCH_ADDITION_TEST_SIZE);
//         let y: nalgebra::DVector<isize> = nalgebra::DVector::new_random(BENCH_ADDITION_TEST_SIZE);

//         group.bench_function("Nalgebra Vector Addition", |c| {
//             c.iter(|| {
//                 &x + &y
//             })
//         });
//     }
// }

criterion_group!(
    vector_benches, 
    // bench_multiplication,
    bench_addition,
    // bench_addition_against_nalgebra
);

criterion_main!(vector_benches);
