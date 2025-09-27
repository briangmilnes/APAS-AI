//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
use std::time::Duration;

use apas_ai::Chap36::QuickSortSt::Chapter36St::Chapter36StTrait;
use apas_ai::{ArraySeqStEphSLit, Chap18::ArraySeqStEph::ArraySeqStEph::ArraySeqStEphS};
use criterion::{BatchSize, BenchmarkId, Criterion, criterion_group, criterion_main};

fn gen_data(n: usize) -> ArraySeqStEphS<i32> {
    let mut seed = 0xDEADBEEF12345678u64;
    let mut arr = ArraySeqStEphSLit![0; n]; // *Eph struct: constructor + set pattern
    for i in 0..n {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        let _ = arr.set(i, (seed >> 32) as i32);
    }
    arr
}

fn bench_quicksort_st(c: &mut Criterion) {
    let mut group = c.benchmark_group("Chapter36St");
    group.sample_size(20);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_secs(1));

    for &n in &[128usize, 2_048, 16_384] {
        group.bench_with_input(BenchmarkId::new("first", n), &n, |b, &len| {
            b.iter_batched(
                || gen_data(len),
                |mut seq| {
                    seq.quick_sort_st_first();
                    seq
                },
                BatchSize::SmallInput,
            );
        });

        group.bench_with_input(BenchmarkId::new("median3", n), &n, |b, &len| {
            b.iter_batched(
                || gen_data(len),
                |mut seq| {
                    seq.quick_sort_st_median3();
                    seq
                },
                BatchSize::SmallInput,
            );
        });

        group.bench_with_input(BenchmarkId::new("random", n), &n, |b, &len| {
            b.iter_batched(
                || gen_data(len),
                |mut seq| {
                    seq.quick_sort_st_random();
                    seq
                },
                BatchSize::SmallInput,
            );
        });
    }

    group.finish();
}

criterion_group!(benches, bench_quicksort_st);
criterion_main!(benches);
