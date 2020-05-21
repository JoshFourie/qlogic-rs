macro_rules! bench_vectors {
    ($(($uid:ident, $vec_length:expr, $array_length:expr));+;) => {
        $(
            pub mod $uid 
            {
                use criterion::{criterion_group, Criterion};

                use algebra::vector::*;
                use vector::ndarray;

                const LENGTH: usize = $vec_length;

                ndarray! {
                    Space {
                        vector: Vector,
                        dimension: $vec_length
                    }
                }
                
                ndarray! {
                    ArraySpace {
                        vector: ArrayVector,
                        dimension: $array_length,
                        using: [T; $array_length]
                    }
                }
                
                
                fn random_array() -> ArrayVector<isize> {
                    use rand::{thread_rng, Rng};
                
                    let mut inner: _ = [0; $array_length];
                    for element in inner.iter_mut() {
                        *element = thread_rng().gen()
                    }
                    ArrayVector::new(inner)
                }
                
                fn random() -> Vector<isize> {
                    use rand::{thread_rng, Rng};
                
                    let mut inner: _ = vec![0; $vec_length];
                    for element in inner.iter_mut() {
                        *element = thread_rng().gen()
                    }
                    Vector::new(inner)
                }
                
                fn bench_addition_reference(bench: &mut Criterion) 
                {
                    let mut group: _ = bench.benchmark_group(concat!(stringify!($uid), "-vector-addition-reference"));
                
                    {
                        let vector_space = Space::new();
                
                        group.bench_function("stdvec", |c| {
                            let x: Vector<isize> = random();
                            let y: Vector<isize> = random();
                
                            c.iter(|| {
                                vector_space.vadd(&x, &y)
                            })
                        });
                    }
                
                    if $array_length > 0 {
                        let vector_space = ArraySpace::new();
                
                        group.bench_function("array", |c| {
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
                    let mut group: _ = bench.benchmark_group(concat!(stringify!($uid), "-vector-addition-mutable"));
                
                    {
                        let vector_space = Space::new();
                
                        group.bench_function("stdvec", |c| {
                            let mut x: Vector<isize> = random();
                            let y: Vector<isize> = random();
                
                            c.iter(|| {
                                vector_space.vadd_mut(&mut x, &y);
                            })
                        });
                    }
                
                    if $array_length > 0 {
                        let vector_space = ArraySpace::new();
                
                        group.bench_function("array", |c| {
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
                    let mut group: _ = bench.benchmark_group(concat!(stringify!($uid), "-vector-multiplication-reference"));
                
                    {
                        let vector_space = Space::new();
                
                        group.bench_function("stdvec", |c| {
                            let x: Vector<isize> = random();
                            let y: isize = 125;
                
                            c.iter(|| {
                                vector_space.vscale(&x, &y)
                            })
                        });
                    }
                
                    if $array_length > 0 {
                        let vector_space = ArraySpace::new();
                
                        group.bench_function("array", |c| {
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
                    let mut group: _ = bench.benchmark_group(concat!(stringify!($uid), "-vector-multiplication-mutable"));
                
                    {
                        let vector_space = Space::new();
                
                        group.bench_function("stdvec", |c| {
                            let mut x: Vector<isize> = random();
                            let y: isize = 125;
                
                            c.iter(|| {
                                vector_space.vscale_mut(&mut x, &y);
                            })
                        });
                    }
                
                    if $array_length > 0 {
                        let vector_space = ArraySpace::new();
                
                        group.bench_function("array", |c| {
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
                    let mut group: _ = bench.benchmark_group(concat!(stringify!($uid), "-nalgebra-vector-addition-mutable"));
                
                    // Qlogic: 3.7130 us for 10 000
                    {

                        let vector_space = Space::new();
                
                        group.bench_function("std-vec", |c| {
                            let mut x: Vector<isize> = random();
                            let y: Vector<isize> = random();
                
                            c.iter(|| {
                                vector_space.vadd_mut(&mut x, &y);
                            })
                        });
                    }
                
                    if $array_length > 0 {
                        let vector_space = ArraySpace::new();
                
                        group.bench_function("array", |c| {
                            let mut x: ArrayVector<isize> = random_array();
                            let y: ArrayVector<isize> = random_array();
                
                            c.iter(|| {
                                vector_space.vadd_mut(&mut x, &y);
                            })
                        });
                    }
                
                    // Nalgebra: 4.7408 us for 10 000
                    {
                        group.bench_function("nalgebra", |c| {
                            let mut x: nalgebra::DVector<isize> = nalgebra::DVector::new_random(LENGTH);
                            let y: nalgebra::DVector<isize> = nalgebra::DVector::new_random(LENGTH);
                    
                            c.iter(|| {
                                x += &y
                            })
                        });
                    }
                }
                
                fn bench_addition_against_nalgebra(bench: &mut Criterion)
                {
                    let mut group: _ = bench.benchmark_group(concat!(stringify!($uid), "-nalgebra-vector-addition-reference"));
                
                    // Qlogic: 3.7130 us for 10 000
                    {
                        let vector_space = Space::new();
                
                        group.bench_function("std-vec", |c| {
                            let x: Vector<isize> = random();
                            let y: Vector<isize> = random();
                
                            c.iter(|| {
                                vector_space.vadd(&x, &y)
                            })
                        });
                    }
                
                    if $array_length > 0 {
                        let vector_space = ArraySpace::new();
                
                        group.bench_function("array", |c| {
                            let x: ArrayVector<isize> = random_array();
                            let y: ArrayVector<isize> = random_array();
                
                            c.iter(|| {
                                vector_space.vadd(&x, &y)
                            })
                        });
                    }
                
                    // Nalgebra: 4.7408 us for 10 000
                    {
                        group.bench_function("nalgebra", |c| {
                            let x: nalgebra::DVector<isize> = nalgebra::DVector::new_random(LENGTH);
                            let y: nalgebra::DVector<isize> = nalgebra::DVector::new_random(LENGTH);
                    
                            c.iter(|| {
                                &x + &y
                            })
                        });
                    }
                }
                
                fn bench_multiplication_mut_against_nalgebra(bench: &mut Criterion)
                {
                    let mut group: _ = bench.benchmark_group(concat!(stringify!($uid), "-nalgebra-vector-multiplication-mutable"));
                    let scalar: isize = 125;
                
                    // Qlogic: 3.7130 us for 10 000
                    {
                        let vector_space = Space::new();
                
                        group.bench_function("stdvec", |c| {
                            let mut x: Vector<isize> = random();
                
                            c.iter(|| {
                                vector_space.vscale_mut(&mut x, &scalar)
                            })
                        });
                    }
                
                    if $array_length > 0 {
                        let vector_space = ArraySpace::new();
                
                        group.bench_function("array", |c| {
                            let mut x: ArrayVector<isize> = random_array();
                
                            c.iter(|| {
                                vector_space.vscale_mut(&mut x, &scalar)
                            })
                        });
                    }
                
                    // Nalgebra: 4.7408 us for 10 000
                    {
                        group.bench_function("nalgebra", |c| {
                            let mut x: nalgebra::DVector<isize> = nalgebra::DVector::new_random(LENGTH);
                    
                            c.iter(|| {
                                x *= scalar
                            })
                        });
                    }
                }
                
                fn bench_multiplication_against_nalgebra(bench: &mut Criterion)
                {
                    let mut group: _ = bench.benchmark_group(concat!(stringify!($uid), "-nalgebra-vector-multiplication-reference"));
                    let scalar: isize = 125;
                
                    // Qlogic: 3.7130 us for 10 000
                    {
                        let vector_space = Space::new();
                
                        group.bench_function("stdvec", |c| {
                            let x: Vector<isize> = random();
                
                            c.iter(|| {
                                vector_space.vscale(&x, &scalar)
                            })
                        });
                    }
                
                    if $array_length > 0 {
                        let vector_space = ArraySpace::new();
                
                        group.bench_function("array", |c| {
                            let x: ArrayVector<isize> = random_array();
                
                            c.iter(|| {
                                vector_space.vscale(&x, &scalar)
                            })
                        });
                    }
                
                    // Nalgebra: 4.7408 us for 10 000
                    {
                        group.bench_function("nalgebra", |c| {
                            let x: nalgebra::DVector<isize> = nalgebra::DVector::new_random(LENGTH);
                            c.iter(|| {
                                &x * scalar
                            })
                        });
                    }
                }
                
                fn bench_additive_inverse_against_nalgebra(bench: &mut Criterion)
                {
                    let mut group: _ = bench.benchmark_group(concat!(stringify!($uid), "-nalgebra-vector-additive-inverse-reference"));
                
                    // Qlogic: 3.7130 us for 10 000
                    {
                        let vector_space = Space::new();
                
                        group.bench_function("stdvec", |c| {
                            let x: Vector<isize> = random();
                
                            c.iter(|| {
                                vector_space.additive_inv(&x)
                            })
                        });
                    }
                
                    if $array_length > 0 {
                        let vector_space = ArraySpace::new();
                
                        group.bench_function("array", |c| {
                            let x: ArrayVector<isize> = random_array();
                
                            c.iter(|| {
                                vector_space.additive_inv(&x)
                            })
                        });
                    }
                
                    // Nalgebra: 4.7408 us for 10 000
                    {
                        group.bench_function("nalgebra", |c| {
                            let ref x: nalgebra::DVector<isize> = nalgebra::DVector::new_random(LENGTH);
                    
                            c.iter(|| {
                                -x
                            })
                        });
                    }
                }
                
                fn bench_additive_inverse_mut_against_nalgebra(bench: &mut Criterion)
                {
                    let mut group: _ = bench.benchmark_group(concat!(stringify!($uid), "-nalgebra-vector-additive-inverse-mutable"));
                
                    // Qlogic: 3.7130 us for 10 000
                    {
                        let vector_space = Space::new();
                
                        group.bench_function("stdvec", |c| {
                            let mut x: Vector<isize> = random();
                
                            c.iter(|| {
                                vector_space.additive_inv_mut(&mut x)
                            })
                        });
                    }
                
                    if $array_length > 0 {
                        let vector_space = ArraySpace::new();
                
                        group.bench_function("array", |c| {
                            let mut x: ArrayVector<isize> = random_array();
                
                            c.iter(|| {
                                vector_space.additive_inv_mut(&mut x)
                            })
                        });
                    }
                
                    // Nalgebra: 4.7408 us for 10 000
                    {
                        group.bench_function("nalgebra", |c| {
                            let mut x: nalgebra::DVector<isize> = nalgebra::DVector::new_random(LENGTH);
                    
                            c.iter(|| {
                                x = - x.clone()
                            })
                        });
                    }
                }
                
                criterion_group!(
                    $uid,
                    bench_addition_mutable,
                    bench_addition_reference,
                    bench_multiplication_mutable,
                    bench_multiplication_reference,
                    
                    bench_addition_against_nalgebra,
                    bench_addition_mut_against_nalgebra,
                    bench_multiplication_against_nalgebra,
                    bench_multiplication_mut_against_nalgebra,
                    bench_additive_inverse_against_nalgebra,
                    bench_additive_inverse_mut_against_nalgebra
                );   
            }
        )+
    }
}

bench_vectors!{
    (smallvec, 32, 32);
    (medvec, 512, 512);
    (bigvec, 1000000, 0);
}
