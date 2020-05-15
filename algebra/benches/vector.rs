use criterion::{criterion_group, criterion_main, Criterion};

use algebra::vector::*;
use algebra::{ndvector, vadd};

pub const BENCH_ADDITION_TEST_SIZE: usize = 1024;

ndvector!(1024);

type Vector = Vector1024<isize>;
type Space = VectorSpace1024<isize>;

fn random() -> Vector {
    use rand::{thread_rng, Rng};

    let mut inner: _ = [0; BENCH_ADDITION_TEST_SIZE];
    for element in inner.iter_mut() {
        *element = thread_rng().gen()
    }
    Vector::new(inner)
}

// 294ns/iter
fn bench_addition(bench: &mut Criterion) 
{
    let vector_space = Space::new();

    bench.bench_function("Vector Addition", |c| {
        let mut x: Vector = random();
        let y: Vector = random();
    
        c.iter(|| {
            vector_space.vadd( &mut x, &y )
        })
    });
}

fn bench_multiplication(bench: &mut Criterion) 
{
    let vector_space = Space::new();
    let scalar: isize = 123456789;

    bench.bench_function("Q-Logic Vector Multiplication", |c| {
        let mut x: Vector = random();

        c.iter(|| {
            vector_space.vscale(&mut x, &scalar);
        })
    });
}

fn bench_addition_against_nalgebra(bench: &mut Criterion)
{
    let mut group: _ = bench.benchmark_group("Nalgebra Vector Addition Group");

    // Qlogic: 3.7130 us for 10 000
    {
        let vector_space = Space::new();

        group.bench_function("Q-Logic Vector Addition", |c| {
            let mut x: Vector = random();
            let y: Vector = random();

            c.iter(|| {
                vector_space.vadd(&mut x, &y);
            })
        });
    }

    // Nalgebra: 4.7408 us for 10 000
    {
        group.bench_function("Nalgebra Vector Addition", |c| {
            let mut x: nalgebra::DVector<isize> = nalgebra::DVector::new_random(BENCH_ADDITION_TEST_SIZE);
            let y: nalgebra::DVector<isize> = nalgebra::DVector::new_random(BENCH_ADDITION_TEST_SIZE);
    
            c.iter(|| {
                x += &y
            })
        });
    }
}

fn bench_multiplication_against_nalgebra(bench: &mut Criterion)
{
    let mut group: _ = bench.benchmark_group("Nalgebra Vector Multiplication Group");
    let scalar: isize = 123456789;

    // Qlogic: 3.7130 us for 10 000
    {
        let vector_space = Space::new();

        group.bench_function("Q-Logic Vector Multiplication", |c| {
            let mut x: Vector = random();

            c.iter(|| {
                vector_space.vscale(&mut x, &scalar);
            })
        });
    }

    // Nalgebra: 4.7408 us for 10 000
    {
        group.bench_function("Nalgebra Vector Multiplication", |c| {
            let mut x: nalgebra::DVector<isize> = nalgebra::DVector::new_random(BENCH_ADDITION_TEST_SIZE);
    
            c.iter(|| {
                x *= scalar
            })
        });
    }
}

fn bench_vadd_macro(bench: &mut Criterion)
{
    let mut group: _ = bench.benchmark_group("Vector Addition Macro");

    // Macro: 2.3369 us - ~1us to clone 'x'
    {
        let vector_space = Space::new();

        group.bench_function("Macro", |c| {
            let y: Vector = random();
            let z: Vector = random();
            let a: Vector = random();
            let b: Vector = random();

            let x: _ = random();
            c.iter(|| {
                vadd!(vector_space, x.clone(), &y, &z, &a, &b)
            })
        });
    }

    // Hand-Coded: 1.2746 us
    {
        let vector_space = Space::new();

        group.bench_function("Hand-Coded", |c| {
            let y: Vector = random();
            let z: Vector = random();
            let a: Vector = random();
            let b: Vector = random();

            let mut x: Vector = random();
            c.iter(|| {
                vector_space.vadd(&mut x, &y);
                vector_space.vadd(&mut x, &z);
                vector_space.vadd(&mut x, &a);
                vector_space.vadd(&mut x, &b);
            })
        });
    }
}

criterion_group!(
    vector_benches, 
    bench_multiplication,
    bench_addition,
    bench_vadd_macro,
    bench_addition_against_nalgebra,
    bench_multiplication_against_nalgebra
);

criterion_main!(vector_benches);
