//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Benchmarks for OptBinSearchTreeStPer

use apas_ai::{
    Chap50::{OptBinSearchTreeStPer::OptBinSearchTreeStPer::*, Probability::Probability::Probability},
    prob,
};
use criterion::*;
use std::time::Duration;

fn bench_obst_st_per_small(c: &mut Criterion) {
    let mut group = c.benchmark_group("OBST_StPer_Small");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    group.warm_up_time(std::time::Duration::from_millis(500));
    group.measurement_time(std::time::Duration::from_secs(3));
    group.sample_size(20);

    for size in [5, 10, 15].iter() {
        let keys: Vec<char> = (0..*size).map(|i| (b'A' + i as u8) as char).collect();
        let probs: Vec<Probability> = (0..*size).map(|i| prob!(0.1 + (i as f64) * 0.05)).collect();
        let obst = OBSTStPerS::from_keys_probs(keys, probs);

        group.bench_with_input(BenchmarkId::new("optimal_cost", size), size, |b, _| {
            b.iter(|| black_box(obst.optimal_cost()))
        });
    }
    group.finish();
}

criterion_group!(benches, bench_obst_st_per_small);
criterion_main!(benches);
