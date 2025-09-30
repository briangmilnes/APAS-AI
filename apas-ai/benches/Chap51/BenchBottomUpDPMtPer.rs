//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Benchmark for BottomUpDPMtPer
use apas_ai::Chap18::ArraySeqMtPer::ArraySeqMtPer::*;
use apas_ai::Chap51::BottomUpDPMtPer::BottomUpDPMtPer::*;
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use std::time::Duration;

fn bench_med(c: &mut Criterion) {
    let mut group = c.benchmark_group("BottomUpDPMtPer_MED");
    group.warm_up_time(Duration::from_millis(500));
    group.measurement_time(Duration::from_secs(6));
    group.sample_size(30);
    let s = ArraySeqMtPerS::from_vec(vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j']);
    let t = ArraySeqMtPerS::from_vec(vec!['x', 'b', 'y', 'd', 'z', 'f', 'w', 'h', 'v', 'j']);
    group.bench_function(BenchmarkId::new("med_parallel", "small"), |b| {
        b.iter(|| {
            let dp = BottomUpDPMtPerS::new(s.clone(), t.clone());
            dp.med_bottom_up_parallel()
        });
    });
    group.finish();
}
criterion_group!(benches, bench_med);
criterion_main!(benches);
