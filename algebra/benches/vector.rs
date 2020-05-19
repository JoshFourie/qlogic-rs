use criterion::{criterion_group, criterion_main, Criterion};

use algebra::vector::*;
use algebra::{ndarray, vadd};

pub const BENCH_ADDITION_TEST_SIZE: usize = 1024;

ndarray!{
    @vector_space(Space) {
        @vector_ident(Vector)
        @length(1024)
        @generic(T)
        @with_array(Vec<T>)        
    }
}

ndarray!{
    @vector_space(ArraySpace) {
        @vector_ident(ArrayVector)
        @length(1024)
        @generic(T)
        @with_array([T; 1024])        
    }
}

fn random_array() -> ArrayVector<isize> {
    use rand::{thread_rng, Rng};

    let mut inner: _ = [0; BENCH_ADDITION_TEST_SIZE];
    for element in inner.iter_mut() {
        *element = thread_rng().gen()
    }
    ArrayVector::new(inner)
}

fn random() -> Vector<isize> {
    use rand::{thread_rng, Rng};

    let mut inner: _ = vec![0; BENCH_ADDITION_TEST_SIZE];
    for element in inner.iter_mut() {
        *element = thread_rng().gen()
    }
    Vector::new(inner)
}

// 294ns/iter
fn bench_addition_mut(bench: &mut Criterion) 
{
    let mut group: _ = bench.benchmark_group("Internal Vector Mutable Addition Group");

    {
        let vector_space = Space::new();

        group.bench_function("Vec", |c| {
            let mut x: Vector<isize> = random();
            let y: Vector<isize> = random();

            c.iter(|| {
                vector_space.vadd_mut(&mut x, &y);
            })
        });
    }

    {
        let vector_space = ArraySpace::new();

        group.bench_function("Array", |c| {
            let mut x: ArrayVector<isize> = random_array();
            let y: ArrayVector<isize> = random_array();

            c.iter(|| {
                vector_space.vadd_mut(&mut x, &y);
            })
        });
    }
}

fn bench_addition(bench: &mut Criterion) 
{
    let mut group: _ = bench.benchmark_group("Internal Vector Reference Addition Group");

    {
        let vector_space = Space::new();

        group.bench_function("Vec", |c| {
            let x: Vector<isize> = random();
            let y: Vector<isize> = random();

            c.iter(|| {
                vector_space.vadd(&x, &y);
            })
        });
    }

    {
        let vector_space = ArraySpace::new();

        group.bench_function("Array", |c| {
            let x: ArrayVector<isize> = random_array();
            let y: ArrayVector<isize> = random_array();

            c.iter(|| {
                vector_space.vadd(&x, &y);
            })
        });
    }
}

fn bench_multiplication(bench: &mut Criterion) 
{
    let vector_space = Space::new();
    let scalar: isize = 123456789;

    bench.bench_function("Vector Multiplication", |c| {
        let mut x: Vector<isize> = random();

        c.iter(|| {
            vector_space.vscale(&mut x, &scalar);
        })
    });
}

fn bench_addition_mut_against_nalgebra(bench: &mut Criterion)
{
    let mut group: _ = bench.benchmark_group("Nalgebra Vector Addition Mutable Group");

    // Qlogic: 3.7130 us for 10 000
    {
        let vector_space = Space::new();

        group.bench_function("Q-Logic Vector Addition", |c| {
            let mut x: Vector<isize> = random();
            let y: Vector<isize> = random();

            c.iter(|| {
                vector_space.vadd_mut(&mut x, &y);
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

fn bench_addition_against_nalgebra(bench: &mut Criterion)
{
    let mut group: _ = bench.benchmark_group("Nalgebra Vector Addition Reference Group");

    // Qlogic: 3.7130 us for 10 000
    {
        let vector_space = Space::new();

        group.bench_function("Q-Logic Vector Addition", |c| {
            let x: Vector<isize> = random();
            let y: Vector<isize> = random();

            c.iter(|| {
                vector_space.vadd(&x, &y);
            })
        });
    }

    // Nalgebra: 4.7408 us for 10 000
    {
        group.bench_function("Nalgebra Vector Addition", |c| {
            let x: nalgebra::DVector<isize> = nalgebra::DVector::new_random(BENCH_ADDITION_TEST_SIZE);
            let y: nalgebra::DVector<isize> = nalgebra::DVector::new_random(BENCH_ADDITION_TEST_SIZE);
    
            c.iter(|| {
                &x + &y
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
            let mut x: Vector<isize> = random();

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

fn bench_additive_inverse_against_nalgebra(bench: &mut Criterion)
{
    let mut group: _ = bench.benchmark_group("Nalgebra Vector Additive Inverse Group");

    // Qlogic: 3.7130 us for 10 000
    {
        let vector_space = Space::new();

        group.bench_function("Q-Logic Vector Additive Inverse", |c| {
            let mut x: Vector<isize> = random();

            c.iter(|| {
                vector_space.additive_inv(&mut x);
            })
        });
    }

    // Nalgebra: 4.7408 us for 10 000
    {
        group.bench_function("Nalgebra Vector Additive Inverse", |c| {
            let ref x: nalgebra::DVector<isize> = nalgebra::DVector::new_random(BENCH_ADDITION_TEST_SIZE);
    
            c.iter(|| {
                -x
            })
        });
    }
}

criterion_group!(
    vector_benches, 
    bench_multiplication,
    bench_addition,
    bench_addition_mut,
    bench_addition_against_nalgebra,
    bench_addition_mut_against_nalgebra,
    bench_multiplication_against_nalgebra,
    bench_additive_inverse_against_nalgebra
);

criterion_main!(vector_benches);
