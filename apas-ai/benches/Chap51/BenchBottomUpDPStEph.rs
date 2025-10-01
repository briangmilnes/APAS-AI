//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Benchmark for BottomUpDPStEph
use apas_ai::Chap18::ArraySeqStEph::ArraySeqStEph::*;
use apas_ai::Chap51::BottomUpDPStEph::BottomUpDPStEph::*;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::time::Duration;

fn bench_med(c: &mut Criterion) {
    let mut group = c.benchmark_group("BottomUpDPStEph_MED");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    group.sample_size(30);
    let s = ArraySeqStEphS::tabulate(&|i| ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j'][i], 10);
    let t = ArraySeqStEphS::tabulate(&|i| ['x', 'b', 'y', 'd', 'z', 'f', 'w', 'h', 'v', 'j'][i], 10);
    group.bench_function(BenchmarkId::new("med", "small"), |b| {
        b.iter(|| {
            let mut dp = BottomUpDPStEphS::new(s.clone(), t.clone());
            dp.med_bottom_up()
        });
    });
    group.finish();
}
criterion_group!(benches, bench_med);
criterion_main!(benches);
