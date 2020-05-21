
#[macro_use]
macro_rules! benchmark
{
    (
       $(($uid:ident, $vec_length:expr, $array_length:expr)),+
    ) => {     
        $(        
            pub use $uid::$uid;

            mod $uid
            {    
                use criterion::{criterion_group, Criterion};
                use vector::ndarray;             
                use algebra::vector::*;   

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


                benchmark!{
                    Vector<isize>, ArrayVector<isize>
                    {
                        Global: 
                        {
                            ArrayLength: $array_length,
                            Module: $uid
                        },
                        
                        Benches: 
                        {
                            bench_multiplication_against_nalgebra
                            {
                                GroupIdentifier: "-nalgebra-vector-multiplication-reference",
                                TargetAlpha: {
                                    let vector_space = Space::new();
                                    let x: Vector<isize> = random();
                                    let scalar: isize = 125;

                                    move |c| {
                                        c.iter(|| {
                                            vector_space.vscale(&x, &scalar)
                                        })
                                    }
                                },
                                TargetBeta: {
                                    let vector_space = ArraySpace::new();
                                    let x: ArrayVector<isize> = random_array();
                                    let scalar: isize = 125;

                                    move |c| {
                                        c.iter(|| {
                                            vector_space.vscale(&x, &scalar)
                                        })
                                    }
                                },
                                Nalgebra: {
                                    let x: nalgebra::DVector<isize> = nalgebra::DVector::new_random(LENGTH);
                                    let scalar: isize = 125;

                                    move |c| {
                                        c.iter(|| {
                                            &x * scalar
                                        })
                                    }
                                }
                            },

                            bench_addition_mut_against_nalgebra
                            {
                                GroupIdentifier: "-nalgebra-vector-addition-mutable",
                                TargetAlpha: {
                                    let vector_space = Space::new();
                                    let mut x: Vector<isize> = random();
                                    let y: Vector<isize> = random();    

                                    move |c| {
                                        c.iter(|| {
                                            vector_space.vadd_mut(&mut x, &y);
                                        })
                                    }
                                },
                                TargetBeta: {
                                    let vector_space = ArraySpace::new();
                                    let mut x: ArrayVector<isize> = random_array();
                                    let y: ArrayVector<isize> = random_array();    

                                    move |c| {
                                        c.iter(|| {
                                            vector_space.vadd_mut(&mut x, &y);
                                        })
                                    }
                                },
                                Nalgebra: {
                                    let mut x: nalgebra::DVector<isize> = nalgebra::DVector::new_random(LENGTH);
                                    let y: nalgebra::DVector<isize> = nalgebra::DVector::new_random(LENGTH);
                            
                                    move |c| {
                                        c.iter(|| {
                                            x += &y
                                        })
                                    }
                                }
                            },

                            bench_addition_against_nalgebra
                            {
                                GroupIdentifier: "-nalgebra-vector-addition-reference",
                                TargetAlpha: {
                                    let vector_space = Space::new();
                                    let x: Vector<isize> = random();
                                    let y: Vector<isize> = random();
                        
                                    move |c| {
                                        c.iter(|| {
                                            vector_space.vadd(&x, &y)
                                        })
                                    }
                                },
                                TargetBeta: {
                                    let vector_space = ArraySpace::new();
                                    let x: ArrayVector<isize> = random_array();
                                    let y: ArrayVector<isize> = random_array();    

                                    move |c| {
                                        c.iter(|| {
                                            vector_space.vadd(&x, &y)
                                        })
                                    }
                                },
                                Nalgebra: {
                                    let x: nalgebra::DVector<isize> = nalgebra::DVector::new_random(LENGTH);
                                    let y: nalgebra::DVector<isize> = nalgebra::DVector::new_random(LENGTH);
                            
                                    move |c| {
                                        c.iter(|| {
                                            &x + &y
                                        })
                                    }
                                }
                            },

                            bench_multiplication_mut_against_nalgebra
                            {
                                GroupIdentifier: "-nalgebra-vector-multiplication-mutable",
                                TargetAlpha: {
                                    let vector_space = Space::new();
                                    let mut x: Vector<isize> = random();
                                    let scalar: isize = 125;
                        
                                    move |c| {
                                        c.iter(|| {
                                            vector_space.vscale_mut(&mut x, &scalar)
                                        })
                                    }
                                },
                                TargetBeta: {
                                    let vector_space = ArraySpace::new();
                                    let mut x: ArrayVector<isize> = random_array();
                                    let scalar: isize = 125;

                                    move |c| {
                                        c.iter(|| {
                                            vector_space.vscale_mut(&mut x, &scalar)
                                        })
                                    }
                                },
                                Nalgebra: {
                                    let mut x: nalgebra::DVector<isize> = nalgebra::DVector::new_random(LENGTH);
                                    let scalar: isize = 125;
                            
                                    move |c| {
                                        c.iter(|| {
                                            x *= scalar
                                        })
                                    }
                                }
                            },

                            bench_additive_inverse_mut_against_nalgebra
                            {
                                GroupIdentifier: "-nalgebra-vector-additive-inverse-mutable",
                                TargetAlpha: {
                                    let vector_space = Space::new();
                                    let mut x: Vector<isize> = random();
                        
                                    move |c| {
                                        c.iter(|| {
                                            vector_space.additive_inv_mut(&mut x)
                                        })
                                    }
                                },
                                TargetBeta: {
                                    let vector_space = ArraySpace::new();
                                    let mut x: ArrayVector<isize> = random_array();

                                    move |c| {
                                        c.iter(|| {
                                            vector_space.additive_inv_mut(&mut x)
                                        })
                                    }
                                },
                                Nalgebra: {
                                    let mut x: nalgebra::DVector<isize> = nalgebra::DVector::new_random(LENGTH);
                            
                                    move |c| {
                                        c.iter(|| {
                                            x = - x.clone()
                                        })
                                    }
                                }
                            }
                        }
                    }
                }
                
                criterion_group!(
                    $uid,
                    bench_addition_against_nalgebra,
                    bench_addition_mut_against_nalgebra,
                    bench_multiplication_against_nalgebra,
                    bench_multiplication_mut_against_nalgebra,
                    bench_additive_inverse_mut_against_nalgebra
                );   
            }
        )+
    };

    (
        $space:ty, $array_space:ty
        {
            Global: {
                ArrayLength: $array_length:expr,
                Module: $uid:ident
            },
            Benches: {
                $(
                    $function_name:ident {    
                        GroupIdentifier: $group_name:expr,
                        TargetAlpha: $target_alpha:block,
                        TargetBeta: $target_beta:block,
                        Nalgebra: $nalgebra:block
                    }
                ),*
            }
        }
    ) => {
        $(
            fn $function_name(bench: &mut Criterion)
            {
                let mut group: _ = bench.benchmark_group(
                    concat!( stringify!($uid), $group_name )
                );
            
                // Qlogic: 3.7130 us for 10 000
                {
                    group.bench_function("stdvec", $target_alpha);
                }
            
                if $array_length > 0 {
                    group.bench_function("array", $target_beta);
                }
            
                // Nalgebra: 4.7408 us for 10 000
                {
                    group.bench_function("nalgebra", $nalgebra);
                }
            }
        )*
    };
}

benchmark!{
    (nalgebra_smallvec, 32, 32),
    (nalgebra_medvec, 1024, 1024),
    (nalgebra_bigvec, 1000000, 0)
}
