
#[macro_use]
macro_rules! benchmark
{
    (
       $(
           ($uid:ident, $vec_length:expr, $array_length:expr)
        ),+
    ) => {     
        $(        
            pub use $uid::$uid;

            mod $uid
            {    
                use criterion::{criterion_group, Criterion};
                use vector::vspace;             
                use algebra::*;   

                use ndarray::prelude::*;

                use rand::{thread_rng, Rng};

                const LENGTH: usize = $vec_length;

                vspace! {
                    Space {
                        vector: Vector,
                        dimension: $vec_length
                    }
                }

                vspace! {
                    ArraySpace {
                        vector: ArrayVector,
                        dimension: $array_length,
                        using: [T; $array_length]
                    }
                }


                fn random_array() -> ArrayVector<isize> {
                    let mut inner: _ = [0; $array_length];
                    for element in inner.iter_mut() {
                        *element = thread_rng().gen()
                    }
                    ArrayVector::new(inner)
                }

                fn random() -> Vector<isize> {
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
                            bench_multiplication_against_ndarray
                            {
                                GroupIdentifier: "-ndarray-vector-multiplication-reference",
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
                                NdArray: {
                                    let mut x: _ = array!([10; LENGTH]);
                                    for val in x.iter_mut() {
                                        *val = thread_rng().gen();
                                    }

                                    let scalar: _ = 125;

                                    move |c| {
                                        c.iter(|| {
                                            scalar * &x
                                        })
                                    }
                                }
                            }
                        }
                    }
                }
                
                criterion_group!(
                    $uid,
                    // bench_addition_against_ndarray,
                    // bench_addition_mut_against_ndarray,
                    bench_multiplication_against_ndarray,
                    // bench_multiplication_mut_against_ndarray,
                    // bench_additive_inverse_mut_against_ndarray,
                    // bench_vaxpy_mut_against_ndarray,
                    // bench_dotv_against_ndarray
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
                        NdArray: $ndarray:block
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
            
                // NdArray: 4.7408 us for 10 000
                {
                    group.bench_function("ndarray", $ndarray);
                }
            }
        )*
    };
}

benchmark!{
    (ndarray_smallvec, 3, 3),
    (ndarray_medvec, 12, 12),
    (ndarray_bigvec, 16, 16)
}
