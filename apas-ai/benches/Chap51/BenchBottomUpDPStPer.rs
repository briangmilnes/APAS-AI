//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Benchmark for BottomUpDPStPer - Bottom-Up DP Single-Threaded Persistent

use std::time::Duration;

use criterion::*;

use apas_ai::Chap18::ArraySeqStPer::ArraySeqStPer::*;
use apas_ai::Chap51::BottomUpDPStPer::BottomUpDPStPer::*;

fn bench_med_bottom_up(c: &mut Criterion) {
    let mut group = c.benchmark_group("BottomUpDPStPer_MED");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    group.sample_size(30);

    // Small: 10 chars
    let s_small = ArraySeqStPerS::tabulate(&|i| ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j'][i], 10);
    let t_small = ArraySeqStPerS::tabulate(&|i| ['x', 'b', 'y', 'd', 'z', 'f', 'w', 'h', 'v', 'j'][i], 10);
    group.bench_function(BenchmarkId::new("med", "small_10"), |b| {
        b.iter(|| {
            let dp = BottomUpDPStPerS::new(s_small.clone(), t_small.clone());
            dp.med_bottom_up()
        });
    });

    // Medium: 30 chars
    let s_med_data = ['a'; 30];
    let t_med_data = ['b'; 30];
    let s_med = ArraySeqStPerS::tabulate(&|i| s_med_data[i], 30);
    let t_med = ArraySeqStPerS::tabulate(&|i| t_med_data[i], 30);
    group.bench_function(BenchmarkId::new("med", "medium_30"), |b| {
        b.iter(|| {
            let dp = BottomUpDPStPerS::new(s_med.clone(), t_med.clone());
            dp.med_bottom_up()
        });
    });

    group.finish();
}

criterion_group!(benches, bench_med_bottom_up);
criterion_main!(benches);
