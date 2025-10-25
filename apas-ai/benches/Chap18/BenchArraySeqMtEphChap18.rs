//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
use std::time::Duration;

use criterion::*;

use apas_ai::Chap18::ArraySeqMtEph::ArraySeqMtEph::ArraySeqMtEphBaseTrait;
use apas_ai::Chap18::ArraySeqMtEph::ArraySeqMtEph::ArraySeqMtEphRedefinableTrait;
use apas_ai::Chap18::ArraySeqMtEph::ArraySeqMtEph::*;
use apas_ai::Types::Types::*;

fn identity(i: N) -> N { i }

fn increment(x: &N) -> N { x + 1 }

fn bench_tabulate_map_mteph_ch18(c: &mut Criterion) {
    let mut group = c.benchmark_group("BenchArraySeqMtEphChap18");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));

    // Test different sizes (all use unconditional parallelism with ParaPair!)
    for &n in &[1_000, 10_000] {
        group.bench_with_input(BenchmarkId::new("tabulate_then_map", n), &n, |b, &len| {
            b.iter(|| {
                let s = ArraySeqMtEphS::<N>::tabulate(&identity, len);
                let m = ArraySeqMtEphS::<N>::map(&s, &increment);
                black_box((s.length(), m.length()))
            })
        });
    }
    group.finish();
}

fn one_plus(i: N) -> N { i + 1 }

fn add(x: &N, y: &N) -> N { x + y }

fn bench_reduce_parallel_mteph_ch18(c: &mut Criterion) {
    let mut group = c.benchmark_group("BenchArraySeqMtEphReduce");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));

    // Test different sizes (all use unconditional parallelism with ParaPair!)
    for &n in &[1_000, 10_000] {
        group.bench_with_input(BenchmarkId::new("reduce_sum", n), &n, |b, &len| {
            let s = ArraySeqMtEphS::tabulate(&one_plus, len);
            b.iter(|| {
                let sum = ArraySeqMtEphS::reduce(&s, add, 0);
                black_box(sum)
            })
        });
    }
    group.finish();
}

fn is_even(x: &N) -> bool { x % 2 == 0 }

fn bench_filter_mteph_ch18(c: &mut Criterion) {
    let mut group = c.benchmark_group("BenchArraySeqMtEphFilter");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));

    let n: N = 10_000;
    group.bench_with_input(BenchmarkId::new("filter_evens", n), &n, |b, &len| {
        let s = ArraySeqMtEphS::tabulate(&identity, len);
        b.iter(|| {
            let evens = ArraySeqMtEphS::filter(&s, &is_even);
            black_box(evens.length())
        })
    });
    group.finish();
}

fn bench_scan_mteph_ch18(c: &mut Criterion) {
    let mut group = c.benchmark_group("BenchArraySeqMtEphScan");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));

    let n: N = 5_000;
    group.bench_with_input(BenchmarkId::new("scan_sum", n), &n, |b, &len| {
        let s = ArraySeqMtEphS::tabulate(&one_plus, len);
        b.iter(|| {
            let (prefixes, final_sum) = ArraySeqMtEphS::scan(&s, &add, 0);
            black_box((prefixes.length(), final_sum))
        })
    });
    group.finish();
}

criterion_group!(
    benches,
    bench_tabulate_map_mteph_ch18,
    bench_reduce_parallel_mteph_ch18,
    bench_filter_mteph_ch18,
    bench_scan_mteph_ch18
);
criterion_main!(benches);
