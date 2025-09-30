//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
use apas_ai::Chap18::ArraySeqMtPer::ArraySeqMtPer::*;
use apas_ai::ArraySeqMtPerChap18::ArraySeqMtPerChap18::*;
use apas_ai::Types::Types::*;
use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use std::time::Duration;

// Helper functions for benchmarks
fn identity(i: N) -> N { i }
fn add_one(x: &N) -> N { x + 1 }

fn bench_tabulate_map_mtper_ch18(c: &mut Criterion) {
    let mut group = c.benchmark_group("BenchArraySeqMtPerChap18");
    group.sample_size(10);
    group.warm_up_time(Duration::from_secs(1));
    group.measurement_time(Duration::from_secs(1));
    let n: N = 10_000;
    group.bench_with_input(BenchmarkId::new("tabulate_then_map", n), &n, |b, &len| {
        b.iter(|| {
            let s: ArraySeqMtPerS<N> = <ArraySeqMtPerS<N> as ArraySeqMtPerChap18Trait<N>>::tabulate(identity, len);
            let m: ArraySeqMtPerS<N> = <ArraySeqMtPerS<N> as ArraySeqMtPerChap18Trait<N>>::map(&s, add_one);
            black_box((s.length(), m.length()))
        })
    });
    group.finish();
}

fn bench_reduce_parallel_mtper_ch18(c: &mut Criterion) {
    let mut group = c.benchmark_group("BenchArraySeqMtPerReduce");
    group.sample_size(30);
    group.warm_up_time(Duration::from_millis(800));
    group.measurement_time(Duration::from_secs(6));
    
    for &n in &[1_000, 5_000, 10_000] {
        group.bench_with_input(BenchmarkId::new("reduce_sum", n), &n, |b, &len| {
            let s: ArraySeqMtPerS<N> = <ArraySeqMtPerS<N> as ArraySeqMtPerChap18Trait<N>>::tabulate(identity, len);
            b.iter(|| {
                let sum = <ArraySeqMtPerS<N> as ArraySeqMtPerChap18Trait<N>>::reduce(&s, &|x, y| x + y, 0);
                black_box(sum)
            })
        });
    }
    group.finish();
}

criterion_group!(benches, bench_tabulate_map_mtper_ch18, bench_reduce_parallel_mtper_ch18);
criterion_main!(benches);
