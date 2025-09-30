//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Benchmark for TopDownDPStPer
use apas_ai::Chap18::ArraySeqStPer::ArraySeqStPer::*;
use apas_ai::Chap51::TopDownDPStPer::TopDownDPStPer::*;
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use std::time::Duration;

fn bench_med(c: &mut Criterion) {
    let mut group = c.benchmark_group("TopDownDPStPer_MED");
    group.warm_up_time(Duration::from_millis(500));
    group.measurement_time(Duration::from_secs(6));
    group.sample_size(30);
    let s = ArraySeqStPerS::tabulate(&|i| ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j'][i], 10);
    let t = ArraySeqStPerS::tabulate(&|i| ['x', 'b', 'y', 'd', 'z', 'f', 'w', 'h', 'v', 'j'][i], 10);
    group.bench_function(BenchmarkId::new("med_memoized", "small"), |b| {
        b.iter(|| {
            let dp = TopDownDPStPerS::new(s.clone(), t.clone());
            dp.med_memoized()
        });
    });
    group.finish();
}
criterion_group!(benches, bench_med);
criterion_main!(benches);
