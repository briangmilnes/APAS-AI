//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Benchmark for TopDownDPMtEph
use std::time::Duration;

use criterion::*;

use apas_ai::Chap19::ArraySeqMtEph::ArraySeqMtEph::*;
use apas_ai::Chap51::TopDownDPMtEph::TopDownDPMtEph::*;

fn bench_med(c: &mut Criterion) {
    let mut group = c.benchmark_group("TopDownDPMtEph_MED");
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));
    group.sample_size(30);
    let s = ArraySeqMtEphS::from_vec(vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j']);
    let t = ArraySeqMtEphS::from_vec(vec!['x', 'b', 'y', 'd', 'z', 'f', 'w', 'h', 'v', 'j']);
    group.bench_function(BenchmarkId::new("med_concurrent", "small"), |b| {
        b.iter(|| {
            let mut dp = TopDownDPMtEphS::new(s.clone(), t.clone());
            dp.med_memoized_concurrent()
        });
    });
    group.finish();
}
criterion_group!(benches, bench_med);
criterion_main!(benches);
