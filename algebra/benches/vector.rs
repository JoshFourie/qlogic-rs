use criterion::{criterion_group, criterion_main, Criterion};

use algebra::vector::*;
use algebra::{ndarray, vadd};

pub const BENCH_ADDITION_TEST_SIZE: usize = 1024;

pub mod small
{
    use super::*;

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

    fn bench_multiplication(bench: &mut Criterion) 
    {
        let mut group: _ = bench.benchmark_group("internal-vector-multiplication-small-group");

        {
            let vector_space = Space::new();

            group.bench_function("stdvec-reference", |c| {
                let x: Vector<isize> = random();
                let y: isize = 125;

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

    criterion_group!(
        small_vector_benchmarks,
        bench_addition_mutable,
        bench_addition_reference,
        bench_multiplication
    );
}

pub mod big 
{
    use super::*;

    ndarray!{
        @vector_space(Space) {
            @vector_ident(Vector)
            @length(9999)
            @generic(T)
            @with_vec(Vec<T>)        
        }
    }

    fn random() -> Vector<i8> {
        use rand::{thread_rng, Rng};
    
        let mut inner: _ = vec![0; 9999];
        for element in inner.iter_mut() {
            *element = thread_rng().gen()
        }
        Vector::new(inner)
    }
        
    ndarray!{
        @vector_space(ArraySpace) {
            @vector_ident(ArrayVector)
            @length(9999)
            @generic(T)
            @with_array([T; 9999])        
        }
    }

    fn random_array() -> ArrayVector<i8> {
        use rand::{thread_rng, Rng};
    
        let mut inner: _ = [0; 9999];
        for element in inner.iter_mut() {
            *element = thread_rng().gen()
        }
        ArrayVector::new(inner)
    }


    fn bench_addition_reference(bench: &mut Criterion) 
    {
        let mut group: _ = bench.benchmark_group("internal-vector-addition-reference-big-group");
        group.sample_size(10);

        {
            let vector_space = Space::new();

            group.bench_function("stdvec-reference", |c| {
                let x: Vector<i8> = random();
                let y: Vector<i8> = random();

                c.iter(|| {
                    vector_space.vadd(&x, &y)
                })
            });
        }

        {
            let vector_space = ArraySpace::new();

            group.bench_function("array-reference", |c| {
                let x: ArrayVector<i8> = random_array();
                let y: ArrayVector<i8> = random_array();

                c.iter(|| {
                    vector_space.vadd(&x, &y)
                })
            });
        }
    }

    fn bench_addition_mutable(bench: &mut Criterion) 
    {
        let mut group: _ = bench.benchmark_group("internal-vector-addition-mutable-big-group");
        group.sample_size(10);

        {
            let vector_space = Space::new();

            group.bench_function("stdvec-mutable", |c| {
                let mut x: Vector<i8> = random();
                let y: Vector<i8> = random();

                c.iter(|| {
                    vector_space.vadd_mut(&mut x, &y);
                })
            });
        }

        {
            let vector_space = ArraySpace::new();

            group.bench_function("array-mutable", |c| {
                let mut x: ArrayVector<i8> = random_array();
                let y: ArrayVector<i8> = random_array();

                c.iter(|| {
                    vector_space.vadd_mut(&mut x, &y);
                })
            });
        }
    }

    fn bench_multiplication(bench: &mut Criterion) 
    {
        let mut group: _ = bench.benchmark_group("internal-vector-multiplication-big-group");
        group.sample_size(10);

        {
            // let vector_space = Space::new();

            // group.bench_function("VecReference", |c| {
            //     let x: Vector<i8> = random();
            //     let y: i8 = 125;

            //     c.iter(|| {
            //         vector_space.vscale(&x, &y);
            //     })
            // });
        }

        {
            // let vector_space = ArraySpace::new();

            // group.bench_function("ArrayReference", |c| {
            //     let x: ArrayVector<i8> = random_array();
            //     let y: ArrayVector<i8> = random_array();

            //     c.iter(|| {
            //         vector_space.vadd(&x, &y);
            //     })
            // });
        }

        {
            let vector_space = Space::new();

            group.bench_function("stdvec-mutable", |c| {
                let mut x: Vector<i8> = random();
                let y: i8 = 125;

                c.iter(|| {
                    vector_space.vscale(&mut x, &y);
                })
            });
        }

        {
            let vector_space = ArraySpace::new();

            group.bench_function("array-mutable", |c| {
                let mut x: ArrayVector<i8> = random_array();
                let y: i8 = 125;

                c.iter(|| {
                    vector_space.vscale(&mut x, &y);
                })
            });
        }
    }

    criterion_group!(
        big_vector_benchmarks,
        bench_addition_mutable,
        bench_addition_reference,
        bench_multiplication
    );
}

// fn bench_addition_mut_against_nalgebra(bench: &mut Criterion)
// {
//     let mut group: _ = bench.benchmark_group("NalgebraVectorAdditionMutableGroup");

//     // Qlogic: 3.7130 us for 10 000
//     {
//         let vector_space = Space::new();

//         group.bench_function("Q-Logic Vector Addition", |c| {
//             let mut x: Vector<isize> = random();
//             let y: Vector<isize> = random();

//             c.iter(|| {
//                 vector_space.vadd_mut(&mut x, &y);
//             })
//         });
//     }

//     // Nalgebra: 4.7408 us for 10 000
//     {
//         group.bench_function("Nalgebra Vector Addition", |c| {
//             let mut x: nalgebra::DVector<isize> = nalgebra::DVector::new_random(BENCH_ADDITION_TEST_SIZE);
//             let y: nalgebra::DVector<isize> = nalgebra::DVector::new_random(BENCH_ADDITION_TEST_SIZE);
    
//             c.iter(|| {
//                 x += &y
//             })
//         });
//     }
// }

// fn bench_addition_against_nalgebra(bench: &mut Criterion)
// {
//     let mut group: _ = bench.benchmark_group("NalgebraVectorAdditionReferenceGroup");

//     // Qlogic: 3.7130 us for 10 000
//     {
//         let vector_space = Space::new();

//         group.bench_function("Q-Logic Vector Addition", |c| {
//             let x: Vector<isize> = random();
//             let y: Vector<isize> = random();

//             c.iter(|| {
//                 vector_space.vadd(&x, &y);
//             })
//         });
//     }

//     // Nalgebra: 4.7408 us for 10 000
//     {
//         group.bench_function("Nalgebra Vector Addition", |c| {
//             let x: nalgebra::DVector<isize> = nalgebra::DVector::new_random(BENCH_ADDITION_TEST_SIZE);
//             let y: nalgebra::DVector<isize> = nalgebra::DVector::new_random(BENCH_ADDITION_TEST_SIZE);
    
//             c.iter(|| {
//                 &x + &y
//             })
//         });
//     }
// }

// fn bench_multiplication_against_nalgebra(bench: &mut Criterion)
// {
//     let mut group: _ = bench.benchmark_group("NalgebraVectorMultiplicationGroup");
//     let scalar: isize = 125;

//     // Qlogic: 3.7130 us for 10 000
//     {
//         let vector_space = Space::new();

//         group.bench_function("Q-Logic Vector Multiplication", |c| {
//             let mut x: Vector<isize> = random();

//             c.iter(|| {
//                 vector_space.vscale(&mut x, &scalar);
//             })
//         });
//     }

//     // Nalgebra: 4.7408 us for 10 000
//     {
//         group.bench_function("Nalgebra Vector Multiplication", |c| {
//             let mut x: nalgebra::DVector<isize> = nalgebra::DVector::new_random(BENCH_ADDITION_TEST_SIZE);
    
//             c.iter(|| {
//                 x *= scalar
//             })
//         });
//     }
// }

// fn bench_additive_inverse_against_nalgebra(bench: &mut Criterion)
// {
//     let mut group: _ = bench.benchmark_group("NalgebraVectorAdditiveInverseGroup");

//     // Qlogic: 3.7130 us for 10 000
//     {
//         let vector_space = Space::new();

//         group.bench_function("Q-Logic Vector Additive Inverse", |c| {
//             let mut x: Vector<isize> = random();

//             c.iter(|| {
//                 vector_space.additive_inv(&mut x);
//             })
//         });
//     }

//     // Nalgebra: 4.7408 us for 10 000
//     {
//         group.bench_function("Nalgebra Vector Additive Inverse", |c| {
//             let ref x: nalgebra::DVector<isize> = nalgebra::DVector::new_random(BENCH_ADDITION_TEST_SIZE);
    
//             c.iter(|| {
//                 -x
//             })
//         });
//     }
// }
