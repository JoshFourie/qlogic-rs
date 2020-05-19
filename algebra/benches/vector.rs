use criterion::{criterion_group, criterion_main, Criterion};

use algebra::vector::*;
use algebra::{ndarray, vadd};

pub const BENCH_ADDITION_TEST_SIZE: usize = 1024;


ndarray!{
    @vector_space(Space) {
        @vector_ident(Vector)
        @length(1024)
        @generic(T)
        @with_vec(Vec<T>)        
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

fn bench_addition_reference(bench: &mut Criterion) 
{
    let mut group: _ = bench.benchmark_group("internal-vector-addition-reference-small-group");

    {
        let vector_space = Space::new();

        group.bench_function("stdvec-reference", |c| {
            let x: Vector<isize> = random();
            let y: Vector<isize> = random();

            c.iter(|| {
                vector_space.vadd(&x, &y)
            })
        });
    }

    {
        let vector_space = ArraySpace::new();

        group.bench_function("array-reference", |c| {
            let x: ArrayVector<isize> = random_array();
            let y: ArrayVector<isize> = random_array();

            c.iter(|| {
                vector_space.vadd(&x, &y)
            })
        });
    }
}

fn bench_addition_mutable(bench: &mut Criterion) 
{
    let mut group: _ = bench.benchmark_group("internal-vector-addition-mutable-small-group");

    {
        let vector_space = Space::new();

        group.bench_function("stdvec-mutable", |c| {
            let mut x: Vector<isize> = random();
            let y: Vector<isize> = random();

            c.iter(|| {
                vector_space.vadd_mut(&mut x, &y);
            })
        });
    }

    {
        let vector_space = ArraySpace::new();

        group.bench_function("array-mutable", |c| {
            let mut x: ArrayVector<isize> = random_array();
            let y: ArrayVector<isize> = random_array();

            c.iter(|| {
                vector_space.vadd_mut(&mut x, &y);
            })
        });
    }
}

fn bench_multiplication_reference(bench: &mut Criterion) 
{
    let mut group: _ = bench.benchmark_group("internal-vector-multiplication-reference-small-group");

    {
        let vector_space = Space::new();

        group.bench_function("stdvec-reference", |c| {
            let x: Vector<isize> = random();
            let y: Vector<isize> = random();

            c.iter(|| {
                vector_space.vscale(&x, &y)
            })
        });
    }

    {
        let vector_space = ArraySpace::new();

        group.bench_function("array-reference", |c| {
            let x: ArrayVector<isize> = random_array();
            let y: isize = 125;

            c.iter(|| {
                vector_space.vscale(&x, &y)
            })
        });
    }
}

fn bench_multiplication_mutable(bench: &mut Criterion) 
{
    let mut group: _ = bench.benchmark_group("internal-vector-multiplication-mutable-small-group");

    {
        let vector_space = Space::new();

        group.bench_function("stdvec-mutable", |c| {
            let mut x: Vector<isize> = random();
            let y: isize = 125;

            c.iter(|| {
                vector_space.vscale_mut(&mut x, &y);
            })
        });
    }

    {
        let vector_space = ArraySpace::new();

        group.bench_function("array-mutable", |c| {
            let mut x: ArrayVector<isize> = random_array();
            let y: isize = 125;

            c.iter(|| {
                vector_space.vscale_mut(&mut x, &y);
            })
        });
    }
}

fn bench_addition_mut_against_nalgebra(bench: &mut Criterion)
{
    let mut group: _ = bench.benchmark_group("nalgebra-vector-additional-mutable-group");

    // Qlogic: 3.7130 us for 10 000
    {
        let vector_space = Space::new();

        group.bench_function("internal-stdvec-addition", |c| {
            let mut x: Vector<isize> = random();
            let y: Vector<isize> = random();

            c.iter(|| {
                vector_space.vadd_mut(&mut x, &y);
            })
        });
    }

    {
        let vector_space = ArraySpace::new();

        group.bench_function("internal-array-addition", |c| {
            let mut x: ArrayVector<isize> = random_array();
            let y: ArrayVector<isize> = random_array();

            c.iter(|| {
                vector_space.vadd_mut(&mut x, &y);
            })
        });
    }

    // Nalgebra: 4.7408 us for 10 000
    {
        group.bench_function("nalgebra-vector-addition", |c| {
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
    let mut group: _ = bench.benchmark_group("nalgebra-vector-addition-reference-group");

    // Qlogic: 3.7130 us for 10 000
    {
        let vector_space = Space::new();

        group.bench_function("internal-stdvec-addition", |c| {
            let x: Vector<isize> = random();
            let y: Vector<isize> = random();

            c.iter(|| {
                vector_space.vadd(&x, &y)
            })
        });
    }

    {
        let vector_space = ArraySpace::new();

        group.bench_function("internal-array-addition", |c| {
            let x: ArrayVector<isize> = random_array();
            let y: ArrayVector<isize> = random_array();

            c.iter(|| {
                vector_space.vadd(&x, &y)
            })
        });
    }

    // Nalgebra: 4.7408 us for 10 000
    {
        group.bench_function("nalgebra-vector-addition", |c| {
            let x: nalgebra::DVector<isize> = nalgebra::DVector::new_random(BENCH_ADDITION_TEST_SIZE);
            let y: nalgebra::DVector<isize> = nalgebra::DVector::new_random(BENCH_ADDITION_TEST_SIZE);
    
            c.iter(|| {
                &x + &y
            })
        });
    }
}

fn bench_multiplication_mut_against_nalgebra(bench: &mut Criterion)
{
    let mut group: _ = bench.benchmark_group("nalgebra-vector-multiplication-mutable-group");
    let scalar: isize = 125;

    // Qlogic: 3.7130 us for 10 000
    {
        let vector_space = Space::new();

        group.bench_function("internal-stdvec-multiplication-mutable", |c| {
            let mut x: Vector<isize> = random();

            c.iter(|| {
                vector_space.vscale_mut(&mut x, &scalar)
            })
        });
    }

    {
        let vector_space = ArraySpace::new();

        group.bench_function("internal-array-multiplication-mutable", |c| {
            let mut x: ArrayVector<isize> = random_array();

            c.iter(|| {
                vector_space.vscale_mut(&mut x, &scalar)
            })
        });
    }

    // Nalgebra: 4.7408 us for 10 000
    {
        group.bench_function("nalgebra-vector-multiplication-mutable", |c| {
            let mut x: nalgebra::DVector<isize> = nalgebra::DVector::new_random(BENCH_ADDITION_TEST_SIZE);
    
            c.iter(|| {
                x *= scalar
            })
        });
    }
}

fn bench_multiplication_against_nalgebra(bench: &mut Criterion)
{
    let mut group: _ = bench.benchmark_group("nalgebra-vector-multiplication-reference-group");
    let scalar: isize = 125;

    // Qlogic: 3.7130 us for 10 000
    {
        let vector_space = Space::new();

        group.bench_function("internal-stdvec-multiplication-reference", |c| {
            let x: Vector<isize> = random();

            c.iter(|| {
                vector_space.vscale(&x, &scalar)
            })
        });
    }

    {
        let vector_space = ArraySpace::new();

        group.bench_function("internal-array-multiplication-reference", |c| {
            let x: ArrayVector<isize> = random_array();

            c.iter(|| {
                vector_space.vscale(&x, &scalar)
            })
        });
    }

    // Nalgebra: 4.7408 us for 10 000
    {
        group.bench_function("nalgebra-vector-multiplication-reference", |c| {
            let mut x: nalgebra::DVector<isize> = nalgebra::DVector::new_random(BENCH_ADDITION_TEST_SIZE);
    
            c.iter(|| {
                x *= scalar
            })
        });
    }
}

fn bench_additive_inverse_against_nalgebra(bench: &mut Criterion)
{
    let mut group: _ = bench.benchmark_group("nalgebra-vector-additive-inverse-group");

    // Qlogic: 3.7130 us for 10 000
    {
        let vector_space = Space::new();

        group.bench_function("internal-stdvec-vector-additive-inverse-mut", |c| {
            let mut x: Vector<isize> = random();

            c.iter(|| {
                vector_space.additive_inv(&mut x);
            })
        });
    }

    {
        let vector_space = ArraySpace::new();

        group.bench_function("internal-array-vector-additive-inverse-mut", |c| {
            let mut x: ArrayVector<isize> = random_array();

            c.iter(|| {
                vector_space.additive_inv(&mut x);
            })
        });
    }

    // Nalgebra: 4.7408 us for 10 000
    {
        group.bench_function("nalgebra-vector-additive-inverse-reference", |c| {
            let ref x: nalgebra::DVector<isize> = nalgebra::DVector::new_random(BENCH_ADDITION_TEST_SIZE);
    
            c.iter(|| {
                -x
            })
        });
    }
}

criterion_group!(
    vector_benchmarks,
    bench_addition_mutable,
    bench_addition_reference,
    bench_multiplication_mutable,
    bench_multiplication_reference,
    
    bench_addition_against_nalgebra,
    bench_addition_mut_against_nalgebra,
    bench_multiplication_against_nalgebra,
    bench_multiplication_mut_against_nalgebra,
    bench_additive_inverse_against_nalgebra,
);
