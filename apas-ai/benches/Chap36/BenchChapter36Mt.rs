//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
use std::time::Duration;

use criterion::*;

use apas_ai::Chap19::ArraySeqMtEph::ArraySeqMtEph::*;
use apas_ai::Chap36::QuickSortMtEph::Chapter36Mt::*;
use apas_ai::ArraySeqMtEphSLit;
use apas_ai::*;

fn gen_data(n: usize) -> apas_ai::Chap19::ArraySeqMtEph::ArraySeqMtEph::ArraySeqMtEphS<i32> {
    let mut seed = 0x1234_5678_9ABC_DEF0u64;
    let arr = ArraySeqMtEphSLit![0; n]; // *Eph struct: constructor + set pattern
    for i in 0..n {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        let _ = arr.set(i, (seed >> 32) as i32);
    }
    arr
}

fn bench_quicksort_mt(c: &mut Criterion) {
    let mut group = c.benchmark_group("Chapter36Mt");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));

    for &n in &[256usize, 2_048, 5_000] {
        group.bench_with_input(BenchmarkId::new("first", n), &n, |b, &len| {
            b.iter_batched(
                || gen_data(len),
                |mut seq| {
                    seq.quick_sort_mt_first();
                    seq
                },
                BatchSize::SmallInput,
            );
        });

        group.bench_with_input(BenchmarkId::new("median3", n), &n, |b, &len| {
            b.iter_batched(
                || gen_data(len),
                |mut seq| {
                    seq.quick_sort_mt_median3();
                    seq
                },
                BatchSize::SmallInput,
            );
        });

        group.bench_with_input(BenchmarkId::new("random", n), &n, |b, &len| {
            b.iter_batched(
                || gen_data(len),
                |mut seq| {
                    seq.quick_sort_mt_random();
                    seq
                },
                BatchSize::SmallInput,
            );
        });
    }

    group.finish();
}

criterion_group!(benches, bench_quicksort_mt);
criterion_main!(benches);
